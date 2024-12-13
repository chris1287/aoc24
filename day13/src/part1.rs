use glam::{Mat2, Vec2};
use nom::{
    bytes::complete::take_while,
    character::complete::{i32, line_ending},
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
pub struct Machine {
    pub a: Vec2,
    pub b: Vec2,
    pub p: Vec2,
}

fn nan(c: char) -> bool {
    !c.is_ascii_digit()
}

fn parse_line(s: &str) -> IResult<&str, Vec2> {
    let (s, a) = preceded(take_while(nan), i32)(s)?;
    let (s, b) = preceded(take_while(nan), i32)(s)?;
    Ok((s, Vec2::new(a as f32, b as f32)))
}

fn parse_machine(s: &str) -> IResult<&str, Machine> {
    let (s, a) = parse_line(s)?;
    let (s, b) = parse_line(s)?;
    let (s, p) = parse_line(s)?;
    Ok((s, Machine { a, b, p }))
}

fn parse(s: &str) -> IResult<&str, Vec<Machine>> {
    let (s, machines) = separated_list1(many1(line_ending), parse_machine)(s)?;
    Ok((s, machines))
}

fn solution(m: &Machine) -> Option<(i32, i32)> {
    let c = Mat2::from_cols(m.a, m.b);
    let ci = c.inverse();
    let sol = ci * m.p;
    let a = sol.x.round();
    let b = sol.y.round();

    let check_x = (a * m.a.x + b * m.b.x) == m.p.x;
    let check_y = a * m.a.y + b * m.b.y == m.p.y;

    if check_x && check_y {
        Some((a as i32, b as i32))
    } else {
        None
    }
}

pub fn solve(s: &str) -> i32 {
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
        let data = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(solve(&data), 480);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 26299);
    }
}
