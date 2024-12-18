use std::{cmp::min, collections::HashMap, vec};

use grid::Grid;

const MEMORY_SIZE: usize = 71;
const MIN_BYTES_TO_SIMULATE: usize = 1024;

fn main() {
    let input = include_str!("../input.txt").trim();
    let blocking_byte = process_input(input);
    println!("Part 2: {},{}", blocking_byte.1 - 1, blocking_byte.0 - 1);
}

fn process_input(input: &str) -> (usize, usize) {
    let mut memory = Grid::init(MEMORY_SIZE, MEMORY_SIZE, '.');

    memory.insert_row(0, vec!['#'; memory.cols()]);
    memory.push_row(vec!['#'; memory.cols()]);
    memory.insert_col(0, vec!['#'; memory.rows()]);
    memory.push_col(vec!['#'; memory.rows()]);

    let falling_bytes = parse_falling_bytes(input, 1);
    falling_bytes
        .iter()
        .take(MIN_BYTES_TO_SIMULATE)
        .for_each(|&byte_pos| {
            memory[byte_pos] = '#';
        });

    let start = (1, 1);
    let end = (memory.rows() - 2, memory.cols() - 2);
    let mut blocking_byte = (0, 0);

    for falling_byte in falling_bytes.into_iter().skip(MIN_BYTES_TO_SIMULATE) {
        memory[falling_byte] = '#';
        if find_shortest_path(&memory, start, end) == usize::MAX {
            blocking_byte = falling_byte;
            break;
        }
    }

    blocking_byte
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

fn parse_falling_bytes(input: &str, offset: usize) -> Vec<(usize, usize)> {
    input
        .lines()
        .flat_map(move |l| {
            l.split_once(',').map(|(x, y)| {
                (
                    y.parse::<usize>().unwrap() + offset,
                    x.parse::<usize>().unwrap() + offset,
                )
            })
        })
        .collect()
}
