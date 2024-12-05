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
    let mut updates: Vec<Vec<usize>> = section2
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    updates.retain(|x| !rules.iter().all(|r| update_is_valid(x, (r[0], r[1]))));

    loop {
        for rule in &rules {
            for pages in &mut updates {
                apply_rule_on_update(pages, (rule[0], rule[1]));
            }
        }
        if updates
            .iter()
            .all(|x| rules.iter().all(|r| update_is_valid(x, (r[0], r[1]))))
        {
            break;
        }
    }

    updates.iter().map(|x| x[(x.len() - 1) / 2]).sum()
}

fn apply_rule_on_update(update: &mut [usize], rule: (usize, usize)) {
    let Some(pos1) = update.iter().position(|&p| p == rule.0) else {
        return;
    };
    let Some(pos2) = update.iter().position(|&p| p == rule.1) else {
        return;
    };

    if pos1 > pos2 {
        update.swap(pos1, pos2);
    }
}

fn update_is_valid(update: &[usize], rule: (usize, usize)) -> bool {
    let Some(pos1) = update.iter().position(|&p| p == rule.0) else {
        return true;
    };
    let Some(pos2) = update.iter().position(|&p| p == rule.1) else {
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
