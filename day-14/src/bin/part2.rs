use std::{fs::File, io::Write};

use grid::Grid;
use itertools::Itertools;

const OUT_FILE: &str = "day-14-part2-out.txt";
const SPACE_WIDTH: usize = 101;
const SPACE_HEIGHT: usize = 103;

fn main() {
    let input = include_str!("../input.txt").trim();
    process_input(input);
    println!("Output written to {}", OUT_FILE);
    println!("Open the file and try to search for multiple ones in a row...");
}

fn process_input(input: &str) {
    let space = Grid::init(SPACE_HEIGHT, SPACE_WIDTH, 0);
    let mut robots = input.lines().map(Robot::from).collect_vec();
    let robots_clone = robots.clone();

    let file = File::create(OUT_FILE).unwrap();
    let mut buf_writer = std::io::BufWriter::new(file);

    for s in 1.. {
        robots
            .iter_mut()
            .for_each(|r| r.move_forward_in_grid(space.rows(), space.cols(), 1));

        writeln!(buf_writer, "After seconds: {s}").unwrap();
        write_space_to_file(&space, &robots, &mut buf_writer);

        if robots == robots_clone {
            break;
        }
    }
}

fn write_space_to_file(space: &Grid<usize>, robots: &[Robot], file: &mut impl Write) {
    let mut space = space.clone();
    for robot in robots {
        space[robot.pos] += 1;
    }

    for space_row in space.iter_rows() {
        for space_cell in space_row {
            write!(file, "{:1?}", space_cell).unwrap();
        }
        writeln!(file).unwrap();
    }
    writeln!(file).unwrap();
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
