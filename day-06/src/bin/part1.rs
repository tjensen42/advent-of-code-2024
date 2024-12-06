use grid::Grid;
use std::{collections::HashSet, vec};

const BORDER: char = '0';
const OBSTACLE: char = '#';

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut map: Grid<char> = Grid::new(0, 0);
    input
        .lines()
        .for_each(|l| map.push_row(l.chars().collect()));

    // Add map border
    map.insert_row(0, vec![BORDER; map.cols()]);
    map.insert_col(0, vec![BORDER; map.rows()]);
    map.push_row(vec![BORDER; map.cols()]);
    map.push_col(vec![BORDER; map.rows()]);

    let start_pos = map.indexed_iter().find(|(_, c)| **c == '^').unwrap().0;

    move_until_border(&map, start_pos)
}

fn move_until_border(map: &Grid<char>, mut pos: (usize, usize)) -> usize {
    let mut dir = Direction::Up;
    let mut distinct_positions = HashSet::new();

    loop {
        distinct_positions.insert(pos);
        let next_pos = move_forward(pos, dir);
        match map[next_pos] {
            BORDER => break,
            OBSTACLE => dir.rotate_right(),
            _ => pos = next_pos,
        }
    }

    distinct_positions.len()
}

fn move_forward(pos: (usize, usize), dir: Direction) -> (usize, usize) {
    match dir {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0, pos.1 + 1),
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 41);
    }
}
