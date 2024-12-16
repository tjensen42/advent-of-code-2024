use core::panic;
use grid::Grid;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let (map_input, steps) = input.split_once("\n\n").unwrap();
    let mut map = parse_map(map_input);
    let steps = steps.chars().filter(|&c| c != '\n');
    let mut pos = map.indexed_iter().find(|(_, &c)| c == '@').unwrap().0;

    for step in steps {
        pos = match step {
            '^' => move_vertical(&mut map, pos, -1),
            'v' => move_vertical(&mut map, pos, 1),
            '<' => move_horizontal(&mut map, pos, -1),
            '>' => move_horizontal(&mut map, pos, 1),
            _ => panic!("Invalid step"),
        };
    }

    map.indexed_iter()
        .map(|(pos, &c)| if c == '[' { 100 * pos.0 + pos.1 } else { 0 })
        .sum()
}

fn move_horizontal(map: &mut Grid<char>, pos: (usize, usize), dir: isize) -> (usize, usize) {
    let new_pos = (pos.0, pos.1.saturating_add_signed(dir));

    if map[new_pos] == '.' {
        map[pos] = '.';
        map[new_pos] = '@';
        return new_pos;
    } else if map[new_pos] == '[' || map[new_pos] == ']' {
        let mut skipped = 0;
        let mut new_box_pos = new_pos;
        while map[new_box_pos] == if dir == 1 { '[' } else { ']' } {
            skipped += 1;
            new_box_pos.1 = new_box_pos.1.saturating_add_signed(2 * dir);
        }
        if map[new_box_pos] == '.' {
            map[pos] = '.';
            map[new_pos] = '@';
            for _ in 0..skipped {
                map[new_box_pos] = if dir == 1 { ']' } else { '[' };
                map[(new_box_pos.0, new_box_pos.1.saturating_add_signed(-dir))] =
                    if dir == 1 { '[' } else { ']' };
                new_box_pos.1 = new_box_pos.1.saturating_add_signed(-2 * dir);
            }
            return new_pos;
        }
    }

    pos
}

fn move_vertical(map: &mut Grid<char>, pos: (usize, usize), dir: isize) -> (usize, usize) {
    let new_pos = (pos.0.saturating_add_signed(dir), pos.1);

    if map[new_pos] == '.' {
        map[pos] = '.';
        map[new_pos] = '@';
        return new_pos;
    } else if map[new_pos] == '[' || map[new_pos] == ']' {
        let box_pos = if map[new_pos] == '[' {
            new_pos
        } else {
            (new_pos.0, new_pos.1 - 1)
        };
        if boxes_can_be_moved_vertical(map, box_pos, dir) {
            move_boxes_vertical(map, box_pos, dir);
            map[pos] = '.';
            map[new_pos] = '@';
            return new_pos;
        }
    }

    pos
}

fn boxes_can_be_moved_vertical(map: &Grid<char>, pos: (usize, usize), dir: isize) -> bool {
    let (left, right) = if map[pos] == '[' {
        (pos, (pos.0, pos.1 + 1))
    } else if map[pos] == ']' {
        ((pos.0, pos.1 - 1), pos)
    } else {
        panic!("Invalid box position");
    };

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
    let (left, right) = if map[pos] == '[' {
        (pos, (pos.0, pos.1 + 1))
    } else if map[pos] == ']' {
        ((pos.0, pos.1 - 1), pos)
    } else {
        panic!("Invalid box position");
    };

    let next_left = (left.0.saturating_add_signed(dir), left.1);
    let next_right = (right.0.saturating_add_signed(dir), right.1);

    match (map[next_left], map[next_right]) {
        ('.', '.') => {}
        ('[', ']') => move_boxes_vertical(map, next_left, dir),
        (']', '[') => {
            move_boxes_vertical(map, next_left, dir);
            move_boxes_vertical(map, next_right, dir);
        }
        (']', '.') => move_boxes_vertical(map, next_left, dir),
        ('.', '[') => move_boxes_vertical(map, next_right, dir),
        _ => panic!("Invalid map state for moving boxes up"),
    }

    map[left] = '.';
    map[right] = '.';
    map[next_left] = '[';
    map[next_right] = ']';
}

fn parse_map(input: &str) -> Grid<char> {
    let mut map = Grid::new(0, 0);

    for line in input.lines() {
        let row: Vec<_> = line
            .chars()
            .flat_map(|c| match c {
                '#' => vec!['#', '#'],
                'O' => vec!['[', ']'],
                '.' => vec!['.', '.'],
                '@' => vec!['@', '.'],
                _ => unreachable!(),
            })
            .collect();
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
