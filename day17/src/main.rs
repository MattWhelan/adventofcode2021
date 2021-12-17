use anyhow::Result;
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Probe {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Probe {
    fn new(dx: i32, dy: i32) -> Probe {
        Probe { x: 0, y: 0, dx, dy }
    }

    fn step(&mut self) -> (i32, i32) {
        self.x += self.dx;
        self.y += self.dy;
        if self.dx > 0 {
            self.dx -= 1;
        } else if self.dx < 0 {
            self.dx += 1;
        }
        self.dy -= 1;

        self.pos()
    }

    fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn on_target(
    pos: (i32, i32),
    target_x: &RangeInclusive<i32>,
    target_y: &RangeInclusive<i32>,
) -> bool {
    target_x.contains(&pos.0) && target_y.contains(&pos.1)
}

fn main() -> Result<()> {
    let target_x = 241..=273;
    let target_y = -97..=-63;

    let floor = *target_y.start();
    let far_wall = *target_x.end();

    let mut records = HashMap::new();

    for dx in 1..=*target_x.end() {
        for dy in *target_y.start()..200 {
            let mut p = Probe::new(dx, dy);
            let mut pos = p.pos();
            let v0 = (dx, dy);
            let mut highest = 0;

            while pos.0 <= far_wall && pos.1 >= floor {
                pos = p.step();
                if pos.1 > highest {
                    highest = pos.1;
                }
                if on_target(pos, &target_x, &target_y) {
                    records.insert(v0, highest);
                }
            }
        }
    }

    let best = records.iter().max_by_key(|(_, h)| *h).unwrap();

    println!("Part 1: {} from {:?}", best.1, best.0);

    println!("Part 2: {}", records.len());
    Ok(())
}
