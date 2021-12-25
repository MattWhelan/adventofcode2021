use std::collections::HashMap;

use anyhow::Result;

use day23::*;

fn main() -> Result<()> {
    {
        let maze = TARGET_1.parse::<Maze>().unwrap();
        let pawns: Vec<Pawn> = INPUT_1.parse::<Maze>().unwrap().pawns().collect();

        let world = World::new(maze, pawns);
        println!("\nPart 1: {}", solve(world).expect("Failed to solve"));
    }

    {
        let maze = TARGET_2.parse::<Maze>().unwrap();
        let pawns: Vec<Pawn> = INPUT_2.parse::<Maze>().unwrap().pawns().collect();

        let world = World::new(maze, pawns);
        println!("\nPart 2: {}", solve(world).expect("Failed to solve"));
    }

    Ok(())
}

fn solve(world: World) -> Option<u32> {
    let distances = dijkstra(&world, |n| neighbors(n));
    distances
        .iter()
        .filter(|(w, _)| w.is_settled())
        .min_by_key(|(_, d)| *d)
        .map(|(w, cost)| {
            println!("World\n{}", w);
            *cost
        })
}

fn neighbors(world: &World) -> impl Iterator<Item = (World, u32)> {
    //println!("{}\n", &world);
    //if there are move-home moves, do those
    let ready_homes: HashMap<Color, (usize, usize)> = world
        .find_ready_homes()
        .map(|h| {
            if let Tile::HOME(color) = world.tile(&h) {
                (color, h)
            } else {
                unreachable!()
            }
        })
        .fold(HashMap::new(), |mut acc, (color, h)| {
            // pick the furthest-in spot in the room
            if let Some(pos) = acc.get(&color) {
                if pos.1 < h.1 {
                    acc.insert(color, h);
                }
            } else {
                acc.insert(color, h);
            }
            acc
        });
    let move_homes: Vec<_> = if ready_homes.is_empty() {
        Vec::new()
    } else {
        world
            .pawns()
            .iter()
            .filter_map(move |(pawn_pos, pawn)| {
                if let Tile::HALL = world.tile(pawn_pos) {
                    if let Some(ready_home_pos) = ready_homes.get(&pawn.color) {
                        let dist = world.path_dist(pawn_pos);
                        if let Some(distance) = dist.get(ready_home_pos) {
                            let cost = distance * pawn.color.step_cost();
                            Some((*pawn_pos, *ready_home_pos, cost))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    };

    if !move_homes.is_empty() {
        move_homes
            .iter()
            .map(move |(from, to, dist)| {
                let mut w = world.clone();
                w.do_move(from, to);
                (w, *dist)
            })
            .collect::<Vec<_>>()
            .into_iter()
    } else {
        //no move-homes, do a move-out

        //find unsettled pawns
        let move_outs: Vec<_> = world
            .pawns()
            .iter()
            .filter(|(pawn_pos, _)| {
                if let Tile::HOME(_) = world.tile(pawn_pos) {
                    !world.is_pawn_settled_at(pawn_pos)
                } else {
                    false
                }
            })
            .flat_map(|(pawn_pos, pawn)| {
                let dist = world.path_dist(pawn_pos);
                dist.into_iter()
                    .filter(|(pos, _)| {
                        matches!(world.tile(pos), Tile::HALL) && !world.blocks_doorway(pos)
                    })
                    .map(|(dest, distance)| {
                        let cost = distance * pawn.color.step_cost();
                        (*pawn_pos, dest, cost)
                    })
            })
            .collect();
        move_outs
            .iter()
            .map(move |(from, to, dist)| {
                let mut w = world.clone();
                w.do_move(&from, &to);
                (w, *dist)
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

const INPUT_1: &str = r#"#############
#...........#
###C#A#B#D###
  #B#A#D#C#
  #########"#;

const TARGET_1: &str = r#"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########"#;

const INPUT_2: &str = r#"#############
#...........#
###C#A#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #B#A#D#C#
  #########"#;

const TARGET_2: &str = r#"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########"#;
