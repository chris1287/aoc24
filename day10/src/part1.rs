use glam::IVec2;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use std::collections::HashSet;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
pub struct Token {
    pub c: u32,
    pub x: i32,
    pub y: i32,
}

fn token(s: Span) -> IResult<Span, (IVec2, i32)> {
    let x = s.location_line() as i32 - 1;
    let y = s.get_column() as i32 - 1;
    let (s, c) = one_of(".0123456789")(s)?;
    let c = match c.to_digit(10) {
        Some(n) => n as i32,
        None => -1,
    };
    Ok((s, (IVec2::new(x, y), c)))
}

fn parse(s: Span) -> IResult<Span, HashMap<IVec2, i32>> {
    let (s, v) = separated_list1(line_ending, many1(token))(s)?;

    let m: HashMap<IVec2, i32> = v.into_iter().flatten().collect();
    Ok((s, m))
}

fn recurse(
    data: &HashMap<IVec2, i32>,
    start: &IVec2,
    pos: &IVec2,
    n: i32,
    destinations: &mut HashSet<(IVec2, IVec2)>,
) {
    if n == 9 {
        destinations.insert((*start, *pos));
        return;
    }

    let x = pos + IVec2::X;
    if let Some(&value) = data.get(&x) {
        if value == n + 1 {
            recurse(data, start, &x, n + 1, destinations);
        }
    }
    let x = pos + IVec2::NEG_X;
    if let Some(&value) = data.get(&x) {
        if value == n + 1 {
            recurse(data, start, &x, n + 1, destinations);
        }
    }
    let x = pos + IVec2::Y;
    if let Some(&value) = data.get(&x) {
        if value == n + 1 {
            recurse(data, start, &x, n + 1, destinations);
        }
    }
    let x = pos + IVec2::NEG_Y;
    if let Some(&value) = data.get(&x) {
        if value == n + 1 {
            recurse(data, start, &x, n + 1, destinations);
        }
    }
}

pub fn solve(s: &str) -> usize {
    let (_, data) = parse(Span::new(s)).unwrap();
    let mut destinations = HashSet::new();
    data.iter()
        .filter(|(_, n)| **n == 0)
        .for_each(|(pos, n)| recurse(&data, pos, pos, *n, &mut destinations));

    destinations.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(solve(&data), 36);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 548);
    }
}
