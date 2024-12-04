use grid::Grid;

const PATTERN: &str = "XMAS";

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {}", process_input(input));
}

fn process_input(input: &str) -> usize {
    let mut grid = Grid::new(0, 0);
    input
        .lines()
        .for_each(|line| grid.push_row(line.chars().collect()));

    let mut sum = 0;

    // Rows
    sum += grid
        .iter_rows()
        .map(|c| find_pattern_in_line(&c.collect::<String>()))
        .sum::<usize>();
    sum += grid
        .iter_rows()
        .map(|c| find_pattern_in_line(&c.rev().collect::<String>()))
        .sum::<usize>();

    // Columns
    sum += grid
        .iter_cols()
        .map(|c| find_pattern_in_line(&c.collect::<String>()))
        .sum::<usize>();
    sum += grid
        .iter_cols()
        .map(|c| find_pattern_in_line(&c.rev().collect::<String>()))
        .sum::<usize>();

    // Diagonals
    let mut diagonals = get_diagonals(&grid);
    diagonals.retain(|d| d.len() >= PATTERN.len());

    sum += diagonals
        .iter()
        .map(|d| find_pattern_in_line(d))
        .sum::<usize>();
    sum += diagonals
        .iter()
        .map(|d| find_pattern_in_line(&d.chars().rev().collect::<String>()))
        .sum::<usize>();

    sum
}

fn find_pattern_in_line(line: &str) -> usize {
    let mut sum = 0;

    for i in 0..=line.len() - PATTERN.len() {
        if &line[i..i + PATTERN.len()] == PATTERN {
            sum += 1;
        }
    }

    sum
}

fn get_diagonals(grid: &Grid<char>) -> Vec<String> {
    let mut diagonals: Vec<String> = vec![];

    // Top-left to bottom-right diagonals
    for c in 0..grid.cols() {
        let mut diagonal = String::new();
        for (i, j) in (0..grid.rows()).zip(c..grid.cols()) {
            diagonal.push(grid[(i, j)]);
        }
        diagonals.push(diagonal);
    }

    for c in 1..grid.rows() {
        let mut diagonal = String::new();
        for (i, j) in (c..grid.rows()).zip(0..grid.cols()) {
            diagonal.push(grid[(i, j)]);
        }
        diagonals.push(diagonal);
    }

    // Top-right to bottom-left diagonals
    for c in 0..grid.cols() {
        let mut diagonal = String::new();
        for (i, j) in (0..grid.rows()).zip((0..=c).rev()) {
            diagonal.push(grid[(i, j)]);
        }
        diagonals.push(diagonal);
    }

    for c in 1..grid.rows() {
        let mut diagonal = String::new();
        for (i, j) in (c..grid.rows()).zip((0..grid.cols()).rev()) {
            diagonal.push(grid[(i, j)]);
        }
        diagonals.push(diagonal);
    }

    diagonals
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 18);
    }
}
