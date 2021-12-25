use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::iter::once;
use std::ops::Index;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Color {
    A,
    B,
    C,
    D,
}

impl Color {
    pub fn step_cost(&self) -> u32 {
        match self {
            Color::A => 1,
            Color::B => 10,
            Color::C => 100,
            Color::D => 1000,
        }
    }
}

impl From<char> for Color {
    fn from(ch: char) -> Self {
        match ch {
            'A' => Color::A,
            'B' => Color::B,
            'C' => Color::C,
            'D' => Color::D,
            ch => panic!("unsupported color {}", ch),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Color::A => 'a',
            Color::B => 'b',
            Color::C => 'c',
            Color::D => 'd',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Hash, Eq, PartialEq)]
pub struct Pawn {
    pub color: Color,
    start: (usize, usize),
}

impl Pawn {
    pub fn new(color: Color, pos: (usize, usize)) -> Pawn {
        Pawn { color, start: pos }
    }
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Hash, Copy, Clone)]
pub enum Tile {
    WALL,
    HOME(Color),
    HALL,
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Tile::WALL,
            '.' => Tile::HALL,
            ch if ch.is_uppercase() => Tile::HOME(ch.into()),
            ' ' => Tile::WALL,
            _ => panic!("unexpected char {}", ch),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Tile::WALL => '#',
            Tile::HOME(_) => '_',
            Tile::HALL => '.',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Maze {
    grid: Vec<Vec<Tile>>,
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|l| l.chars().map(|ch| ch.into()).collect())
            .collect();
        Ok(Maze { grid })
    }
}

impl Maze {
    pub fn pawns<'a>(&'a self) -> impl Iterator<Item = Pawn> + 'a {
        self.grid.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, t)| {
                if let Tile::HOME(c) = t {
                    Some(Pawn::new(*c, (x, y)))
                } else {
                    None
                }
            })
        })
    }

    pub fn neighbors<'a>(
        &'a self,
        (x, y): &(usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        let x_range = 0..self.grid[0].len() as isize;
        let y_range = 0..self.grid.len() as isize;
        let x = *x as isize;
        let y = *y as isize;
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(move |&(p, q)| {
                ((p + q) as i32).abs() == 1
                    && x_range.contains(&(x + p))
                    && y_range.contains(&(y + q))
            })
            .map(move |(p, q)| ((x + p) as usize, (y + q) as usize))
            .filter(|pos| self.is_open(pos))
    }

    fn is_open(&self, pos: &(usize, usize)) -> bool {
        match self[pos] {
            Tile::WALL => false,
            Tile::HOME(_) => true,
            Tile::HALL => true,
        }
    }

    pub fn blocks_doorway(&self, pos: &(usize, usize)) -> bool {
        match self[pos] {
            Tile::WALL => false,
            Tile::HOME(_) => false,
            Tile::HALL => self.neighbors(pos).count() > 2,
        }
    }

    fn homes<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, t)| ((x, y), t)))
            .filter(|(_, t)| matches!(t, Tile::HOME(_)))
            .map(|(p, _)| p)
    }
}

impl Index<&(usize, usize)> for Maze {
    type Output = Tile;

    fn index(&self, index: &(usize, usize)) -> &Self::Output {
        &self.grid[index.1][index.0]
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct VisitState<Node>
where
    Node: Eq + PartialEq + Clone,
{
    dist: u32,
    n: Node,
}

impl<Node> Ord for VisitState<Node>
where
    Node: Ord + Eq + PartialEq + Clone,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.n.cmp(&other.n))
    }
}

// `PartialOrd` needs to be implemented as well.
impl<Node> PartialOrd for VisitState<Node>
where
    Node: Ord + Eq + PartialEq + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct World {
    maze: Rc<Maze>,
    pawns: BTreeMap<(usize, usize), Pawn>,
}

impl World {
    pub fn new(maze: Maze, pawns: Vec<Pawn>) -> World {
        World {
            maze: Rc::new(maze),
            pawns: pawns.into_iter().map(|p| (p.start, p)).collect(),
        }
    }

    fn is_vacant(&self, pos: &(usize, usize)) -> bool {
        !self.pawns.contains_key(pos)
    }

    fn pawn_at(&self, pos: &(usize, usize)) -> Option<&Pawn> {
        self.pawns.get(pos)
    }

    fn pawn_matches_room(&self, pos: &(usize, usize)) -> bool {
        if let Some(pawn) = self.pawn_at(pos) {
            if let Tile::HOME(home_color) = self.maze[pos] {
                pawn.color == home_color
            } else {
                false
            }
        } else {
            false
        }
    }

