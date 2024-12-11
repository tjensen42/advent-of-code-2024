fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut stones: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    for _ in 0..25 {
        let mut tmp_stones = Vec::with_capacity(stones.len() * 2);

        for stone in stones.into_iter() {
            let digits = (0..).take_while(|n| 10_usize.pow(*n) <= stone).count();

            if stone == 0 {
                tmp_stones.push(1);
            } else if digits % 2 == 0 {
                let (left, right) = split_num(stone, digits);
                tmp_stones.push(left);
                tmp_stones.push(right);
            } else {
                tmp_stones.push(stone * 2024);
            }
        }

        stones = tmp_stones;
    }

    stones.len()
}

fn split_num(num: usize, digits: usize) -> (usize, usize) {
    let num_str = num.to_string();
    let splitted = num_str.split_at(digits / 2);
    (splitted.0.parse().unwrap(), splitted.1.parse().unwrap_or(0))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 55312);
    }
}
