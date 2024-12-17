use rayon::prelude::*;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut instructions = input
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<_>>();

    instructions.remove(0); // reg_a
    let reg_b = instructions.remove(0);
    let reg_c = instructions.remove(0);
    let out_template = Vec::with_capacity(10 * 1024);

    (0..usize::MAX)
        .into_par_iter()
        .find_first(|&i| {
            let mut reg_a = i;
            let mut reg_b = reg_b;
            let mut reg_c = reg_c;
            let mut out = out_template.clone();
            let mut pc = 0;

            while pc < instructions.len() {
                let instruction = instructions[pc];
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
                    5 => out.push(combo_operand(operand) % 8),
                    6 => reg_b = reg_a / 2_usize.pow(combo_operand(operand) as u32),
                    7 => reg_c = reg_a / 2_usize.pow(combo_operand(operand) as u32),
                    _ => panic!("Invalid instruction"),
                }
                pc += 2;
            }

            out == instructions
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input_part2.txt").trim();
        assert_eq!(process_input(input), 117440);
    }
}
