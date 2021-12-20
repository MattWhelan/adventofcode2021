use anyhow::Result;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
struct Image {
    grid: Vec<Vec<bool>>,
    default_value: bool,
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.grid.iter().for_each(|row| {
            row.iter().for_each(|b| {
                if *b {
                    write!(f, "#").unwrap();
                } else {
                    write!(f, ".").unwrap();
                }
            });
            write!(f, "\n").unwrap();
        });
        write!(f, "\n",)
    }
}

impl Image {
    const OFFSETS: [(isize, isize); 9] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    fn at(&self, pos: (isize, isize)) -> bool {
        if pos.0 < 0
            || pos.1 < 0
            || pos.1 as usize >= self.grid.len()
            || pos.0 as usize >= self.grid[pos.1 as usize].len()
        {
            self.default_value
        } else {
            self.grid[pos.1 as usize][pos.0 as usize]
        }
    }

    fn convolved(&self, pos: (isize, isize)) -> usize {
        number(
            &mut Image::OFFSETS
                .iter()
                .map(|off| self.at((off.0 + pos.0, off.1 + pos.1))),
            Image::OFFSETS.len(),
        )
    }

    fn count_lit(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|b| **b).count())
            .sum()
    }

    fn enhance(&self, table: &[bool]) -> Image {
        let target_max_y: isize = self.grid.len() as isize + 1;
        let target_max_x: isize = self.grid[0].len() as isize + 1;

        let new_default_value = if self.default_value {
            table[0x1ff]
        } else {
            table[0]
        };

        Image {
            grid: (-1..target_max_y)
                .map(|y| {
                    (-1..target_max_x)
                        .map(|x| {
                            let index = self.convolved((x, y));
                            table[index]
                        })
                        .collect()
                })
                .collect(),
            default_value: new_default_value,
        }
    }
}

fn number(it: &mut dyn Iterator<Item = bool>, n: usize) -> usize {
    it.zip(0..n)
        .map(|(b, i)| if b { 1 << (n - 1 - i) } else { 0 })
        .fold(0, |acc, n| acc | n)
}

fn main() -> Result<()> {
    let input_lines: Vec<&str> = INPUT.lines().collect();
    let input_parts: Vec<&[&str]> = input_lines.split(|l| l.is_empty()).collect();

    let table: Vec<bool> = input_parts[0][0]
        .chars()
        .map(|ch| if ch == '#' { true } else { false })
        .collect();

    let input_image_bits: Vec<Vec<bool>> = input_parts[1]
        .iter()
        .map(|l| {
            l.chars()
                .map(|ch| if ch == '#' { true } else { false })
                .collect()
        })
        .collect();

    let image_raw = Image {
        grid: input_image_bits,
        default_value: false,
    };

    part_1(&table, &image_raw);
    part_2(&table, &image_raw);

    Ok(())
}

fn part_1(table: &Vec<bool>, image_raw: &Image) {
    let image_first_pass = image_raw.enhance(&table);
    let image_second_pass = image_first_pass.enhance(&table);

    // println!("{}", &image_first_pass);
    // println!("{}", &image_second_pass);

    //not 5294
    println!("Part 1 {}", image_second_pass.count_lit());
}

fn part_2(table: &Vec<bool>, image_raw: &Image) {
    let mut image = image_raw.clone();
    for _ in 0..50 {
        image = image.enhance(table);
    }
    println!("Part 2 {}", image.count_lit());
}

const INPUT: &str = r#"#.#.#.#.#......#.#.#.#.##..#.##.##..#..##...#.#.#.#...##.##.##.###....#..#...#.#..###.#...#..##.#.###..#..####.###...#.#.#..##..##.##..##..###..#....#.#....#####.#...###...#.#....###...#..##.##..#..#.##..###..#.##.###..#.####...#.##.....#.###...#.##.##.#.#######...#.###..##..##..#.#.#.#####...#....#.....##.#.#...##.######....#..#......#.#.#.#.##...######.#.#####..#####..#.#.#.#.###.#.#....#..##..#..#.#.#..##....##..#.#.......##...#..####.####.#.#..#.###..#...#......###...#...#.##.#.####..#.#....###.####..#.

