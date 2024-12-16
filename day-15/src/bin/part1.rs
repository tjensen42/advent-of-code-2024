use grid::Grid;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut map = Grid::new(0, 0);

    let (map_input, steps) = input.split_once("\n\n").unwrap();
    map_input
        .lines()
        .for_each(|l| map.push_row(l.chars().collect()));

    let steps = steps.chars().filter(|&c| c != '\n').collect::<Vec<_>>();
    let mut pos = map.indexed_iter().find(|(_, &c)| c == '@').unwrap().0;

    steps
        .iter()
        .for_each(|&step| pos = move_robot_and_boxes(&mut map, pos, step));

    map.indexed_iter()
        .map(|(pos, &c)| if c == 'O' { 100 * pos.0 + pos.1 } else { 0 })
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 10092);
    }
}
