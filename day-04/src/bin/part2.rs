use grid::Grid;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut grid = Grid::new(0, 0);
    input
        .lines()
        .for_each(|line| grid.push_row(line.chars().collect()));

    let mut sum = 0;

    for row in 1..grid.rows() - 1 {
        for col in 1..grid.cols() - 1 {
            if grid[(row, col)] == 'A' {
                let dia1 = [grid[(row - 1, col - 1)], grid[(row + 1, col + 1)]];
                let dia2 = [grid[(row + 1, col - 1)], grid[(row - 1, col + 1)]];

                if (dia1 == ['M', 'S'] || dia1 == ['S', 'M'])
                    && (dia2 == ['M', 'S'] || dia2 == ['S', 'M'])
                {
                    sum += 1;
                }
            }
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
        assert_eq!(process_input(input), 9);
    }
}
