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
            let region = flood_fill_region(&garden, pos);
            visited.extend(region.cells.keys());
            regions.push(region);
        }
    }

    regions.iter().map(|r| r.area() * r.sides_in_grid()).sum()
}

fn flood_fill_region(garden: &Grid<char>, start_pos: (usize, usize)) -> Region {
    let mut region = Region::new();
    let mut stack = vec![start_pos];

    while let Some(cell_pos) = stack.pop() {
        if region.cells.contains_key(&cell_pos) {
            continue;
        }

        let possible_neighbors = get_neighbors(cell_pos);
        let cell_fences = Fences {
            top_fence: garden[possible_neighbors[0]] != garden[cell_pos],
            bottom_fence: garden[possible_neighbors[1]] != garden[cell_pos],
            left_fence: garden[possible_neighbors[2]] != garden[cell_pos],
            right_fence: garden[possible_neighbors[3]] != garden[cell_pos],
        };

        region.cells.insert(cell_pos, cell_fences);
        stack.extend(
            possible_neighbors
                .into_iter()
                .filter(|&pos| garden[pos] == garden[cell_pos])
                .filter(|pos| !region.cells.contains_key(pos)),
        );
    }

    region
}

fn get_neighbors((x, y): (usize, usize)) -> [(usize, usize); 4] {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

#[derive(Debug)]
struct Region {
    cells: HashMap<(usize, usize), Fences>,
}

impl Region {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    fn area(&self) -> usize {
        self.cells.len()
    }

    fn sides_in_grid(&self) -> usize {
        let mut sides = self.cells.values().map(|f| f.corners()).sum();

        for (pos, fences) in self.cells.iter() {
            sides += corners_of_diagonal_cells(self, pos, fences);
        }

        sides
    }
}

fn corners_of_diagonal_cells(region: &Region, pos: &(usize, usize), fences: &Fences) -> usize {
    let mut corners = 0;

    let dia_left = (pos.0 + 1, pos.1 - 1);
    if let Some(dia_fences) = region.cells.get(&dia_left) {
        if (region.cells.get(&(pos.0, pos.1 - 1)).is_some()
            || region.cells.get(&(pos.0 + 1, pos.1)).is_some())
            && ((fences.left_fence && dia_fences.top_fence)
                || (fences.bottom_fence && dia_fences.right_fence))
        {
            corners += 1;
        }
    }

    let dia_right = (pos.0 + 1, pos.1 + 1);
    if let Some(dia_fences) = region.cells.get(&dia_right) {
        if (region.cells.get(&(pos.0, pos.1 + 1)).is_some()
            || region.cells.get(&(pos.0 + 1, pos.1)).is_some())
            && ((fences.right_fence && dia_fences.top_fence)
                || (fences.bottom_fence && dia_fences.left_fence))
        {
            corners += 1;
        }
    }

    corners
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 1206);
    }
}
