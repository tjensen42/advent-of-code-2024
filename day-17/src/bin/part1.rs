fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> String {
    let mut instructions = input
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<_>>();

    let mut reg_a = instructions.remove(0);
    let mut reg_b = instructions.remove(0);
    let mut reg_c = instructions.remove(0);
    let mut out = String::new();
    let mut pc = 0;

    while let Some(&instruction) = instructions.get(pc) {
        let operand = instructions[pc + 1];
        let combo_operand = |op: usize| match op {
            0..=3 => op,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => panic!("Invalid combo operator."),
        };

        match instruction {
            0 => reg_a /= 2_usize.pow(combo_operand(operand) as u32),
            1 => reg_b ^= operand,
            2 => reg_b = combo_operand(operand) % 8,
            3 => {
                if reg_a != 0 {
                    pc = operand;
                    continue;
                }
            }
            4 => reg_b ^= reg_c,
            5 => out.push_str(&format!("{},", combo_operand(operand) % 8)),
            6 => reg_b = reg_a / 2_usize.pow(combo_operand(operand) as u32),
            7 => reg_c = reg_a / 2_usize.pow(combo_operand(operand) as u32),
            _ => panic!("Invalid instruction"),
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
