use anyhow::Result;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use itertools::Itertools;

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<u32>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "\n")
    }
}

impl Index<(usize, usize)> for Board {
    type Output = u32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.1][index.0]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.1][index.0]
    }
}

impl Board {
    fn around(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let x_range = 0..self.grid[0].len() as isize;
        let y_range = 0..self.grid.len() as isize;
        let x = x as isize;
        let y = y as isize;
        (-1..=1).cartesian_product(-1..=1)
            .filter(move |&(p, q)| (p,q) != (0, 0)
                && x_range.contains(&(x + p))
                && y_range.contains(&(y + q)))
            .map(move |(p, q)| ((x + p) as usize, (y + q) as usize))
    }

    fn sum(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().map(|x| *x as usize).sum::<usize>())
            .sum()
    }

    fn increase(&mut self) {
        self.grid
            .iter_mut()
            .for_each(|row| row.iter_mut()
                .for_each(|cell| *cell += 1))
    }

    fn flash(&mut self, pos: (usize, usize), flashed: &mut HashSet<(usize, usize)>) {
        flashed.insert(pos);

        for neighbor in self.around(pos) {
            self[neighbor] += 1;
            if self[neighbor] > 9 && !flashed.contains(&neighbor) {
                self.flash(neighbor, flashed);
            }
        }
    }

    fn step(&mut self) -> usize {
        self.increase();

        let width = self.grid[0].len();
        let mut flashed = HashSet::new();
        for y in 0..self.grid.len() {
            for x in 0..width {
                if self[(x, y)] > 9 && !flashed.contains(&(x, y)) {
                    self.flash((x, y), &mut flashed);
                }
            }
        }

        let count = flashed.len();
        for pos in flashed {
            self[pos] = 0;
        }
        count
    }
}

fn main() -> Result<()> {
    let input: Vec<Vec<u32>> = INPUT
        .lines()
        .map(|l| l.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let mut count = 0;
    let mut board = Board {
        grid: input.clone(),
    };

    for _ in 0..100 {
        count += board.step();
    }

    println!("Part 1: {}", count);

    let mut board = Board {
        grid: input.clone(),
    };
    let step_no = (1..)
        .filter(|_| {
            board.step();
            board.sum() == 0
        })
        .next()
        .unwrap();

    println!("Part 2: {}", step_no);

    Ok(())
}

const INPUT: &str = r#"3322874652
5636588857
7755117548
5854121833
2856682477
3124873812
1541372254
8634383236
2424323348
2265635842
"#;
