use core::panic;

use grid::Grid;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let (map_input, steps) = input.split_once("\n\n").unwrap();

    let mut map = parse_map(map_input);
    let steps = steps.chars().filter(|&c| c != '\n').collect::<Vec<_>>();
    let mut pos = map.indexed_iter().find(|(_, &c)| c == '@').unwrap().0;

    // Print map
    for row in map.iter_rows() {
        for c in row {
            print!("{}", c);
        }
        println!();
    }

    for step in steps {
        // println!("Step: {}", step);
        pos = match step {
            '^' => move_up(&mut map, pos),
            'v' => move_down(&mut map, pos),
            '<' => move_left(&mut map, pos),
            '>' => move_right(&mut map, pos),
            _ => panic!("Invalid step"),
        };

        // Print map
        // for row in map.iter_rows() {
        //     for c in row {
        //         print!("{}", c);
        //     }
        //     println!();
        // }
    }

    map.indexed_iter()
        .map(|(pos, &c)| if c == '[' { 100 * pos.0 + pos.1 } else { 0 })
        .sum()
}

fn boxes_can_be_moved_vertical(map: &Grid<char>, pos: (usize, usize), dir: isize) -> bool {
    let left;
    let right;

    if map[pos] == '[' {
        left = pos;
        right = (pos.0, pos.1 + 1);
    } else if map[pos] == ']' {
        left = (pos.0, pos.1 - 1);
        right = pos;
    } else {
        panic!("Invalid box position");
    }

    let next_left = (left.0.saturating_add_signed(dir), left.1);
    let next_right = (right.0.saturating_add_signed(dir), right.1);

    match (map[next_left], map[next_right]) {
        ('.', '.') => true,
        ('[', ']') => boxes_can_be_moved_vertical(map, next_left, dir),
        (']', '[') => {
            boxes_can_be_moved_vertical(map, next_left, dir)
                && boxes_can_be_moved_vertical(map, next_right, dir)
        }
        (']', '.') => boxes_can_be_moved_vertical(map, next_left, dir),
        ('.', '[') => boxes_can_be_moved_vertical(map, next_right, dir),
        _ => false,
    }
}

fn move_boxes_vertical(map: &mut Grid<char>, pos: (usize, usize), dir: isize) {
    let left;
    let right;

    if map[pos] == '[' {
        left = pos;
        right = (pos.0, pos.1 + 1);
    } else if map[pos] == ']' {
        left = (pos.0, pos.1 - 1);
        right = pos;
    } else {
        panic!("Invalid box position");
    }

    let next_left = (left.0.saturating_add_signed(dir), left.1);
    let next_right = (right.0.saturating_add_signed(dir), right.1);

    match (map[next_left], map[next_right]) {
        ('.', '.') => {}
        ('[', ']') => move_boxes_vertical(map, next_left, dir),
        (']', '[') => {
            move_boxes_vertical(map, next_left, dir);
            move_boxes_vertical(map, next_right, dir);
        }
        (']', '.') => {
            move_boxes_vertical(map, next_left, dir);
        }
        ('.', '[') => {
            move_boxes_vertical(map, next_right, dir);
        }
        _ => panic!("Invalid map state for moving boxes up"),
    }

    map[left] = '.';
    map[right] = '.';
    map[next_left] = '[';
    map[next_right] = ']';
}

fn move_up(map: &mut Grid<char>, pos: (usize, usize)) -> (usize, usize) {
    let dir = -1;
    let new_pos = (pos.0 - 1, pos.1);

    if map[new_pos] == '.' {
        map[pos] = '.';
        map[new_pos] = '@';
        return new_pos;
    } else if map[new_pos] == '[' {
        if boxes_can_be_moved_vertical(map, new_pos, dir) {
            move_boxes_vertical(map, new_pos, dir);
            map[pos] = '.';
            map[new_pos] = '@';
            return new_pos;
        }
    } else if map[new_pos] == ']' {
        let box_pos_left = (new_pos.0, new_pos.1 - 1);
        if boxes_can_be_moved_vertical(map, box_pos_left, dir) {
            move_boxes_vertical(map, box_pos_left, dir);
            map[pos] = '.';
            map[new_pos] = '@';
            return new_pos;
        }
    }

    pos
}

fn move_down(map: &mut Grid<char>, pos: (usize, usize)) -> (usize, usize) {
    let dir = 1;
    let new_pos = (pos.0 + 1, pos.1);

    if map[new_pos] == '.' {
        map[pos] = '.';
        map[new_pos] = '@';
        return new_pos;
    } else if map[new_pos] == '[' {
        if boxes_can_be_moved_vertical(map, new_pos, dir) {
            move_boxes_vertical(map, new_pos, dir);
            map[pos] = '.';
            map[new_pos] = '@';
            return new_pos;
        }
    } else if map[new_pos] == ']' {
        let box_pos_left = (new_pos.0, new_pos.1 - 1);
        if boxes_can_be_moved_vertical(map, box_pos_left, dir) {
            move_boxes_vertical(map, box_pos_left, dir);
            map[pos] = '.';
            map[new_pos] = '@';
            return new_pos;
        }
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
        let mut skipped = 0;
        let mut new_box_pos = new_pos;
        while map[new_box_pos] == ']' {
            skipped += 1;
            new_box_pos.1 -= 2;
        }
        if map[new_box_pos] == '.' {
            map[pos] = '.';
            map[new_pos] = '@';
            for _ in 0..skipped {
                map[new_box_pos] = '[';
                map[(new_box_pos.0, new_box_pos.1 + 1)] = ']';
                new_box_pos.1 += 2;
            }
            return new_pos;
        }
    }

    pos
}

fn move_right(map: &mut Grid<char>, pos: (usize, usize)) -> (usize, usize) {
    let new_pos = (pos.0, pos.1 + 1);

    if map[new_pos] == '.' {
        map[pos] = '.';
        map[new_pos] = '@';
        return new_pos;
    } else if map[new_pos] == '[' {
        let mut skipped = 0;
        let mut new_box_pos = new_pos;
        while map[new_box_pos] == '[' {
            skipped += 1;
            new_box_pos.1 += 2;
        }
        if map[new_box_pos] == '.' {
            map[pos] = '.';
            map[new_pos] = '@';
            for _ in 0..skipped {
                map[new_box_pos] = ']';
                map[(new_box_pos.0, new_box_pos.1 - 1)] = '[';
                new_box_pos.1 -= 2;
            }
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
