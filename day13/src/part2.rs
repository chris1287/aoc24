use glam::{DMat2, DVec2};
use nom::{
    bytes::complete::take_while,
    character::complete::{i32, line_ending},
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
pub struct Machine {
    pub a: DVec2,
    pub b: DVec2,
    pub p: DVec2,
}

fn nan(c: char) -> bool {
    !c.is_ascii_digit()
}

fn parse_line(s: &str) -> IResult<&str, DVec2> {
    let (s, a) = preceded(take_while(nan), i32)(s)?;
    let (s, b) = preceded(take_while(nan), i32)(s)?;
    Ok((s, DVec2::new(a as f64, b as f64)))
}

fn parse_machine(s: &str) -> IResult<&str, Machine> {
    let (s, a) = parse_line(s)?;
    let (s, b) = parse_line(s)?;
    let (s, p) = parse_line(s)?;
    Ok((s, Machine { a, b, p: p + 10000000000000.0 }))
}

fn parse(s: &str) -> IResult<&str, Vec<Machine>> {
    let (s, machines) = separated_list1(many1(line_ending), parse_machine)(s)?;
    Ok((s, machines))
}

fn solution(m: &Machine) -> Option<(i64, i64)> {
    let c = DMat2::from_cols(m.a, m.b);
    let ci = c.inverse();
    let sol = ci * m.p;
    let a = sol.x.round();
    let b = sol.y.round();

    let check_x = (a * m.a.x + b * m.b.x) == m.p.x;
    let check_y = a * m.a.y + b * m.b.y == m.p.y;

    if check_x && check_y {
        Some((a as i64, b as i64))
    } else {
        None
    }
}

pub fn solve(s: &str) -> i64 {
    let (_, machines) = parse(s).unwrap();
    machines
        .iter()
        .filter_map(solution)
        .map(|(a, b)| a * 3 + b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 107824497933339);
    }
}
