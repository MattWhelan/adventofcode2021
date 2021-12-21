use std::collections::HashMap;
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct PlayerState {
    pos: u8,
    score: u8,
}

impl PlayerState {
    fn roll(&self, r: u8) -> PlayerState {
        let pos = (self.pos + r) % 10;
        let score = self.score + pos + 1;
        PlayerState {
            pos, score
        }
    }

    fn is_win(&self) -> bool {
        self.score >= 21
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct GameState {
    players: [PlayerState; 2]
}

struct DiracDiceGame {
    universes: HashMap<GameState, u64>,
}

impl DiracDiceGame {
    const ROLLS: [(u64, u8); 7] = [
        (1, 3),
        (3, 4),
        (6, 5),
        (7, 6),
        (6, 7),
        (3, 8),
        (1, 9),
    ];

    fn new(p1_start: u8, p2_start: u8) -> DiracDiceGame {
        let players = [
            PlayerState {
                pos: p1_start-1,
                score: 0,
            },
            PlayerState {
                pos: p2_start-1,
                score: 0,
            }
        ];

        let mut universes = HashMap::new();
        universes.insert(GameState {players}, 1);

        DiracDiceGame {
            universes
        }
    }

    fn turn(&mut self, player_index: usize) -> u64 {
        let mut win_count = 0;
        let mut new_universes = HashMap::new();
        for (roll_count, r) in DiracDiceGame::ROLLS {
            for (g, state_count) in self.universes.iter() {
                let new_player_state = g.players[player_index].roll(r);
                if new_player_state.is_win() {
                    win_count += state_count * roll_count;
                } else {
                    let watcher_index = (player_index + 1) % 2;
                    let watcher_state = g.players[watcher_index].clone();
                    let mut new_g = g.clone();
                    new_g.players[player_index] = new_player_state;
                    new_g.players[watcher_index] = watcher_state;

                    *new_universes.entry(new_g).or_insert(0) += state_count * roll_count;
                }
            }
        }
        self.universes = new_universes;
        win_count
    }

    fn round(&mut self) -> (u64, u64) {
        let p1_wins = self.turn(0);
        let p2_wins = self.turn(1);
        (p1_wins, p2_wins)
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

    let mut dirac_dice = DiracDiceGame::new(8, 9);
    let result = (0..21).map(|_| dirac_dice.round())
        .fold((0,0), |(p, q), (x, y)| (p + x, q + y));

    let most_wins = result.0.max(result.1);
    println!("Part 2 {}", most_wins);

    Ok(())
}
