use grid::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Guard {
    r: i32,
    c: i32,
    d: Direction,
}

fn parse(s: &str) -> (Grid<char>, Guard) {
    let mut guard = Guard {
        r: 0,
        c: 0,
        d: Direction::Up,
    };
    let rows = s.lines().count();
    let cols = s.lines().next().unwrap().len();
    let mut grid = Grid::new(rows, cols);
    for (r, line) in s.lines().enumerate() {
        for (c, character) in line.chars().enumerate() {
            let pos = grid.get_mut(r, c).unwrap();
            match character {
                '^' => {
                    *pos = '.';
                    guard.r = r as i32;
                    guard.c = c as i32;
                    guard.d = Direction::Up;
                }
                '<' => {
                    *pos = '.';
                    guard.r = r as i32;
                    guard.c = c as i32;
                    guard.d = Direction::Left;
                }
                '>' => {
                    *pos = '.';
                    guard.r = r as i32;
                    guard.c = c as i32;
                    guard.d = Direction::Right;
                }
                'v' => {
                    *pos = '.';
                    guard.r = r as i32;
                    guard.c = c as i32;
                    guard.d = Direction::Down;
                }
                _ => *pos = character,
            };
        }
    }
    (grid, guard)
}

pub fn solve(s: &str) -> usize {
    let (grid, mut guard) = parse(s);
    let mut m = HashMap::<(i32, i32), bool>::new();

    loop {
        let (r, c) = match guard.d {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        if guard.r + r < 0
            || guard.c + c < 0
            || guard.r + r >= grid.rows() as i32
            || guard.c + c >= grid.cols() as i32
        {
            break;
        }
        m.insert((guard.r, guard.c), true);
        if let Some(next) = grid.get(guard.r + r, guard.c + c) {
            if *next != '.' {
                guard.d = match guard.d {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                continue;
            }
        } else {
            unreachable!();
        }

        guard.r += r;
        guard.c += c;
    }

    m.len() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(solve(&data), 41);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 4752);
    }
}
