fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut sections = input.split("\n\n");
    let section1 = sections.next().unwrap().lines();
    let section2 = sections.next().unwrap().lines();

    let rules: Vec<Vec<usize>> = section1
        .map(|l| l.split('|').map(|n| n.parse::<usize>().unwrap()).collect())
        .collect();
    let page_nums: Vec<Vec<usize>> = section2
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut sum = 0;
    for pages in &page_nums {
        if rules.iter().all(|r| list_is_valid(&pages, (r[0], r[1]))) {
            sum += pages[(pages.len() - 1) / 2];
        }
    }

    sum
}

fn list_is_valid(list: &[usize], rule: (usize, usize)) -> bool {
    let Some(pos1) = list.iter().position(|n| *n == rule.0) else {
        return true;
    };
    let Some(pos2) = list.iter().position(|n| *n == rule.1) else {
        return true;
    };

    pos1 < pos2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 143);
    }
}
