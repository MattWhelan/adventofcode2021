use anyhow::Result;

trait Rng {
    fn roll(&mut self) -> u32;
}

struct Part1Rng {
    next: u32,
    counter: u32
}

impl Rng for Part1Rng {
    fn roll(&mut self) -> u32 {
        self.counter += 1;
        let ret = self.next;
        if ret == 100 {
            self.next = 1;
        } else {
            self.next += 1;
        }
        ret
    }
}

#[derive(Debug)]
struct Game {
    pos1: u32,
    pos2: u32,
    score1: u32,
    score2: u32,
    target: u32,
}

impl Game {
    fn round<R: Rng>(&mut self, rng: &mut R) -> Option<(u32, u32)> {
        let roll1: u32 = (0..3).map(|_| rng.roll()).sum();
        self.pos1 = (self.pos1 + roll1) % 10;
        self.score1 += self.pos1+1;

        if self.score1 >= self.target {
            return Some((self.score1, self.score2))
        }

        let roll2: u32 = (0..3).map(|_| rng.roll()).sum();
        self.pos2 = (self.pos2 + roll2) % 10;
        self.score2 += self.pos2+1;

        if self.score2 >= self.target {
            return Some((self.score2, self.score1))
        }

        None
    }
}

fn main() -> Result<()> {
    let mut rng1 = Part1Rng { next: 1, counter: 0 };
    let mut game = Game {
        pos1: 8-1,
        pos2: 9-1,
        score1: 0,
        score2: 0,
        target: 1000,
    };

    let (_winner, loser) = loop {
        if let Some(scores) = game.round(&mut rng1) {
            break scores
        }
    };

    println!("Part 1 {}", rng1.counter * loser);

    Ok(())
}
