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

    let start = find_point(&map, 'S').unwrap();
    map[start] = '.';

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

        if let Some(cached_score) = cache.get_mut(&(reindeer.pos, reindeer.dir)) {
            if *cached_score > reindeer.score {
                *cached_score = reindeer.score;
            } else {
                continue;
            }
        } else {
            cache.insert((reindeer.pos, reindeer.dir), reindeer.score);
        }

        let next_positions = get_next_positions(map, &reindeer);
        stack.extend_from_slice(&next_positions);
    }

    min_score
}

fn find_point(map: &Grid<char>, point: char) -> Option<(usize, usize)> {
    map.indexed_iter()
        .find(|(_, &c)| c == point)
        .map(|(pos, _)| pos)
}

fn get_next_positions(map: &Grid<char>, reindeer: &Reindeer) -> Vec<Reindeer> {
    let (row, col) = reindeer.pos;
    let next_positions = [
        ((row - 1, col), Direction::North),
        ((row + 1, col), Direction::South),
        ((row, col - 1), Direction::West),
        ((row, col + 1), Direction::East),
    ];

    next_positions
        .iter()
        .filter(|(pos, _)| map[*pos] == '.' || map[*pos] == 'E')
        .map(|&(pos, dir)| Reindeer {
            pos,
            dir,
            score: reindeer.score + reindeer.dir.diff_score(dir) + 1,
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn diff_score(self, other: Direction) -> usize {
        const SCORES: [[usize; 4]; 4] = [
            [0, 1000, 2000, 1000], // North
            [1000, 0, 1000, 2000], // East
            [2000, 1000, 0, 1000], // South
            [1000, 2000, 1000, 0], // West
        ];

        SCORES[self as usize][other as usize]
    }
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
