use grid::Grid;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut map = Grid::new(0, 0);
    input
        .lines()
        .for_each(|l| map.push_row(l.chars().collect()));

    let start_pos = map
        .indexed_iter()
        .find(|(_, &c)| c == 'S')
        .map(|(pos, _)| pos)
        .unwrap_or_else(|| panic!("Start position 'S' not found"));

    count_tiles_on_best_paths(&map, start_pos)
}

fn count_tiles_on_best_paths(map: &Grid<char>, start: (usize, usize)) -> usize {
    let mut cache = HashMap::new();
    let mut min_score = usize::MAX;
    let mut min_score_positions = HashSet::new();
    let mut stack = vec![Reindeer::new(start)];

    while let Some(reindeer) = stack.pop() {
        if map[reindeer.pos] == 'E' {
            match reindeer.score.cmp(&min_score) {
                std::cmp::Ordering::Less => {
                    min_score = reindeer.score;
                    min_score_positions.clear();
                    min_score_positions.extend(reindeer.path);
                }
                std::cmp::Ordering::Equal => {
                    min_score_positions.extend(reindeer.path);
                }
                _ => {}
            }
            continue;
        }

        if let Some(&cached_score) = cache.get(&reindeer.pos) {
            if cached_score < reindeer.score {
                continue;
            }
        }

        let next_postions = reindeer.next_positions();
        if map[next_postions.straight] != '#' {
            cache.insert(reindeer.pos, reindeer.score + 1);
            stack.push(reindeer.clone().move_straight());
        } else {
            cache.insert(reindeer.pos, reindeer.score + 1001);
        }
        if map[next_postions.left] != '#' {
            stack.push(reindeer.clone().move_left());
        }
        if map[next_postions.right] != '#' {
            stack.push(reindeer.move_right());
        }
    }

    min_score_positions.len()
}

#[derive(Debug, Clone)]
struct Reindeer {
    pos: (usize, usize),
    dir: Direction,
    score: usize,
    path: Vec<(usize, usize)>,
}

struct NextPositions {
    straight: (usize, usize),
    left: (usize, usize),
    right: (usize, usize),
}

impl Reindeer {
    fn new(pos: (usize, usize)) -> Self {
        Self {
            pos,
            dir: Direction::East,
            score: 0,
            path: vec![pos],
        }
    }

    fn next_pos_in_dir(&self, dir: Direction) -> (usize, usize) {
        match dir {
            Direction::North => (self.pos.0 - 1, self.pos.1),
            Direction::East => (self.pos.0, self.pos.1 + 1),
            Direction::South => (self.pos.0 + 1, self.pos.1),
            Direction::West => (self.pos.0, self.pos.1 - 1),
        }
    }

    fn next_positions(&self) -> NextPositions {
        let straight = self.next_pos_in_dir(self.dir);
        let left = self.next_pos_in_dir(self.dir.turn_left());
        let right = self.next_pos_in_dir(self.dir.turn_right());
        NextPositions {
            straight,
            left,
            right,
        }
    }

    fn move_straight(mut self) -> Self {
        self.pos = self.next_pos_in_dir(self.dir);
        self.score += 1;
        self.path.push(self.pos);
        self
    }

    fn move_left(mut self) -> Self {
        self.dir = self.dir.turn_left();
        self.score += 1000;
        self.move_straight()
    }

    fn move_right(mut self) -> Self {
        self.dir = self.dir.turn_right();
        self.score += 1000;
        self.move_straight()
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input1() {
        let input = include_str!("../test_input1.txt").trim();
        assert_eq!(process_input(input), 45);
    }

    #[test]
    fn test_input2() {
        let input = include_str!("../test_input2.txt").trim();
        assert_eq!(process_input(input), 64);
    }
}
