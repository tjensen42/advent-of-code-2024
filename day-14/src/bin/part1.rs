use std::cmp::Ordering;

use grid::Grid;
use itertools::Itertools;

const MOVES: isize = 100;
const SPACE_WIDTH: usize = 101;
const SPACE_HEIGHT: usize = 103;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let space = Grid::init(SPACE_HEIGHT, SPACE_WIDTH, 0);
    let mut robots = input.lines().map(Robot::from).collect_vec();

    robots
        .iter_mut()
        .for_each(|r| r.move_forward_in_grid(space.rows(), space.cols(), MOVES));

    let (q1, q2, q3, q4) = count_robots_in_quadrant(&robots);
    q1 * q2 * q3 * q4
}

fn count_robots_in_quadrant(robots: &[Robot]) -> (usize, usize, usize, usize) {
    let (mid_row, mid_col) = (SPACE_HEIGHT / 2, SPACE_WIDTH / 2);
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    for robot in robots {
        let (row, col) = robot.pos;

        match (row.cmp(&mid_row), col.cmp(&mid_col)) {
            (Ordering::Less, Ordering::Less) => q1 += 1,
            (Ordering::Less, Ordering::Greater) => q2 += 1,
            (Ordering::Greater, Ordering::Less) => q3 += 1,
            (Ordering::Greater, Ordering::Greater) => q4 += 1,
            _ => {}
        }
    }

    (q1, q2, q3, q4)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    pos: (usize, usize),
    dir: (isize, isize),
}

impl Robot {
    fn move_forward_in_grid(&mut self, rows: usize, cols: usize, times: isize) {
        let rows = rows as isize;
        let cols = cols as isize;

        let new_py = self.pos.0 as isize + times * self.dir.0;
        let new_px = self.pos.1 as isize + times * self.dir.1;

        let wrapped_py = ((new_py % rows) + rows) % rows;
        let wrapped_px = ((new_px % cols) + cols) % cols;

        self.pos = (wrapped_py as usize, wrapped_px as usize)
    }
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let (px, py, dx, dy) = value
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();

        Self {
            pos: (py as usize, px as usize),
            dir: (dy, dx),
        }
    }
}
