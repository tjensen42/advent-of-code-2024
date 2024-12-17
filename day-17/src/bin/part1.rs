fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> String {
    let input_nums = input
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<_>>();

    let (mut a, mut b, mut c) = (input_nums[0], input_nums[1], input_nums[2]);
    let instructions = &input_nums[3..];
    let mut out = String::new();
    let mut pc = 0;

    while pc < instructions.len() - 1 {
        let instruction = instructions[pc];
        let operand = instructions[pc + 1];
        let combo = [0, 1, 2, 3, a, b, c];

        match instruction {
            0 => a /= 2_usize.pow(combo[operand] as u32),
            1 => b ^= operand,
            2 => b = combo[operand] % 8,
            3 if a != 0 => {
                pc = operand;
                continue;
            }
            4 => b ^= c,
            5 => out.push_str(&format!("{},", combo[operand] % 8)),
            6 => b = a / 2_usize.pow(combo[operand] as u32),
            7 => c = a / 2_usize.pow(combo[operand] as u32),
            _ => {}
        }
        pc += 2;
    }

    out.pop();
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input_part1.txt").trim();
        assert_eq!(process_input(input), "4,6,3,5,6,3,5,2,1,0");
    }
}
