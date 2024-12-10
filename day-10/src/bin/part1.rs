use std::collections::HashSet;

use grid::Grid;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut map: Grid<u32> = Grid::new(input.lines().count(), 0);

    input.lines().for_each(|line| {
        map.push_row(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    });

    map.insert_row(0, vec![u32::MAX; map.cols()]);
    map.insert_col(0, vec![u32::MAX; map.rows()]);
    map.push_row(vec![u32::MAX; map.cols()]);
    map.push_col(vec![u32::MAX; map.rows()]);

    map.indexed_iter()
        .filter_map(|(pos, &c)| if c == 0 { Some(pos) } else { None })
        .map(|trailhead| {
            let mut reached_tails = HashSet::new();
            find_valid_tails(&map, trailhead, &mut reached_tails);
            reached_tails.len()
        })
        .sum()
}

fn find_valid_tails(map: &Grid<u32>, pos: (usize, usize), tails: &mut HashSet<(usize, usize)>) {
    let new_poses = &[
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ];

    for new_pos in new_poses {
        if map[*new_pos] == map[pos] + 1 {
            if map[*new_pos] == 9 {
                tails.insert(*new_pos);
            } else {
                find_valid_tails(map, *new_pos, tails);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 36);
    }
}
