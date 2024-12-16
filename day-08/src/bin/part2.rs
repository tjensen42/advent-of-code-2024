use std::collections::{HashMap, HashSet};

use grid::Grid;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut grid: Grid<char> = Grid::new(0, 0);
    input
        .lines()
        .for_each(|l| grid.push_row(l.chars().collect()));

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (pos, c) in grid.indexed_iter() {
        if *c != '.' && *c != '#' {
            if let Some(poses) = antennas.get_mut(c) {
                poses.push(pos);
            } else {
                antennas.insert(*c, vec![pos]);
            }
        }
    }

    let mut antinodes = HashSet::new();
    for (_, positions) in antennas {
        for combo in positions.iter().combinations(2) {
            let antinode_pos = calculate_antinodes_positions(*combo[0], *combo[1], &grid);
            antinodes.extend(antinode_pos);
        }
    }

    antinodes.len()
}

fn calculate_antinodes_positions(
    pos1: (usize, usize),
    pos2: (usize, usize),
    grid: &Grid<char>,
) -> Vec<(usize, usize)> {
    let diff_x = pos1.0 as isize - pos2.0 as isize;
    let diff_y = pos1.1 as isize - pos2.1 as isize;

    let mut positions = Vec::new();
    let (mut x, mut y) = (pos1.0 as isize - diff_x, pos1.1 as isize - diff_y);
    while x >= 0 && x < grid.rows() as isize && y >= 0 && y < grid.cols() as isize {
        positions.push((x as usize, y as usize));
        x -= diff_x;
        y -= diff_y;
    }

    let (mut x, mut y) = (pos2.0 as isize + diff_x, pos2.1 as isize + diff_y);
    while x >= 0 && x < grid.rows() as isize && y >= 0 && y < grid.cols() as isize {
        positions.push((x as usize, y as usize));
        x += diff_x;
        y += diff_y;
    }
    positions
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 34);
    }
}
