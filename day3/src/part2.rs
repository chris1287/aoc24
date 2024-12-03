use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::i32;
use nom::combinator::map;
use nom::multi::many0;
use nom::IResult;

#[derive(Debug)]
pub enum Token {
    Operation(i32, i32),
    Skip,
    Do,
    Dont,
}

fn parse_operation(s: &str) -> IResult<&str, Token> {
    let (s, _) = tag("mul(")(s)?;
    let (s, a) = i32(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, b) = i32(s)?;
    let (s, _) = tag(")")(s)?;
    Ok((s, Token::Operation(a, b)))
}

fn parse(s: &str) -> IResult<&str, Vec<Token>> {
    let (s, operations) = many0(alt((
        parse_operation,
        map(tag("do()"), |_| Token::Do),
        map(tag("don't()"), |_| Token::Dont),
        map(take(1usize), |_| Token::Skip),
    )))(s)?;
    Ok((s, operations))
}

pub fn solve(s: &str) -> i32 {
    let (_, v) = parse(s).unwrap();
    let mut res = 0;
    let mut state = Token::Do;
    for token in v {
        match token {
            Token::Operation(a, b) => {
                if let Token::Do = state {
                    res += a * b;
                }
            }
            Token::Do => state = Token::Do,
            Token::Dont => state = Token::Dont,
            _ => {}
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve(&data), 48);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 63866497);
    }
}
