use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::i32;
use nom::combinator::map;
use nom::multi::many0;
use nom::IResult;

#[derive(Debug)]
pub struct Operation {
    pub a: i32,
    pub b: i32,
}

fn parse_operation(s: &str) -> IResult<&str, Option<Operation>> {
    let (s, _) = tag("mul(")(s)?;
    let (s, a) = i32(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, b) = i32(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, Some(Operation { a, b })))
}

fn parse(s: &str) -> IResult<&str, Vec<Option<Operation>>> {
    let (s, operations) = many0(alt((parse_operation, map(take(1usize), |_| None))))(s)?;
    Ok((s, operations))
}

pub fn solve(s: &str) -> i32 {
    let (_, v) = parse(s).unwrap();
    v.iter()
        .map(|o| {
            if let Some(o) = o {
                return o.a * o.b;
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(solve(&data), 161);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 171183089);
    }
}
