use grid::*;

fn word_found(g: &Grid<char>, r: usize, c: usize) -> bool {
    if g.get(r, c) != Some(&'A') {
        return false;
    }
    if r > 0 && c + 1 < g.cols() && r + 1 < g.rows() && c > 0 {
        if let (Some(&ne), Some(&se), Some(&nw), Some(&sw)) = (
            g.get(r - 1, c + 1),
            g.get(r + 1, c + 1),
            g.get(r - 1, c - 1),
            g.get(r + 1, c - 1),
        ) {
            if (ne == 'S' && sw == 'M' || ne == 'M' && sw == 'S')
                && (nw == 'S' && se == 'M' || nw == 'M' && se == 'S')
            {
                return true;
            }
        }
    }

    false
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
            if word_found(&grid, r, c) {
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
        assert_eq!(solve(&data), 9);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 1982);
    }
}
