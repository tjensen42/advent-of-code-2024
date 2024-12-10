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
        .map(|trailhead| find_distinct_hiking_trails(&map, trailhead))
        .sum()
}

fn find_distinct_hiking_trails(map: &Grid<u32>, pos: (usize, usize)) -> usize {
    let new_poses = &[
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ];

    let mut sum = 0;
    for new_pos in new_poses {
        if map[*new_pos] == map[pos] + 1 {
            if map[*new_pos] == 9 {
                sum += 1;
            } else {
                sum += find_distinct_hiking_trails(map, *new_pos);
            }
        }
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