.#.#.########.##.#...##.####...##..#####...##..#.#.###..#...#.#.##.#.....#..#.###...##..#.###.###.##
.######.#..##.##...#.#.####.#.###..#..#.##.##...##.#...##..#.######...##....###...###..#.#.##.##....
..######.#.#.##.#.##.###.##..#####.####..#......#.##.###.#.#.##...#####.###..###...#..#..#..##.#....
###..##.#...##.#.#...######..#.#..##..##.#.##.#.#.#.##.#..####.#.#.##......#.#.#...#.##..#.###.###.#
#.#.#...#..####...#.#..#.##.####.#..#..###########...#....#.#.##.###..#####.#.#.#.###...##..####.#..
#.#..#.##.#..#..#..#....##.#.#.#...#..###.#.##.##.#.#.##.##..#.#.####.#######....###..#.######.#.#..
......###..#..##.####.##.##..#..#.##.##.#..#...###...####.#..#...###.##.#.#####.....#...#..#...####.
####..##.###.#...##..#.##....#..##.##..####.#.#.####.##..#..#.....#.###......#.#....#.#.....#.#..#..
.###.####.#..###..#.#.#.#...##.####.#..#..##....#...###.#.#....#..#######.###.....#.#.#.##...##.#..#
##.#......#.##.###.#.##..##.##..#######.###..##..#.#####..#.#..#.#.#..#..##..##.##...#####.#.##.####
.#.###....##.......#######..#.########..#..##..#.####.###..#..###.##..#...####.#.#..#.##.######.#..#
..###.#.##..##.#....#..#.####.....#.#..#..##.##.###..#.###.....##..#.##..#..#.#...#...#.#########.##
..#.#.#.#..#...#.##.#.##.#..#...##.#..........#.###.##.##....#####...#.#####.###.#......#..#.#.#..##
#....#..#.#.#.####.#####.#.#.####.###......#.....#.#..######....##....#.##..##.##...#.#####.##..##.#
#.#....#####....###.###.#.#...#.##.#..........#....##.#..##..##.####.##.#.##....##..#.#.##..##.#.###
..####...####..##.#.....#........#.#..##..#.#..###.....####...#...#.###....#....##.#..##.#.##....##.
...#.###........####...#...##..#..##.#.######...#...#.#.#...###....##..##.#..##.......###.##.###...#
##.#####...#.#.##.##.#...#.....#.##.########.....#.##..#.####.##...#......####..#.#..#..#...#.......
.....###..##.###.#.#.#.....##.#.####...#..######.##....#.##.#.#.#...##.####.####.##....##.#.#.###..#
.########..#.#.#.##..#..#..#.#..#..#.#.##.###.###.#...#.#..#####..##.###.#.##..###.###.#.#.####...##
###..##.##...#...#.....####.#.#......##.####..#......##..#####.#.....#.###.##.....#.##...#...#...###
#.#.#.....##.##..#..#.###.#..##..##..#....#...##.....##..##.####.#######.....#........###..#.##.#.##
.#....##..###.#..###.#...###.#...###...#.#..####..######......##.##.##..#...#..#####.####...##..#...
####.#..#.##....#.#.#...#.#.#####.###.#..##.#.###.....#..#.##......#.##...##..##...##..####.#...##..
....#.####..#..##.####.#######...#..#########.##.##..#..#.##.##.#.###.##.#.#....#####.###...#.####.#
..#.....##...###..#.##...#.#...###...#######.##..####.##.##..##.#########..###........##....#..#..#.
.#.##.#.#.##.....##.#.##..###.###.#.....###...###..#.#.#####.##.#....##.#.##.##.##.#.#....#........#
.#.#..#.#............#....###.#...####..####....#..#.##..#.##..#####...###.######...##..#####..###..
###.#..##.#...##..#.#..#..##.#..#..#..#..#####......##.##..#####..#......#..#####.##.####......##.##
##...#...##.#####..#...#.#.....#.#..###.#..#.####.....#.....#.#.#.#.###.####..#.......##.#....#..##.
#...#.#..#####..###.........#.####.###..#....###...#......#...#.#..#.....#####..#.###...#.#...#.....
###....#..##.##.####.###.#.#.##.#.#....##..#....##..#.#.###..######.#.#.#..####.##...####..##..##.##
.###...##.#.#####.#...#.###..##..###....#.##....#...#.#..#...##.###...#.#.#.#######......###.###.#.#
.#.#.#.....#..##..#.##.#..#.....#.#.######..#.##.#....###.#....#...#####..#.######.###....###...####
####..#....########...#..##..#####.#...#.##..##..###...###.#####....##.#.#..#.###.#.#..#.#.#..#.###.
.##.##.##.#####...#..#..#.###.#.#..###.#......###.##.#.###..#####.#.#...#.###.#.#.###.##.#...#.##.#.
#.#.##....#####...##.#.##.######.#.#.##.###.#.......###......##..###.#.###...##..##.#.#...#..##.####
##.#....#.##.#..#...#.#...#.##......####.#.#..#..###.#####.#...##.####...#..###.##.#.##..###.####.#.
#.#..#.#.##...##.....###.#.#.#..##.###.#######........###..#.....####.#.##.##...####.##.#.##.###.###
##...#.####..#...#...##.#.##..#.##.###..#.##..##......#..#.#...######..#....#.######..######..####..
.....#.#.......####.....#..#.####.#...##..##...#....#.########.#.#.##..##.##..####..##.####..#.#.###
.#..#.#....#..##..#....##...###........#.#..#.##.####....###..#..##...#..##.#.#.###.#.#.#..#.####...
#.#...###.#....#...##############...###..##...#.#.##..#..#....#...####....##..###.##..##..##.#..##.#
##..#....#...#..#..##.#..###....#.#.#.###.#.#.###..###..####.###.#.#.###.#..###.#.###.#.##...#.#####
#..#..####.....###..#.#...##.######..####..#..#....#####...#...#..#..#.....#.##.####......#..##.###.
##.##..###.###.#..###...#.####...#....#........#..#..##..##..##.#.##....#.......#.###...#..###.#....
#.##.#.#..#..###..###...#...#.###..####.#.#....#.#.......##...#..#......##....##.#####.......#...#..
....#..#..##.##.#.##.#.##..#.##..##.##.##.##..##.######.##..##.....#.###..#...#.##.#.#.####.###.###.
.###....##.##..##..#.###..##.#..#.#.##..##.###..##.###..#......########.#####....##...#...##...##.##
.###.###.#.#######.......#.#.#.##.##..#.#.#.##..#.##...##.##.#...##.#.#######.#..#..##..#.#..##.##.#
....#.###.#.##...##..#...#...#..##...#.##...##.##.##.#.#####.....#.#.#.##..####.####.#..####..#..##.
##..#.##.#######.#.#...###.#....###.###..##.###.##.##.#.#..##.....#.##......##.###..##.#.#.##..##...
...#####.#.##.#..##....##.###.##.#..##..###......#.#..####.##...##.#.###...##.....#..#..###..#####..
.#####.#.#..#....####.#####.#..##..#.#####.##..#.#.#..##...#.#..######.###..##....###.#....###...#.#
..###......##.##.#.##..#..##.....##....##.##..##.....###.#..##.#.#######..#...####..#.###.#####.###.
##..#.######.####..#.#..###.#..#.#..#######.#...#...##.#.###.#.##..##.#......##.#.#..##.#.#.#.####..
..#.#.####.##..##.#.#.#..####.#.##...###..#.##..#.##.###..####......#..#..#####.#..#.#####.#.###.###
#..#.#.....#.#...#..###.####..##..#....##.#.###..##..#.#..#.####...##..##..#..####...#########....##
.#.....#......##.#..#.####.######..#.#.#.#.##...#..#..#...#.###..#....#..#.#...#####..###.##...#.##.
.####....#.###..#..###.#...#..###.#.#..#......###.##.#.#.#.#####..#######..####.##.....#....#..##.#.
...#.##.#.#..####.#####..##..#.##.###.#####..###...##...##.##..#.####..##.#...####...#..##......###.
###...##.....#.###...##.#..#.#......##..#...##.#..##.#.#..#.###..#..#####..#.###..#.#...#..##.#.#.#.
##.##.##.#.#.#.####.##..##....#.####.####.####...##.####.#....###....###.###.#..#.####.#...####.#.#.
####.###...####.#.##...#...##..##...##....#.#.####..#..#.....#.....##....###.....###...#...##.#.###.
######..#..#####...#..#..######.##...#.###.#....####..####.##.#.#..#.#.####.##.####..##...####.##.#.
##....#.#...#..###..#.##.#.#...###..####.....#.##.#.####..#.##....##...##########..##...###.#.###.##
..#.#######....#####..##.####.##.#####..###.#.#..####.....###########...##.#....#.#..##.##.###..###.
##.....##...#..#....#####...#...#..#.#.#.#.####....##...####.#.#.#.....#.#..........#.###..##.#.#.#.
.#####...#..#.##..##.#...#.#.##..###..#.#....#...#.#.#..#.#..#..#.###.#...######.#.####..##..##.#...
..#.#.#.##..#.##..#.###......###..#....##.###.#..###.#...##.#.#.....#.##....##.##.##.#...####.####.#
##..#.##.##.##.#....##.#..#..#.##.###..##.##.#.#.#......####..##.#.#.###.....##.....#.##..####..##.#
.###.#.##.#..##.##.#.###.#.##.##.#####...#.#..#..#..#..#.####..#.######..#.#.#...#..#####...#.#..##.
..#...#.####.####.####...#..#..##..#.##.#.#..#..####..#...#.####.#.###.##..#......#..#...#..#..###..
.#..#.###.......#..##.#.#..##..#.#..#..##..#.....####.###...#..#.###.##.#..#.#..##..#...##.##.##....
#....##..#...##.###.#......##..###...##.###..##..###.####.....#...###..#.#...#####.#.#.######..#..#.
##.###.....#...#.#..##.#.#..#.#....#...#..##.######.###.#.####.#.######...#.#.#....##.##..#...#...#.
.......#..#.##..#..##...#.##.#####..###.####.###.#.#..###....#.#........#..#.#.#.......####..#......
#####...##.######....#.#.##.#..##...#..#..#..#...#..##.###....#.#.......##...#.###.###..####.##.###.
.###.##.#....#.#..#..#..##....##...##.#...##....#......#####...####.######.#.#.##.##..##.#.#.....#..
..#.##..#.####..####.##.#...#..####..##.....###.#...###..#.###.######.#.#.##..#.#.#####.#.##.#.##...
.##.###########.#..####.#.####..#.####.##########..#.##.#.#.#..#.#..#.#####.....##.#...#.###..#.##..
...###.#.#..#.#.#.#....#..##..####.##...###.#....#.#.####.#.#..#..####...####..###.##..######.##....
#..#####.#######.#.###......#..#.#.##.##.###.#.##.#...#....#.#####..###..##.#.#####.#..#..#.###.#..#
#.####..#..#.......###..#..####.#.#.#..#.#..#..#.#...#####.#.......####.#.#.##.##.#..#..##.#....##.#
###.###...#..#..##.#..#.#..#..#####.#.#####.....#..#..####....##.##.....#.....#.###.#.#.#.##.#.####.
.#.##.#.#..#.#####.#.##.#.###.#...#..#..#.#...##.#.##......#.#.#####..#......###.########.#..#.###.#
#.##.....#.####..######..##..#...#.....##..##..#.#.##.###.#.#.#.#.....#...#.#..###..##..##.######.##
####......#.##..#.##..##.##..#.##.....#####...#.#...###.#..##..###.##.###..............##.#.#....#.#
###...##.##.##.####....####..#...##...#.####.#....#.....#####.....##...##..###.#...##.#...##.##.##..
#.#.####...###.....###.###.##.###.....##.##.....###..###..##......##..####.#..##.#.##....##.#.#..#.#
####...###.#####.#####.......##..##.###..#..##.##..#..###.#..#..###.....##..##..##.##.#..####.#.#.#.
######....#.##.##.#.###...##...###.###.#.##.###....##.#......#.##.##.#####...#..##....###...#..##...
..#...........##..........#.##.#.##.##.###...##..##.#####.#####.#...##..####.....#.##....#..#.###.##
.#..##.#...#.#####...##.#.#.###.#.#..##..#..##########..#.#...#...#...##.##.#...##..#.#.#.##.#..#..#
..##....##.#..#...#.#...#.#....#..#.#.#####.##.#......#...##..######..##....####..##.##.##.###.###..
#.......#..############...##....##...#.#....#...##..###..##....####..##....#...##.###...#..#...#####
..##..##..#.###.#.##.#.####.#.##...#.#..#.#####...######...######.####...###.#.#.##..#.######..#.#..
###.#....#####..#.##.##..###..##.#..#.#.#.##.#......#..#....##.#.#.###..#.#######.###..#..#..#.###..
#...#.#..#.####.###..##.#.#####...#.#.##..#..#.#####.#.###.##.####..#..##.#.##...##...####.#..#.###.
...#.##.##...#.#...####....#..#####...#.#.....####.##.######.#.#...#...###.##..##.#.#.#..#...###.#..
"#;