use nom::character::complete::i32;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::multi::separated_list1;
use nom::IResult;

fn parse_line(s: &str) -> IResult<&str, (i32, i32)> {
    let (s, a) = i32(s)?;
    let (s, _) = multispace1(s)?;
    let (s, b) = i32(s)?;
    Ok((s, (a, b)))
}

fn parse(s: &str) -> IResult<&str, Vec<(i32, i32)>> {
    let (s, v) = separated_list1(line_ending, parse_line)(s)?;
    Ok((s, v))
}

pub fn solve(s: &str) -> i32 {
    let (_, v) = parse(s).unwrap();
    let (left, right): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
    left.iter()
        .map(|a| right.iter().filter(|b| a == *b).count() as i32 * (*a))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(solve(data), 31);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 20520794);
    }
}
