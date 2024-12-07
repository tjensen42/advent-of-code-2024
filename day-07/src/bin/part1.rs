use std::iter::repeat;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let test_value = parts.next().unwrap().parse().unwrap();
            let remaining = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (test_value, remaining)
        })
        .filter(equation_can_be_made_true)
        .map(|(test_value, _)| test_value)
        .sum()
}

fn equation_can_be_made_true(equation: &(usize, Vec<usize>)) -> bool {
    let (test_value, numbers) = equation;

    for ops in repeat(['+', '*'].iter())
        .take(numbers.len() - 1)
        .multi_cartesian_product()
    {
        let mut value = numbers[0];
        for (i, op) in ops.iter().enumerate() {
            match op {
                '+' => value += numbers[i + 1],
                '*' => value *= numbers[i + 1],
                _ => unreachable!(),
            }
        }
        if value == *test_value {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 3749);
    }
}
