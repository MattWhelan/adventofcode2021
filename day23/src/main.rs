use std::collections::HashMap;
use anyhow::Result;
use day23::*;

fn main() -> Result<()> {
    let maze = TARGET.parse::<Maze>().unwrap();
    let pawns: Vec<Pawn> = INPUT.parse::<Maze>().unwrap().pawns().collect();

    let world = World::new(maze, pawns);
    println!("\nPart 1: {}", part1(world, 0).expect("Failed to solve"));

    Ok(())
}

// TODO: Strip cost tracking from pawn, so world state is just positions
// TODO: Dijkstra on world state as nodes

fn part1(world: World, cost: u32) -> Option<u32> {
    if world.is_settled() {
        Some(cost)
    } else {
        //println!("{}\n", &world);
        //if there are move-home moves, do those
        let ready_homes: HashMap<Color, (usize, usize)> = world.find_ready_homes()
            .map(|h| if let Tile::HOME(color) = world.tile(&h) {
                (color, h)
            } else {
                unreachable!()
            })
            .fold(HashMap::new(), |mut acc, (color, h)| {
                // pick the furthest-in spot in the room
                if let Some(pos) = acc.get(&color) {
                    if world.neighbor_count(pos) > world.neighbor_count(&h) {
                        acc.insert(color, h);
                    }
                } else {
                    acc.insert(color, h);
                }
                acc
            });
        let move_homes: Vec<_> = if ready_homes.is_empty(){
            Vec::new()
        } else {
            world.pawns().iter()
                .filter_map(|(pawn_pos, pawn)| {
                    if let Tile::HALL = world.tile(pawn_pos) {
                        if let Some(ready_home_pos) = ready_homes.get(&pawn.color) {
                            let dist = world.path_dist(pawn_pos);
                            if let Some(distance) = dist.get(ready_home_pos) {
                                Some((*pawn_pos, ready_home_pos, *distance))
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
            move_homes.iter()
                .filter_map(|(from, to, dist)| {
                    let mut w = world.clone();
                    w.do_move(from, to);
                    part1(w, cost + *dist)
                })
                .min()
        } else {
            //no move-homes, do a move-out

            //find unsettled pawns
            let move_outs: Vec<_> = world.pawns().iter()
                .filter(|(pawn_pos, _)| {
                    if let Tile::HOME(_) = world.tile(pawn_pos) {
                        !world.is_pawn_settled_at(pawn_pos)
                    } else {
                        false
                    }
                })
                .flat_map(|(pawn_pos, _)| {
                    let dist = world.path_dist(pawn_pos);
                    dist.into_iter()
                        .filter(|(pos, _)| {
                            matches!(world.tile(pos), Tile::HALL) && !world.blocks_doorway(pos)
                        })
                        .map(|(dest, cost)| (*pawn_pos, dest, cost))
                })
                .collect();
            move_outs.iter()
                .filter_map(|(from, to, dist)| {
                    let mut w = world.clone();
                    w.do_move(&from, &to);
                    part1(w, cost + dist)
                })
                .min()
        }
    }
}


const INPUT: &str = r#"#############
#...........#
###C#A#B#D###
  #B#A#D#C#
  #########"#;

const TARGET: &str = r#"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########"#;
