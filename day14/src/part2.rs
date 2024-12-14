use glam::IVec2;
use itertools::*;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
pub struct Robot {
    pub p: IVec2,
    pub v: IVec2,
}

fn parse_line(s: &str) -> IResult<&str, Robot> {
    let (s, ((_, (px, py)), (_, (vx, vy)))) = separated_pair(
        separated_pair(tag("p"), tag("="), separated_pair(i32, tag(","), i32)),
        tag(" "),
        separated_pair(tag("v"), tag("="), separated_pair(i32, tag(","), i32)),
    )(s)?;
    Ok((
        s,
        Robot {
            p: IVec2::new(px, py),
            v: IVec2::new(vx, vy),
        },
    ))
}

fn parse(s: &str) -> IResult<&str, Vec<Robot>> {
    let (s, v) = separated_list1(line_ending, parse_line)(s)?;
    Ok((s, v))
}

#[allow(non_snake_case)]
pub fn solve(s: &str, Y: i32, X: i32) -> i32 {
    let (_, v) = parse(s).unwrap();
    (0..i32::MAX)
        .find(|t| {
            v.iter()
                .map(|bot| {
                    let mut x = (bot.p.x + bot.v.x * t) % X;
                    if x < 0 {
                        x += X;
                    }
                    let mut y = (bot.p.y + bot.v.y * t) % Y;
                    if y < 0 {
                        y += Y;
                    }
                    IVec2::new(x, y)
                })
                .all_unique() // according to Chris Biscardi, the tree appears when all the robots are at different
                              // locations... not sure how I was supposed to know this.
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data, 103, 101), 7861);
    }
}
