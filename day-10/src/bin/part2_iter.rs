use std::collections::VecDeque;

use grid::Grid;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
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
        .map(|trailhead| count_valid_trails(&map, trailhead))
        .sum()
}

fn count_valid_trails(map: &Grid<u32>, start: (usize, usize)) -> usize {
    let mut queue = VecDeque::from([start]);
    let mut sum = 0;

    while let Some(pos) = queue.pop_front() {
        if map[pos] == 9 {
            sum += 1;
            continue;
        }

        let neighbors = [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ];

        neighbors.iter().for_each(|neighbor| {
            if map[*neighbor] == map[pos] + 1 {
                queue.push_back(*neighbor);
            }
        });
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 81);
    }
}
