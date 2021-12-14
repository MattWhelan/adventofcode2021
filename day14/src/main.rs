use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::once;
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
    let input: Vec<&str> = INPUT.lines().collect();

    let template: Vec<char> = input[0].chars().collect();
    let rules: HashMap<String, String> = input[2..]
        .iter()
        .map(|s| {
            s.split(" -> ")
                .map(|s2| s2.to_string())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    part1(&template, &rules);
    part2(&template, &rules);

    Ok(())
}

fn part1(template: &Vec<char>, rules: &HashMap<String, String>) {
    let mut polymer = template.clone();
    for _ in 0..10 {
        polymer = step(&polymer, &rules).collect();
    }

    let counts = polymer.iter().fold(HashMap::new(), |mut acc, ch| {
        *acc.entry(ch).or_insert(0) += 1;
        acc
    });

    let max_count = counts.values().max().unwrap();
    let min_count = counts.values().min().unwrap();

    println!("Part 1: {}", max_count - min_count);
}

fn part2(template: &Vec<char>, rules: &HashMap<String, String>) {
    let rules: HashMap<(char, char), [(char, char); 2]> = rules
        .iter()
        .map(|(k, v)| {
            let k2: (char, char) = k.chars().collect_tuple().unwrap();
            let v = v.chars().next().unwrap();
            (k2, [(k2.0, v), (v, k2.1)])
        })
        .collect();

    let mut pair_counts = HashMap::new();

    template.windows(2).for_each(|chs| {
        let pair = (chs[0], chs[1]);
        *pair_counts.entry(pair).or_insert(0) += 1 as u64;
    });

    for _ in 0..40 {
        pair_counts = pair_counts
            .iter()
            .flat_map(|(k, v)| rules[k].iter().map(|pair| (*pair, *v)))
            .fold(HashMap::new(), |mut acc, (pair, count)| {
                *acc.entry(pair).or_insert(0) += count;
                acc
            });
    }

    let mut counts = pair_counts
        .iter()
        .map(|(pair, count)| (pair.0, count))
        .fold(HashMap::new(), |mut acc, (ch, count)| {
            *acc.entry(ch).or_insert(0) += count;
            acc
        });

    *counts.entry(template[template.len() - 1]).or_insert(0) += 1;

    let max_count = counts.values().max().unwrap();
    let min_count = counts.values().min().unwrap();

    println!("Part 2: {}", max_count - min_count);
}

fn step<'a>(s: &'a [char], rules: &'a HashMap<String, String>) -> impl Iterator<Item = char> + 'a {
    s.windows(2)
        .flat_map(|chs| {
            let k = chs.iter().join("");
            let insertion = &rules[&k];
            once(chs[0]).chain(insertion.chars())
        })
        .chain(once(s[s.len() - 1]))
}

const INPUT: &str = r#"OFSNKKHCBSNKBKFFCVNB

KC -> F
CO -> S
FH -> K
VP -> P
KF -> S
SV -> O
CB -> H
PN -> F
NC -> N
BC -> F
NP -> O
SK -> F
HS -> C
SN -> V
OP -> F
ON -> N
FK -> N
SH -> B
HN -> N
BO -> V
VK -> H
SC -> K
KP -> O
VO -> V
HC -> P
BK -> B
VH -> N
PV -> O
HB -> H
VS -> F
KK -> B
HH -> B
CF -> F
PH -> C
NS -> V
SO -> P
NV -> K
BP -> N
SF -> V
SS -> K
FP -> N
PC -> S
OH -> B
CH -> H
VV -> S
VN -> O
OB -> K
PF -> H
CS -> C
PP -> O
NF -> H
SP -> P
OS -> V
BB -> P
NO -> F
VB -> V
HK -> C
NK -> O
HP -> B
HV -> V
BF -> V
KO -> F
BV -> H
KV -> B
OF -> V
NB -> F
VF -> C
PB -> B
FF -> H
CP -> C
KH -> H
NH -> P
PS -> P
PK -> P
CC -> K
BS -> V
SB -> K
OO -> B
OK -> F
BH -> B
CV -> F
FN -> V
CN -> P
KB -> B
FO -> H
PO -> S
HO -> H
CK -> B
KN -> C
FS -> K
OC -> P
FV -> N
OV -> K
BN -> H
HF -> V
VC -> S
FB -> S
NN -> P
FC -> B
KS -> N
"#;
