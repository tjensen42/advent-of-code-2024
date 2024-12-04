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

    grid.insert_row(0, vec!['0'; grid.cols()]);
    grid.insert_col(0, vec!['0'; grid.rows()]);
    grid.push_row(vec!['0'; grid.cols()]);
    grid.push_col(vec!['0'; grid.rows()]);

    let mut sum = 0;
    for ((row, col), c) in grid.indexed_iter() {
        if *c == 'A'
            && ((grid[(row - 1, col - 1)] == 'M' && grid[(row + 1, col + 1)] == 'S')
                || (grid[(row - 1, col - 1)] == 'S' && grid[(row + 1, col + 1)] == 'M'))
            && ((grid[(row + 1, col - 1)] == 'M' && grid[(row - 1, col + 1)] == 'S')
                || (grid[(row + 1, col - 1)] == 'S' && grid[(row - 1, col + 1)] == 'M'))
        {
            sum += 1;
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
