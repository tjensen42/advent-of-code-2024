use core::panic;

use grid::Grid;

fn main() {
    let input = include_str!("../test_input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let (map_input, steps) = input.split_once("\n\n").unwrap();

    let mut map = parse_map(map_input);
    let steps = steps.chars().filter(|&c| c != '\n').collect::<Vec<_>>();
    let mut pos = map.indexed_iter().find(|(_, &c)| c == '@').unwrap().0;

    steps
        .iter()
        .for_each(|&step| pos = move_robot_and_boxes(&mut map, pos, step));

    map.indexed_iter()
        .map(|(pos, &c)| if c == '[' { 100 * pos.0 + pos.1 } else { 0 })
        .sum()
}

fn move_robot_and_boxes(map: &mut Grid<char>, pos: (usize, usize), step: char) -> (usize, usize) {
    let moved_pos = |pos: (usize, usize)| -> (usize, usize) {
        match step {
            '^' => (pos.0 - 1, pos.1),
            '>' => (pos.0, pos.1 + 1),
            'v' => (pos.0 + 1, pos.1),
            '<' => (pos.0, pos.1 - 1),
            _ => unreachable!(),
        }
    };

    let new_robot_pos = moved_pos(pos);
    if map[new_robot_pos] == '.' {
        map[pos] = '.';
        map[new_robot_pos] = '@';
        return new_robot_pos;
    } else if map[new_robot_pos] == 'O' {
        let mut new_box_pos = moved_pos(new_robot_pos);
        while map[new_box_pos] == 'O' {
            new_box_pos = moved_pos(new_box_pos);
        }
        if map[new_box_pos] == '.' {
            map[pos] = '.';
            map[new_robot_pos] = '@';
            map[new_box_pos] = 'O';
            return new_robot_pos;
        }
    }
    pos
}

fn boxes_can_be_moved_up(map: &Grid<char>, pos: (usize, usize)) -> bool {
    let mut left;
    let mut right;

    if map[pos] == '[' {
        left = pos;
        right = (pos.0, pos.1 + 1);
    } else if map[pos] == ']' {
        left = (pos.0, pos.1 - 1);
        right = pos;
    } else {
        panic!("Invalid box position");
    }

    let left_upper = (left.0 - 1, left.1);
    let right_upper = (right.0 - 1, right.1);

    map[left_upper] == '.' && map[right_upper] == '.'
}

fn boxes_can_be_moved_down(map: &Grid<char>, pos: (usize, usize)) -> bool {
    let mut left;
    let mut right;

    if map[pos] == '[' {
        left = pos;
        right = (pos.0, pos.1 + 1);
    } else if map[pos] == ']' {
        left = (pos.0, pos.1 - 1);
        right = pos;
    } else {
        panic!("Invalid box position");
    }

    let left_lower = (left.0 + 1, left.1);
    let right_lower = (right.0 + 1, right.1);

    map[left_lower] == '.' && map[right_lower] == '.'
}

fn move_right(map: &mut Grid<char>, pos: (usize, usize)) -> (usize, usize) {
    let new_pos = (pos.0, pos.1 + 1);

    if map[new_pos] == '.' {
        map[pos] = '.';
        map[new_pos] = '@';
        return new_pos;
    } else if map[new_pos] == '[' {
        let mut new_box_pos = (pos.0, pos.1 + 2);
        while map[new_box_pos] == '[' {
            new_box_pos.1 += 2;
        }
        if map[new_box_pos] == '.' {
            map[pos] = '.';
            map[new_pos] = '@';
            map[new_box_pos] = '[';
            map[(new_box_pos.0, new_box_pos.1 + 1)] = ']';
            return new_pos;
        }

    pos
}

fn move_left(map: &mut Grid<char>, pos: (usize, usize)) -> (usize, usize) {
    let new_pos = (pos.0, pos.1 - 1);

    if map[new_pos] == '.' {
        map[pos] = '.';
        map[new_pos] = '@';
        return new_pos;
    } else if map[new_pos] == ']' {
        let mut new_box_pos = (pos.0, pos.1 - 2);
        while map[new_box_pos] == ']' {
            new_box_pos.1 -= 2;
        }
        if map[new_box_pos] == '.' {
            map[pos] = '.';
            map[new_pos] = '@';
            map[new_box_pos] = ']';
            map[(new_box_pos.0, new_box_pos.1 - 1)] = '[';
            return new_pos;
        }
    }

    pos
}

fn parse_map(input: &str) -> Grid<char> {
    let mut map = Grid::new(0, 0);

    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '#' => row.extend_from_slice(&['#', '#']),
                'O' => row.extend_from_slice(&['[', ']']),
                '.' => row.extend_from_slice(&['.', '.']),
                '@' => row.extend_from_slice(&['@', '.']),
                _ => unreachable!(),
            }
        }
        map.push_row(row);
    }

    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 9021);
    }
}
