use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Cuboid {
    pub x: RangeInclusive<i32>,
    pub y: RangeInclusive<i32>,
    pub z: RangeInclusive<i32>,
}

impl Cuboid {
    pub fn part1(&self) -> bool {
        let bounds = -50..=50;
        bounds.contains(self.x.start())
            && bounds.contains(self.x.end())
            && bounds.contains(self.y.start())
            && bounds.contains(self.y.end())
            && bounds.contains(self.z.start())
            && bounds.contains(self.z.end())
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (i32, i32, i32)> + 'a {
        self.x.clone().flat_map(move |x| {
            self.y
                .clone()
                .flat_map(move |y| self.z.clone().map(move |z| (x, y, z)))
        })
    }

    fn range_intersection(
        left: &RangeInclusive<i32>,
        right: &RangeInclusive<i32>,
    ) -> Option<RangeInclusive<i32>> {
        if left.contains(right.start()) || left.contains(right.end()) {
            Some(*left.start().max(right.start())..=(*left.end().min(right.end())))
        } else if right.contains(left.start()) || right.contains(left.end()) {
            Some(*right.start().max(left.start())..=(*right.end().min(left.end())))
        } else {
            None
        }
    }

    fn range_sub(
        left: &RangeInclusive<i32>,
        right: &RangeInclusive<i32>,
    ) -> Vec<RangeInclusive<i32>> {
        let mut ret = Vec::new();
        // Left side remainder
        if left.contains(right.start()) && left.start() < right.start() {
            ret.push(RangeInclusive::new(*left.start(), right.start() - 1));
        }
        // Right side remainder
        if left.contains(right.end()) && left.end() > right.end() {
            ret.push(RangeInclusive::new(right.end() + 1, *left.end()))
        }
        ret
    }

    fn intersection(&self, other: &Self) -> Option<Cuboid> {
        let x = Cuboid::range_intersection(&self.x, &other.x);
        let y = Cuboid::range_intersection(&self.y, &other.y);
        let z = Cuboid::range_intersection(&self.z, &other.z);

        x.zip(y).zip(z).map(|((x, y), z)| Cuboid { x, y, z })
    }

    fn sub(&self, other: &Self) -> Vec<Cuboid> {
        if let Some(isect) = Cuboid::intersection(self, other) {
            let x_parts = Cuboid::range_sub(&self.x, &other.x);
            let mut ret: Vec<Cuboid> = x_parts
                .into_iter()
                .map(|x| Cuboid {
                    x,
                    y: self.y.clone(),
                    z: self.z.clone(),
                })
                .collect();
            let y_parts = Cuboid::range_sub(&self.y, &other.y);
            ret.extend(y_parts.into_iter().map(|y| Cuboid {
                x: isect.x.clone(),
                y,
                z: self.z.clone(),
            }));
            let z_parts = Cuboid::range_sub(&self.z, &other.z);
            ret.extend(z_parts.into_iter().map(|z| Cuboid {
                x: isect.x.clone(),
                y: isect.y.clone(),
                z,
            }));

            ret
        } else {
            vec![self.clone()]
        }
    }

    fn volume(&self) -> u64 {
        let mut vol: u64 = (self.x.end() - self.x.start()) as u64 + 1;
        vol *= (self.y.end() - self.y.start()) as u64 + 1;
        vol *= (self.z.end() - self.z.start()) as u64 + 1;
        vol
    }
}

#[derive(Debug)]
pub enum VolSet {
    Simple(Cuboid),
    Union(HashSet<Cuboid>),
}

impl VolSet {
    pub fn union(self, other: Cuboid) -> VolSet {
        match self {
            VolSet::Simple(left) => {
                let mut cubes = left.sub(&other);
                cubes.push(other);
                let ret = VolSet::Union(HashSet::from_iter(cubes.into_iter()));
                ret
            }
            VolSet::Union(left) => {
                let mut cubes: Vec<Cuboid> = left.iter().flat_map(|c| c.sub(&other)).collect();
                cubes.push(other);

                let ret = VolSet::Union(HashSet::from_iter(cubes.into_iter()));
                ret
            }
        }
    }

    pub fn subtract(self, other: Cuboid) -> VolSet {
        match self {
            VolSet::Simple(left) => {
                let mut cubes = left.sub(&other);
                if cubes.len() == 1 {
                    VolSet::Simple(cubes.remove(0))
                } else {
                    VolSet::Union(HashSet::from_iter(cubes.into_iter()))
                }
            }
            VolSet::Union(left) => {
                let cubes = left.iter().flat_map(|c| c.sub(&other)).collect();
                VolSet::Union(cubes)
            }
        }
    }

    pub fn volume(&self) -> u64 {
        match self {
            VolSet::Simple(c) => c.volume(),
            VolSet::Union(cubes) => cubes.iter().map(|c| c.volume()).sum(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_sub_center() {
        let left = Cuboid {
            x: -1..=1,
            y: -1..=1,
            z: -1..=1,
        };

        let right = Cuboid {
            x: 0..=0,
            y: 0..=0,
            z: 0..=0,
        };

        let expected: Vec<Cuboid> = vec![
            Cuboid {
                x: -1..=-1,
                y: -1..=1,
                z: -1..=1,
            },
            Cuboid {
                x: 1..=1,
                y: -1..=1,
                z: -1..=1,
            },
            Cuboid {
                x: 0..=0,
                y: -1..=-1,
                z: -1..=1,
            },
            Cuboid {
                x: 0..=0,
                y: 1..=1,
                z: -1..=1,
            },
            Cuboid {
                x: 0..=0,
                y: 0..=0,
                z: -1..=-1,
            },
            Cuboid {
                x: 0..=0,
                y: 0..=0,
                z: 1..=1,
            },
        ];

        let result = left.sub(&right);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_sub_corner() {
        let left = Cuboid {
            x: -1..=1,
            y: -1..=1,
            z: -1..=1,
        };

        let right = Cuboid {
            x: 1..=2,
            y: 1..=2,
            z: 1..=2,
        };

        let expected: Vec<Cuboid> = vec![
            Cuboid {
                x: -1..=0,
                y: -1..=1,
                z: -1..=1,
            },
            Cuboid {
                x: 1..=1,
                y: -1..=0,
                z: -1..=1,
            },
            Cuboid {
                x: 1..=1,
                y: 1..=1,
                z: -1..=0,
            },
        ];

        let result = left.sub(&right);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_sub_separate() {
        let left = Cuboid {
            x: -1..=1,
            y: -1..=1,
            z: -1..=1,
        };

        let right = Cuboid {
            x: 2..=3,
            y: 2..=3,
            z: 2..=3,
        };

        let expected: Vec<Cuboid> = vec![left.clone()];

        let result = left.sub(&right);
        assert_eq!(expected, result);
    }
}
