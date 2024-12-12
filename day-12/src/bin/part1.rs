use std::{char, collections::HashSet, vec};

use grid::Grid;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut garden: Grid<char> = Grid::new(0, 0);
    input
        .lines()
        .for_each(|l| garden.push_row(l.chars().collect()));

    garden.insert_row(0, vec![char::MAX; garden.cols()]);
    garden.insert_col(0, vec![char::MAX; garden.rows()]);
    garden.push_row(vec![char::MAX; garden.cols()]);
    garden.push_col(vec![char::MAX; garden.rows()]);

    let mut visited_garden_plots: HashSet<(usize, usize)> = HashSet::new();
    let mut regions = vec![];

    for (pos, _) in garden.indexed_iter() {
        if garden[pos] != char::MAX && !visited_garden_plots.contains(&pos) {
            let region = flood_fill_region(&garden, pos);
            visited_garden_plots.extend(region.cells.iter());
            regions.push(region);
        }
    }

    regions.iter().map(|r| r.area() * r.perimeter).sum()
}

fn flood_fill_region(garden: &Grid<char>, start_pos: (usize, usize)) -> Region {
    let mut region = Region {
        cells: HashSet::new(),
        perimeter: 0,
    };

    let mut stack = vec![start_pos];
    while let Some(cell_pos) = stack.pop() {
        if region.cells.contains(&cell_pos) {
            continue;
        }

        region.cells.insert(cell_pos);

        let possible_neighbors = [
            (cell_pos.0 - 1, cell_pos.1),
            (cell_pos.0 + 1, cell_pos.1),
            (cell_pos.0, cell_pos.1 - 1),
            (cell_pos.0, cell_pos.1 + 1),
        ];
        let neighbors: Vec<_> = possible_neighbors
            .into_iter()
            .filter(|&pos| garden[pos] == garden[cell_pos])
            .collect();

        region.perimeter += 4 - neighbors.len();

        stack.extend(neighbors.iter().filter(|pos| !region.cells.contains(pos)));
    }

    region
}

#[derive(Debug)]
struct Region {
    cells: HashSet<(usize, usize)>,
    perimeter: usize,
}

impl Region {
    fn area(&self) -> usize {
        self.cells.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 1930);
    }
}
