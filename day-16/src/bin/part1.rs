use grid::Grid;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut map = Grid::new(0, 0);
    input
        .lines()
        .for_each(|l| map.push_row(l.chars().collect()));

    let start = map
        .indexed_iter()
        .find(|(_, &c)| c == 'S')
        .map(|(pos, _)| pos)
        .unwrap_or_else(|| panic!("Start position 'S' not found"));

    find_best_path(&map, start)
}

fn find_best_path(map: &Grid<char>, start: (usize, usize)) -> usize {
    let mut cache = HashMap::new();
    let mut min_score = usize::MAX;
    let mut stack = vec![Reindeer::new(start)];

    while let Some(reindeer) = stack.pop() {
        if map[reindeer.pos] == 'E' {
            min_score = min_score.min(reindeer.score);
            continue;
        }

        if let Some(&cached_score) = cache.get(&reindeer.pos) {
            if cached_score <= reindeer.score {
                continue;
            }
        }
        cache.insert(reindeer.pos, reindeer.score);

        let next = [
            reindeer.move_left(),
            reindeer.move_right(),
            reindeer.move_straight(),
        ];

        stack.extend(next.iter().filter(|r| map[r.pos] != '#'));
    }

    min_score
}

#[derive(Debug, Clone, Copy)]
struct Reindeer {
    pos: (usize, usize),
    dir: Direction,
    score: usize,
}

impl Reindeer {
    fn new(pos: (usize, usize)) -> Self {
        Self {
            pos,
            dir: Direction::East,
            score: 0,
        }
    }

    fn move_straight(mut self) -> Self {
        self.pos = match self.dir {
            Direction::North => (self.pos.0 - 1, self.pos.1),
            Direction::East => (self.pos.0, self.pos.1 + 1),
            Direction::South => (self.pos.0 + 1, self.pos.1),
            Direction::West => (self.pos.0, self.pos.1 - 1),
        };
        self.score += 1;
        self
    }

    fn move_left(mut self) -> Self {
        self.dir = match self.dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        self.score += 1000;
        self.move_straight()
    }

    fn move_right(mut self) -> Self {
        self.dir = match self.dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input1() {
        let input = include_str!("../test_input1.txt").trim();
        assert_eq!(process_input(input), 7036);
    }

    #[test]
    fn test_input2() {
        let input = include_str!("../test_input2.txt").trim();
        assert_eq!(process_input(input), 11048);
    }
}
