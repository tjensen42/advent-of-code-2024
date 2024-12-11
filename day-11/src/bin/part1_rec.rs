fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1 - Recursive: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    stones
        .iter()
        .map(|&stone| stones_count_after_blinks(stone, 25))
        .sum()
}

fn stones_count_after_blinks(stone: usize, blinks_left: usize) -> usize {
    if blinks_left == 0 {
        return 1;
    }

    let digits = (0..).take_while(|n| 10_usize.pow(*n) <= stone).count();

    let count;
    if stone == 0 {
        count = stones_count_after_blinks(1, blinks_left - 1);
    } else if digits % 2 == 0 {
        let (left, right) = split_num(stone, digits);
        count = stones_count_after_blinks(left, blinks_left - 1)
            + stones_count_after_blinks(right, blinks_left - 1);
    } else {
        count = stones_count_after_blinks(stone * 2024, blinks_left - 1);
    }

    count
}

fn split_num(num: usize, digits: usize) -> (usize, usize) {
    let num_str = num.to_string();
    let splitted = num_str.split_at(digits / 2);
    (splitted.0.parse().unwrap(), splitted.1.parse().unwrap_or(0))
}
