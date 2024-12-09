fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut layout = Vec::with_capacity(input.chars().count() * 4);

    let mut id = 0;
    for (i, c) in input.chars().enumerate() {
        let digit = c.to_digit(10).unwrap() as usize;
        if i % 2 == 1 {
            layout.extend(vec![None; digit]);
        } else {
            layout.extend(vec![Some(id); digit]);
            id += 1;
        }
    }

    loop {
        let len = layout.len();
        let first_none = layout.iter().position(|x| x.is_none()).unwrap();
        let last_some = len - 1 - layout.iter().rev().position(|x| x.is_some()).unwrap();

        if first_none < last_some {
            layout.swap(first_none, last_some);
        } else {
            break;
        }
    }

    layout.retain(|&x| x.is_some());

    let mut sum = 0;
    for (i, x) in layout.iter().enumerate() {
        if let Some(x) = x {
            sum += i * x;
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
        assert_eq!(process_input(input), 1928);
    }
}
