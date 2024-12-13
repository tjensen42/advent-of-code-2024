fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> isize {
    let machines: Vec<Machine> = input.split("\n\n").map(Machine::from).collect();
    machines
        .iter()
        .filter_map(|machine| machine.min_tokens())
        .sum()
}

#[derive(Debug)]
struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

impl Machine {
    fn min_tokens(&self) -> Option<isize> {
        let (ax, ay) = self.a;
        let (bx, by) = self.b;
        let (px, py) = self.prize;

        // Do some math on paper to get the formula...
        let b = (ax * py - ay * px) / (ax * by - ay * bx);
        let a = (px - bx * b) / ax;

        if px == a * ax + b * bx && py == a * ay + b * by {
            Some(3 * a + b)
        } else {
            None
        }
    }
}

impl From<&str> for Machine {
    fn from(input: &str) -> Self {
        let coordinates: Vec<_> = input.lines().map(coordinate_from_line).collect();
        Machine {
            a: coordinates[0],
            b: coordinates[1],
            prize: (
                10000000000000 + coordinates[2].0,
                10000000000000 + coordinates[2].1,
            ),
        }
    }
}

fn coordinate_from_line(line: &str) -> (isize, isize) {
    let mut nums = line
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|n| n.parse().ok());

    (nums.next().unwrap(), nums.next().unwrap())
}
