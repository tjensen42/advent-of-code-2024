fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut sections = input.split("\n\n");
    let section1 = sections.next().unwrap().lines();
    let section2 = sections.next().unwrap().lines();

    let rules: Vec<Vec<usize>> = section1
        .map(|l| l.split('|').map(|n| n.parse().unwrap()).collect())
        .collect();
    let mut page_nums: Vec<Vec<usize>> = section2
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    page_nums.retain(|x| !rules.iter().all(|r| list_is_valid(x, (r[0], r[1]))));

    loop {
        for rule in &rules {
            for pages in &mut page_nums {
                apply_rule_on_list(pages, (rule[0], rule[1]));
            }
        }
        if page_nums
            .iter()
            .all(|x| rules.iter().all(|r| list_is_valid(x, (r[0], r[1]))))
        {
            break;
        }
    }

    page_nums.iter().map(|x| x[(x.len() - 1) / 2]).sum()
}

fn apply_rule_on_list(list: &mut [usize], rule: (usize, usize)) {
    let Some(pos1) = list.iter().position(|n| *n == rule.0) else {
        return;
    };
    let Some(pos2) = list.iter().position(|n| *n == rule.1) else {
        return;
    };

    if pos1 > pos2 {
        list.swap(pos1, pos2);
    }
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
        assert_eq!(process_input(input), 123);
    }
}
