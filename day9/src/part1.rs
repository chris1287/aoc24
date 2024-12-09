#[derive(Debug, Clone)]
pub enum Block {
    Full(usize),
    Free,
}

fn parse(s: &str) -> Vec<Block> {
    let mut v = Vec::new();
    let mut id = 0;
    for (pos, c) in s.chars().enumerate() {
        if let Some(n) = c.to_digit(10) {
            for _ in 0..n {
                if pos % 2 == 0 {
                    v.push(Block::Full(id));
                } else {
                    v.push(Block::Free);
                }
            }
            if pos % 2 == 0 {
                id += 1;
            }
        }
    }
    v
}

pub fn solve(s: &str) -> usize {
    let mut v = parse(s);
    let mut i = 0;
    let mut j = v.len() - 1;
    while i < j {
        match (&v[i], &v[j]) {
            (Block::Full(_), Block::Free) => {
                i += 1;
            }
            (Block::Free, Block::Free) => {
                j -= 1;
            }
            (Block::Free, Block::Full(_)) => {
                v.swap(i, j);
                j -= 1;
                i += 1;
            }
            (Block::Full(_), Block::Full(_)) => {
                i += 1;
            }
        }
    }
    v.into_iter()
        .enumerate()
        .filter(|(_, b)| matches!(b, Block::Full(_)))
        .fold(0, |acc, (pos, b)| match b {
            Block::Full(id) => acc + pos * id,
            _ => acc,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "2333133121414131402";
        assert_eq!(solve(data), 1928);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 6344673854800);
    }
}
