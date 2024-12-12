use std::{
    char,
    collections::{HashMap, HashSet},
    vec,
};

use grid::Grid;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
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
            visited_garden_plots.extend(region.cells.keys());
            regions.push(region);
        }
    }

    regions.iter().map(|r| r.area() * r.sides_in_grid()).sum()
}

fn flood_fill_region(garden: &Grid<char>, start_pos: (usize, usize)) -> Region {
    let mut region = Region {
        cells: HashMap::new(),
    };

    let mut stack = vec![start_pos];
    while let Some(cell_pos) = stack.pop() {
        if region.cells.contains_key(&cell_pos) {
            continue;
        }

        let possible_neighbors = [
            (cell_pos.0 - 1, cell_pos.1),
            (cell_pos.0 + 1, cell_pos.1),
            (cell_pos.0, cell_pos.1 - 1),
            (cell_pos.0, cell_pos.1 + 1),
        ];

        let fences = Fences {
            top_fence: garden[possible_neighbors[0]] != garden[cell_pos],
            bottom_fence: garden[possible_neighbors[1]] != garden[cell_pos],
            left_fence: garden[possible_neighbors[2]] != garden[cell_pos],
            right_fence: garden[possible_neighbors[3]] != garden[cell_pos],
        };

        region.cells.insert(cell_pos, fences);
        stack.extend(
            possible_neighbors
                .into_iter()
                .filter(|&pos| garden[pos] == garden[cell_pos])
                .filter(|pos| !region.cells.contains_key(pos)),
        );
    }

    region
}

#[derive(Debug)]
struct Region {
    cells: HashMap<(usize, usize), Fences>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Fences {
    top_fence: bool,
    bottom_fence: bool,
    left_fence: bool,
    right_fence: bool,
}

impl Fences {
    fn corners(&self) -> usize {
        let mut corners = 0;
        if self.top_fence && self.left_fence {
            corners += 1;
        }
        if self.top_fence && self.right_fence {
            corners += 1;
        }
        if self.bottom_fence && self.left_fence {
            corners += 1;
        }
        if self.bottom_fence && self.right_fence {
            corners += 1;
        }
        corners
    }
}

impl Region {
    fn area(&self) -> usize {
        self.cells.len()
    }

    fn sides_in_grid(&self) -> usize {
        let mut sides = self.cells.values().map(|f| f.corners()).sum();

        for (pos, fences) in self.cells.iter() {
            let dia_left = (pos.0 + 1, pos.1 - 1);
            if let Some(dia_fences) = self.cells.get(&dia_left) {
                if (self.cells.get(&(pos.0, pos.1 - 1)).is_some()
                    || self.cells.get(&(pos.0 + 1, pos.1)).is_some())
                    && ((fences.left_fence && dia_fences.top_fence)
                        || (fences.bottom_fence && dia_fences.right_fence))
                {
                    sides += 1;
                }
            }

            let dia_right = (pos.0 + 1, pos.1 + 1);
            if let Some(dia_fences) = self.cells.get(&dia_right) {
                if (self.cells.get(&(pos.0, pos.1 + 1)).is_some()
                    || self.cells.get(&(pos.0 + 1, pos.1)).is_some())
                    && ((fences.right_fence && dia_fences.top_fence)
                        || (fences.bottom_fence && dia_fences.left_fence))
                {
                    sides += 1;
                }
            }
        }

        sides
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 1206);
    }
}
