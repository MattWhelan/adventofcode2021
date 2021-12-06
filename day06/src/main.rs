use anyhow::Result;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Record;

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Record)
    }
}

fn main() -> Result<()> {
    let input: Vec<u32> = INPUT.split(",").map(|l| l.parse().unwrap()).collect();

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(fish: &[u32]) {
    let mut fish = Vec::from(fish);
    for _ in 0..80 {
        let mut new_fish = Vec::new();
        for f in fish.iter_mut() {
            if *f == 0 {
                *f = 6;
                new_fish.push(8);
            } else {
                *f -= 1;
            }
        }
        fish.append(&mut new_fish);
    }

    println!("Part 1: {}", fish.len());
}

fn part2(fish: &[u32]) {
    let mut ages: HashMap<u32, u64> = fish.iter().fold(HashMap::new(), |mut acc, a| {
        *(acc.entry(*a).or_insert(0)) += 1;
        acc
    });

    let mut eights = 0;
    let mut sevens = 0;

    for i in 0..256 {
        let today = i % 7;
        let today_count = ages.entry(today).or_insert(0);
        let born = today_count.clone();
        *today_count += sevens;
        sevens = eights;
        eights = born;
    }

    println!("Part 2: {}", eights + sevens + ages.values().sum::<u64>());
}

const INPUT: &str = r#"3,5,3,1,4,4,5,5,2,1,4,3,5,1,3,5,3,2,4,3,5,3,1,1,2,1,4,5,3,1,4,5,4,3,3,4,3,1,1,2,2,4,1,1,4,3,4,4,2,4,3,1,5,1,2,3,2,4,4,1,1,1,3,3,5,1,4,5,5,2,5,3,3,1,1,2,3,3,3,1,4,1,5,1,5,3,3,1,5,3,4,3,1,4,1,1,1,2,1,2,3,2,2,4,3,5,5,4,5,3,1,4,4,2,4,4,5,1,5,3,3,5,5,4,4,1,3,2,3,1,2,4,5,3,3,5,4,1,1,5,2,5,1,5,5,4,1,1,1,1,5,3,3,4,4,2,2,1,5,1,1,1,4,4,2,2,2,2,2,5,5,2,4,4,4,1,2,5,4,5,2,5,4,3,1,1,5,4,5,3,2,3,4,1,4,1,1,3,5,1,2,5,1,1,1,5,1,1,4,2,3,4,1,3,3,2,3,1,1,4,4,3,2,1,2,1,4,2,5,4,2,5,3,2,3,3,4,1,3,5,5,1,3,4,5,1,1,3,1,2,1,1,1,1,5,1,1,2,1,4,5,2,1,5,4,2,2,5,5,1,5,1,2,1,5,2,4,3,2,3,1,1,1,2,3,1,4,3,1,2,3,2,1,3,3,2,1,2,5,2"#;
