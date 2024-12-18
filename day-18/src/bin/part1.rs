use std::{cmp::min, collections::HashMap, usize, vec};

use grid::Grid;

const MEMORY_SIZE: usize = 71;
const BYTES_TO_SIMULATE: usize = 1024;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut memory = Grid::init(MEMORY_SIZE, MEMORY_SIZE, '.');

    input
        .lines()
        .flat_map(|l| {
            l.split_once(',')
                .map(|(x, y)| (y.parse().unwrap(), x.parse().unwrap()))
        })
        .take(BYTES_TO_SIMULATE)
        .for_each(|byte_pos| memory[byte_pos] = '#');

    memory.insert_row(0, vec!['#'; memory.cols()]);
    memory.push_row(vec!['#'; memory.cols()]);
    memory.insert_col(0, vec!['#'; memory.rows()]);
    memory.push_col(vec!['#'; memory.rows()]);

    for row in memory.iter_rows() {
        for c in row {
            print!("{c}")
        }
        println!()
    }

    let start = (1, 1);
    let end = (memory.rows() - 2, memory.cols() - 2);

    find_shortest_path(&memory, start, end)
}

fn find_shortest_path(space: &Grid<char>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut min_steps = usize::MAX;
    let mut visited = HashMap::new();
    let mut to_visit = vec![(start, 0)];

    while let Some((pos, steps)) = to_visit.pop() {
        if pos == end {
            min_steps = min(min_steps, steps);
            continue;
        }

        if visited.get(&pos).map_or(false, |&s| s <= steps) {
            continue;
        }
        visited.insert(pos, steps);

        let next_pos = [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ];

        to_visit.extend(
            next_pos
                .iter()
                .filter(|&&p| space[p] != '#')
                .map(|&p| (p, steps + 1)),
        );
    }

    min_steps
}
