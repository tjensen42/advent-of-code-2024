fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 2: {}", process_input(input));
}

#[derive(Debug, Clone, Copy)]
enum MemBlock {
    Some(usize, usize),
    None(usize),
}

fn process_input(input: &str) -> usize {
    let mut layout = Vec::with_capacity(input.chars().count() * 4);

    let mut id = 0;
    for (i, c) in input.chars().enumerate() {
        let digit = c.to_digit(10).unwrap() as usize;
        if i % 2 == 1 {
            layout.push(MemBlock::None(digit));
        } else {
            layout.push(MemBlock::Some(id, digit));
            id += 1;
        }
    }

    id -= 1;
    while id > 0 {
        let block_idx = get_block_index(&layout, id);

        if let MemBlock::Some(id, block_size) = layout[block_idx] {
            let free_space_idx = find_first_free_space(&layout, block_size);
            if let Some(free_space_idx) = free_space_idx {
                if free_space_idx < block_idx {
                    layout[block_idx] = MemBlock::None(block_size);
                    if let MemBlock::None(free_size) = layout[free_space_idx] {
                        layout[free_space_idx] = MemBlock::None(free_size - block_size);
                    }
                    layout.insert(free_space_idx, MemBlock::Some(id, block_size));
                }
            }
        }

        id -= 1;
    }

    let filesystem = layout
        .iter()
        .flat_map(|&x| match x {
            MemBlock::Some(id, size) => vec![id; size],
            MemBlock::None(size) => vec![0; size],
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i, x) in filesystem.iter().enumerate() {
        sum += i * x;
    }

    sum
}

fn get_block_index(layout: &[MemBlock], id: usize) -> usize {
    layout
        .iter()
        .position(|b| match b {
            MemBlock::Some(i, _) => *i == id,
            _ => false,
        })
        .unwrap()
}

fn find_first_free_space(layout: &[MemBlock], needed_space: usize) -> Option<usize> {
    layout.iter().position(|b| match b {
        MemBlock::None(space) => *space >= needed_space,
        _ => false,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../test_input.txt").trim();
        assert_eq!(process_input(input), 2858);
    }
}
