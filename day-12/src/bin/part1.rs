use std::{collections::HashSet, vec};

use grid::Grid;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut garden = Grid::new(0, 0);

    for line in input.lines() {
        garden.push_row(line.chars().collect());
    }

    // Add boundaries
    garden.insert_row(0, vec![char::MAX; garden.cols()]);
    garden.push_row(vec![char::MAX; garden.cols()]);
    garden.insert_col(0, vec![char::MAX; garden.rows()]);
    garden.push_col(vec![char::MAX; garden.rows()]);

    let mut visited = HashSet::<(usize, usize)>::new();
    let mut regions = Vec::new();

    for (pos, &cell) in garden.indexed_iter() {
        if cell != char::MAX && !visited.contains(&pos) {
            let region = flood_fill(&garden, pos);
            visited.extend(&region.cells);
            regions.push(region);
        }
    }

    regions.iter().map(|r| r.area() * r.perimeter).sum()
}

fn flood_fill(garden: &Grid<char>, start: (usize, usize)) -> Region {
    let mut region = Region::new();
    let mut stack = vec![start];

    while let Some(pos) = stack.pop() {
        if !region.cells.insert(pos) {
            continue;
        }

        let neighbors = get_neighbors(pos)
            .into_iter()
            .filter(|&neighbor| garden[neighbor] == garden[pos])
            .collect::<Vec<_>>();

        region.perimeter += 4 - neighbors.len();
        stack.extend(neighbors);
    }

    region
}

fn get_neighbors((x, y): (usize, usize)) -> [(usize, usize); 4] {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

#[derive(Debug)]
struct Region {
    cells: HashSet<(usize, usize)>,
    perimeter: usize,
}

impl Region {
    fn new() -> Self {
        Self {
            cells: HashSet::new(),
            perimeter: 0,
        }
    }

    fn area(&self) -> usize {
        self.cells.len()
    }
}
