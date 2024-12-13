use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let (mut part1, mut part2) = (0, 0);
    for machine_input in input.split("\n\n") {
        let (ax, ay, bx, by, px, py) = machine_input
            .split(|c: char| !c.is_ascii_digit())
            .filter(|x| !x.is_empty())
            .map(|num| num.parse().unwrap())
            .collect_tuple()
            .unwrap();
        part1 += tokens_to_win(ax, ay, bx, by, px, py);
        part2 += tokens_to_win(ax, ay, bx, by, px + 10000000000000, py + 10000000000000);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn tokens_to_win(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> i64 {
    let b = (ax * py - ay * px) / (ax * by - ay * bx);
    let a = (px - bx * b) / ax;

    if (px, py) != (a * ax + b * bx, a * ay + b * by) {
        return 0;
    }

    a * 3 + b
}
