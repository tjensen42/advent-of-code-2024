fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut list1: Vec<usize> = vec![];
    let mut list2: Vec<usize> = vec![];

    for line in input.lines() {
        let line_split: Vec<_> = line.split_whitespace().collect();
        list1.push(line_split[0].parse().unwrap());
        list2.push(line_split[1].parse().unwrap());
    }

    let mut sum = 0;
    for i in list1 {
        sum += i * list2.iter().filter(|j| i == **j).count();
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 31);
    }
}