    fn room<'a>(&'a self, pos: &'a (usize, usize)) -> impl Iterator<Item = (usize, usize)> + 'a {
        assert!(matches!(self.tile(pos), Tile::HOME(_)));
        let y_bounds = 0..self.maze.grid.len();

        (3..8)
            .map(|y| (pos.0, y))
            .filter(move |n| y_bounds.contains(&n.1))
            .filter(|n| matches!(self.tile(n), Tile::HOME(_)))
            .chain(once(*pos))
    }

    pub fn neighbor_count(&self, pos: &(usize, usize)) -> usize {
        self.maze.neighbors(pos).count()
    }

    pub fn is_ready_home(&self, pos: &(usize, usize)) -> bool {
        if matches!(self.tile(pos), Tile::HOME(_)) && self.is_vacant(pos) {
            self.room(pos)
                .all(|p| self.is_vacant(&p) || self.pawn_matches_room(&p))
        } else {
            false
        }
    }

    pub fn is_pawn_in_hall(&self, pos: &(usize, usize)) -> bool {
        if let Tile::HALL = self.maze[pos] {
            self.pawns.contains_key(pos)
        } else {
            false
        }
    }

    pub fn do_move(&mut self, from: &(usize, usize), to: &(usize, usize)) {
        assert!(self.is_vacant(to));
        assert!(self.maze.is_open(to));
        if let Some(p) = self.pawns.remove(from) {
            if !self.pawns.contains_key(to) {
                self.pawns.insert(to.clone(), p);
            } else {
                panic!("move to occupied space at to {:?}", to)
            }
        } else {
            panic!("no pawn at {:?}", from);
        }
    }

    pub fn path_dist(&self, start: &(usize, usize)) -> HashMap<(usize, usize), u32> {
        dijkstra(start, |pos| {
            self.maze
                .neighbors(pos)
                .filter(|n| self.is_vacant(n))
                .map(|n| (n, 1))
        })
    }

    pub fn is_settled(&self) -> bool {
        self.pawns
            .iter()
            .all(|(pos, _)| self.pawn_matches_room(pos))
    }

    pub fn pawns(&self) -> &BTreeMap<(usize, usize), Pawn> {
        &self.pawns
    }

    pub fn find_ready_homes<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.maze.homes().filter(|h| self.is_ready_home(h))
    }

    pub fn tile(&self, pos: &(usize, usize)) -> Tile {
        self.maze[pos]
    }

    pub fn is_pawn_settled_at(&self, pos: &(usize, usize)) -> bool {
        self.pawn_matches_room(pos)
            && self
                .room(pos)
                .all(|place| self.is_vacant(&place) || self.pawn_matches_room(&place))
    }

    pub fn blocks_doorway(&self, pos: &(usize, usize)) -> bool {
        self.maze.blocks_doorway(pos)
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.maze.grid.len() {
            for x in 0..self.maze.grid[y].len() {
                if let Some(pawn) = self.pawn_at(&(x, y)) {
                    let mut s = pawn.color.to_string();
                    if self.is_pawn_settled_at(&(x, y)) {
                        s = s.to_uppercase();
                    }
                    write!(f, "{}", s).unwrap();
                } else {
                    let tile = self.tile(&(x, y));
                    write!(f, "{}", tile).unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

pub fn dijkstra<Node, It: Iterator<Item = (Node, u32)>, NF: Fn(&Node) -> It>(
    start: &Node,
    neighbors: NF,
) -> HashMap<Node, u32>
where
    Node: Ord + Eq + PartialEq + Clone + Hash,
{
    let mut distance = HashMap::new();
    let mut visited = HashSet::new();
    // let mut prev = HashMap::new();

    distance.insert(start.clone(), 0);

    let mut heap = BinaryHeap::new();
    heap.push(VisitState {
        dist: 0,
        n: start.clone(),
    });

    while let Some(VisitState { dist, n: current }) = heap.pop() {
        let current_cost = distance[&current];
        if dist > current_cost || visited.contains(&current) {
            continue;
        }

        if current_cost == u32::MAX {
            // Disconnected; some obstructions exist
            break;
        }
        for (n, cost) in neighbors(&current).filter(|(n, _cost)| !visited.contains(n)) {
            let d = distance.entry(n.clone()).or_insert(u32::MAX);
            if *d > current_cost + cost {
                *d = current_cost + cost;
                // prev.insert(n, current);
                heap.push(VisitState { dist: *d, n })
            }
        }
        visited.insert(current);
    }

    distance
}
