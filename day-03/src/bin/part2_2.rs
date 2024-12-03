
use regex::Regex;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let re = Regex::new(r"(mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\))").unwrap();
    let ops = re.find_iter(input).map(|m| Operation::from(m.as_str()));

    process_ops(ops, 0)
}

fn process_ops(ops: impl Iterator<Item = Operation>, mut sum: usize) -> usize {
    let mut do_mul = true;

    for op in ops {
        match op {
            Operation::Do => do_mul = true,
            Operation::Dont => do_mul = false,
            Operation::Mul(a, b) => {
                if do_mul {
                    sum += a * b;
                }
            }
        }
    }

    sum
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Do,
    Dont,
    Mul(usize, usize),
}

impl From<&str> for Operation {
    fn from(op: &str) -> Self {
        match op {
            "do()" => Operation::Do,
            "don't()" => Operation::Dont,
            _ => {
                let (a, b) = parse_op_mul(op);
                Operation::Mul(a, b)
            }
        }
    }
}

fn parse_op_mul(mul: &str) -> (usize, usize) {
    let num_str = mul
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty());
    let nums: Vec<_> = num_str.map(|num| num.parse().unwrap()).collect();
    (nums[0], nums[1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input_part2.txt").trim();
        assert_eq!(process_input(input), 48);
    }
}
