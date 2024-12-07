use std::iter::repeat;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let equations: Vec<_> = input
        .lines()
        .map(|line| {
            let mut equation = line.split(": ");
            let test_value: usize = equation.next().unwrap().parse().unwrap();
            let remaining: Vec<usize> = equation
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (test_value, remaining)
        })
        .collect();

    equations
        .into_iter()
        .filter(equation_can_be_made_true)
        .map(|(test_value, _)| test_value)
        .sum()
}

fn equation_can_be_made_true(equation: &(usize, Vec<usize>)) -> bool {
    for ops in repeat(['+', '*'].iter())
        .take(equation.1.len() - 1)
        .multi_cartesian_product()
    {
        let mut value = equation.1[0];
        for (i, &op) in ops.iter().enumerate() {
            match op {
                '+' => value += equation.1[i + 1],
                '*' => value *= equation.1[i + 1],
                _ => unreachable!(),
            }
        }
        if value == equation.0 {
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
