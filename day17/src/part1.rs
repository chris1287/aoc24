use itertools::*;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

fn reg_a(s: &str) -> IResult<&str, u32> {
    let (s, n) = preceded(tag("Register A: "), u32)(s)?;
    let (s, _) = newline(s)?;
    Ok((s, n))
}

fn reg_b(s: &str) -> IResult<&str, u32> {
    let (s, n) = preceded(tag("Register B: "), u32)(s)?;
    let (s, _) = newline(s)?;
    Ok((s, n))
}

fn reg_c(s: &str) -> IResult<&str, u32> {
    let (s, n) = preceded(tag("Register C: "), u32)(s)?;
    let (s, _) = newline(s)?;
    Ok((s, n))
}

fn program(s: &str) -> IResult<&str, Vec<u32>> {
    let (s, _) = newline(s)?;
    let (s, v) = preceded(tag("Program: "), separated_list1(tag(","), u32))(s)?;
    Ok((s, v))
}

fn parse(s: &str) -> IResult<&str, (u32, u32, u32, Vec<u32>)> {
    let (s, (a, b, c)) = tuple((reg_a, reg_b, reg_c))(s)?;
    let (s, v) = program(s)?;
    Ok((s, (a, b, c, v)))
}

fn combo(a: u32, b: u32, c: u32, operand: u32) -> u32 {
    match operand {
        (0..=3) => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!("invalid operand: {operand}"),
    }
}

pub fn solve(s: &str) -> String {
    let (_, (mut a, mut b, mut c, v)) = parse(s).unwrap();
    let mut out = Vec::new();
    let mut i = 0;
    loop {
        if i >= v.len() {
            break;
        }
        let opcode = v[i];
        match opcode {
            0 => a = a / 2_u32.pow(combo(a, b, c, v[i + 1])),
            1 => b ^= v[i + 1],
            2 => b = combo(a, b, c, v[i + 1]) % 8,
            3 => {
                if a != 0 {
                    i = v[i + 1] as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => out.push(combo(a, b, c, v[i + 1]) % 8),
            6 => b = a / 2_u32.pow(combo(a, b, c, v[i + 1])),
            7 => c = a / 2_u32.pow(combo(a, b, c, v[i + 1])),
            _ => unreachable!("invalid opcode: {opcode}"),
        };
        i += 2;
    }
    out.iter().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(solve(data), "4,6,3,5,6,3,5,2,1,0");

        let data = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";
        assert_eq!(solve(data), "0,1,2");

        let data = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(solve(data), "4,2,5,6,7,7,7,7,3,1,0");

        let data = "Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0";
        assert_eq!(solve(data), "");
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), "1,7,6,5,1,0,5,0,7");
    }
}
