use regex::Regex;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let matches = re.find_iter(input).map(|m| m.as_str());

    matches.map(parse_mul).map(|(a, b)| a * b).sum()
}

fn parse_mul(mul: &str) -> (usize, usize) {
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
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 161);
    }
}
