fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let machines: Vec<Machine> = input.split("\n\n").map(Machine::from).collect();
    machines
        .iter()
        .filter_map(|machine| machine.min_tokens_to_win())
        .sum()
}

#[derive(Debug)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

impl Machine {
    fn min_tokens_to_win(&self) -> Option<usize> {
        let mut min_tokens = usize::MAX;

        for pa in 0..=100 {
            for pb in 0..=100 {
                let pos_x = pa * self.a.0 + pb * self.b.0;
                let pos_y = pa * self.a.1 + pb * self.b.1;

                if self.prize.0 == pos_x && self.prize.1 == pos_y {
                    let spend_tokens = pa * 3 + pb;
                    min_tokens = min_tokens.min(spend_tokens);
                } else if self.prize.0 < pos_x || self.prize.1 < pos_y {
                    break;
                }
            }
        }

        if min_tokens == usize::MAX {
            None
        } else {
            Some(min_tokens)
        }
    }
}

impl From<&str> for Machine {
    fn from(input: &str) -> Self {
        let mut coordinates = input.lines().map(coordinate_from_line);
        Machine {
            a: coordinates.next().unwrap(),
            b: coordinates.next().unwrap(),
            prize: coordinates.next().unwrap(),
        }
    }
}

fn coordinate_from_line(line: &str) -> (usize, usize) {
    let mut nums = line
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|n| n.parse().ok());

    (nums.next().unwrap(), nums.next().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 480);
    }
}
