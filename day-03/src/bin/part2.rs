use regex::Regex;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let re = Regex::new(r"(mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\))").unwrap();
    let ops = re.find_iter(input).map(|m| m.as_str());

    let mut sum = 0;
    let mut do_mul = true;

    for op in ops {
        match op {
            "do()" => do_mul = true,
            "don't()" => do_mul = false,
            _ if do_mul => {
                let (a, b) = parse_op_mul(op);
                sum += a * b;
            }
            _ => panic!("Invalid operation"),
        }
    }

    sum
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
