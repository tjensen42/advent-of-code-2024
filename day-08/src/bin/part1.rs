use grid::Grid;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut grid: Grid<char> = Grid::new(0, 0);
    input
        .lines()
        .for_each(|l| grid.push_row(l.chars().collect()));

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (pos, c) in grid.indexed_iter() {
        if *c != '.' {
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
            println!("{:?}", combo);
            let antinode_pos = calculate_antinodes_positions(*combo[0], *combo[1]);
            antinodes.insert(antinode_pos.0);
            antinodes.insert(antinode_pos.1);
        }
    }
    // println!("{:?}", antinodes);

    antinodes
        .retain(|&(x, y)| x >= 0 && x < grid.rows() as isize && y >= 0 && y < grid.cols() as isize);

    // println!("{:?}", antinodes);

    antinodes.len()
}

fn calculate_antinodes_positions(
    pos1: (usize, usize),
    pos2: (usize, usize),
) -> ((isize, isize), (isize, isize)) {
    let diff_x = pos1.0 as isize - pos2.0 as isize;
    let diff_y = pos1.1 as isize - pos2.1 as isize;

    if diff_x > 0 && diff_y > 0 {
        // println!("{} {}", diff_x, diff_y);
        (
            (
                pos1.0 as isize - diff_x.abs(),
                pos1.1 as isize - diff_y.abs(),
            ),
            (
                pos2.0 as isize + diff_x.abs(),
                pos2.1 as isize + diff_y.abs(),
            ),
        )
    } else {
        // println!("{} {}", diff_x, diff_y);
        (
            (
                pos1.0 as isize + diff_x.abs(),
                pos1.1 as isize + diff_y.abs(),
            ),
            (
                pos2.0 as isize - diff_x.abs(),
                pos2.1 as isize - diff_y.abs(),
            ),
        )
    }

    // // let mut antinode1 =

    // (
    //     (pos1.0 as isize + diff_x, pos1.1 as isize + diff_y),
    //     (pos2.0 as isize + diff_x, pos2.1 as isize + diff_y),
    // )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 14);
    }
}
