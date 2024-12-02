fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines()
    {
        let report: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        if report.windows(2).all(|x| x[0] < x[1] && x[0].abs_diff(x[1]) < 4)
        {
            sum += 1;
        }
        else if report.windows(2).all(|x| x[0] > x[1] && x[0].abs_diff(x[1]) < 4)
        {
            sum += 1;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 2);
    }
}
