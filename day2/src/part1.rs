use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Debug, PartialEq)]
enum Interval {
    Unknown,
    Decreasing,
    Increasing,
}

fn parse_line(s: &str) -> IResult<&str, Vec<i32>> {
    let (s, v) = separated_list1(tag(" "), i32)(s)?;
    Ok((s, v))
}

fn parse(s: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (s, v) = separated_list1(line_ending, parse_line)(s)?;
    Ok((s, v))
}

pub fn solve(s: &str) -> usize {
    let (_, v1) = parse(s).unwrap();
    v1.iter()
        .filter(|v2| {
            let mut interval = Interval::Unknown;
            v2.windows(2).all(|w| {
                if w[0] < w[1] {
                    if interval == Interval::Decreasing {
                        return false;
                    }
                    interval = Interval::Increasing;
                } else if w[0] > w[1] {
                    if interval == Interval::Increasing {
                        return false;
                    }
                    interval = Interval::Decreasing;
                } else {
                    return false;
                }
                w[0].abs_diff(w[1]) <= 3
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(solve(data), 2);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 432);
    }
}
