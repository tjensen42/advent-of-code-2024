use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    stones
        .iter()
        .map(|&stone| stones_count_after_blinks(stone, 75, &mut HashMap::new()))
        .sum()
}

fn stones_count_after_blinks(
    stone: usize,
    blinks_left: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blinks_left == 0 {
        return 1;
    }

    if let Some(&count) = cache.get(&(stone, blinks_left)) {
        return count;
    }

    let digits = (0..).take_while(|n| 10_usize.pow(*n) <= stone).count();

    let count;
    if stone == 0 {
        count = stones_count_after_blinks(1, blinks_left - 1, cache);
    } else if digits % 2 == 0 {
        let (left, right) = split_num(stone, digits);
        count = stones_count_after_blinks(left, blinks_left - 1, cache)
            + stones_count_after_blinks(right, blinks_left - 1, cache);
    } else {
        count = stones_count_after_blinks(stone * 2024, blinks_left - 1, cache);
    }

    cache.insert((stone, blinks_left), count);

    count
}

fn split_num(num: usize, digits: usize) -> (usize, usize) {
    let mut left_part = 0;
    let mut right_part = 0;
    let half_digits = digits / 2;

    for i in 0..digits {
        let pow_10_i = 10_usize.pow(i as u32);
        let digit = (num / pow_10_i) % 10;
        if i < half_digits {
            left_part += digit * pow_10_i;
        } else {
            right_part += digit * 10_usize.pow((i - half_digits) as u32);
        }
    }

    (left_part, right_part)
}
