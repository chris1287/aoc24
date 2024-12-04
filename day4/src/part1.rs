use grid::*;

#[derive(Debug)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn word_found(g: &Grid<char>, r: usize, c: usize, d: Direction) -> bool {
    match d {
        Direction::North => {
            if r < 3 {
                return false;
            }
            if let (Some(&x), Some(&m), Some(&a), Some(&s)) = (
                g.get(r, c),
                g.get(r - 1, c),
                g.get(r - 2, c),
                g.get(r - 3, c),
            ) {
                x == 'X' && m == 'M' && a == 'A' && s == 'S'
            } else {
                false
            }
        }
        Direction::NorthEast => {
            if r < 3 || c > g.cols() - 4 {
                return false;
            }
            if let (Some(&x), Some(&m), Some(&a), Some(&s)) = (
                g.get(r, c),
                g.get(r - 1, c + 1),
                g.get(r - 2, c + 2),
                g.get(r - 3, c + 3),
            ) {
                x == 'X' && m == 'M' && a == 'A' && s == 'S'
            } else {
                false
            }
        }
        Direction::East => {
            if c > g.cols() - 4 {
                return false;
            }
            if let (Some(&x), Some(&m), Some(&a), Some(&s)) = (
                g.get(r, c),
                g.get(r, c + 1),
                g.get(r, c + 2),
                g.get(r, c + 3),
            ) {
                x == 'X' && m == 'M' && a == 'A' && s == 'S'
            } else {
                false
            }
        }
        Direction::SouthEast => {
            if r > g.rows() - 4 || c > g.cols() - 4 {
                return false;
            }
            if let (Some(&x), Some(&m), Some(&a), Some(&s)) = (
                g.get(r, c),
                g.get(r + 1, c + 1),
                g.get(r + 2, c + 2),
                g.get(r + 3, c + 3),
            ) {
                x == 'X' && m == 'M' && a == 'A' && s == 'S'
            } else {
                false
            }
        }
        Direction::South => {
            if r > g.rows() - 4 {
                return false;
            }
            if let (Some(&x), Some(&m), Some(&a), Some(&s)) = (
                g.get(r, c),
                g.get(r + 1, c),
                g.get(r + 2, c),
                g.get(r + 3, c),
            ) {
                x == 'X' && m == 'M' && a == 'A' && s == 'S'
            } else {
                false
            }
        }
        Direction::SouthWest => {
            if r > g.rows() - 4 || c < 3 {
                return false;
            }
            if let (Some(&x), Some(&m), Some(&a), Some(&s)) = (
                g.get(r, c),
                g.get(r + 1, c - 1),
                g.get(r + 2, c - 2),
                g.get(r + 3, c - 3),
            ) {
                x == 'X' && m == 'M' && a == 'A' && s == 'S'
            } else {
                false
            }
        }
        Direction::West => {
            if c < 3 {
                return false;
            }
            if let (Some(&x), Some(&m), Some(&a), Some(&s)) = (
                g.get(r, c),
                g.get(r, c - 1),
                g.get(r, c - 2),
                g.get(r, c - 3),
            ) {
                x == 'X' && m == 'M' && a == 'A' && s == 'S'
            } else {
                false
            }
        }
        Direction::NorthWest => {
            if r < 3 || c < 3 {
                return false;
            }
            if let (Some(&x), Some(&m), Some(&a), Some(&s)) = (
                g.get(r, c),
                g.get(r - 1, c - 1),
                g.get(r - 2, c - 2),
                g.get(r - 3, c - 3),
            ) {
                x == 'X' && m == 'M' && a == 'A' && s == 'S'
            } else {
                false
            }
        }
    }
}

fn parse(s: &str) -> Grid<char> {
    let mut tmp = Vec::new();
    let mut cols = 0;
    for lines in s.lines() {
        cols = lines.len();
        for c in lines.chars() {
            tmp.push(c);
        }
    }
    Grid::from_vec(tmp, cols)
}

pub fn solve(s: &str) -> usize {
    let grid = parse(s);
    let mut count = 0;
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            if word_found(&grid, r, c, Direction::North) {
                count += 1;
            }
            if word_found(&grid, r, c, Direction::NorthEast) {
                count += 1;
            }
            if word_found(&grid, r, c, Direction::East) {
                count += 1;
            }
            if word_found(&grid, r, c, Direction::SouthEast) {
                count += 1;
            }
            if word_found(&grid, r, c, Direction::South) {
                count += 1;
            }
            if word_found(&grid, r, c, Direction::SouthWest) {
                count += 1;
            }
            if word_found(&grid, r, c, Direction::West) {
                count += 1;
            }
            if word_found(&grid, r, c, Direction::NorthWest) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(solve(&data), 18);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 2464);
    }
}
