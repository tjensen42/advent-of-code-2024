fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut list1: Vec<usize> = vec![];
    let mut list2: Vec<usize> = vec![];

    for line in input.lines() {
        let splitted: Vec<_> = line.split_whitespace().collect();
        list1.push(splitted[0].parse().unwrap());
        list2.push(splitted[1].parse().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut sum = 0;
    for (i, j) in list1.iter().zip(list2) {
        sum += i.abs_diff(j);
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 11);
    }
}
