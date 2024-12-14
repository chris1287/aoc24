use glam::IVec2;
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
fn count(r: &[IVec2], Y: i32, X: i32) -> usize {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let HY = Y / 2;
    let HX = X / 2;
    for y in 0..Y {
        for x in 0..X {
            r.iter()
                .filter(|bot| bot.x == x && bot.y == y)
                .for_each(|_bot| {
                    if (0..HX).contains(&x) && (0..HY).contains(&y) {
                        a += 1;
                    } else if (HX + 1..X).contains(&x) && (0..HY).contains(&y) {
                        b += 1;
                    } else if (0..HX).contains(&x) && (HY + 1..Y).contains(&y) {
                        c += 1;
                    } else if (HX + 1..X).contains(&x) && (HY + 1..Y).contains(&y) {
                        d += 1;
                    }
                });
        }
    }
    a * b * c * d
}

#[allow(non_snake_case)]
pub fn solve(s: &str, Y: i32, X: i32) -> usize {
    let t = 100;
    let (_, v) = parse(s).unwrap();
    let positions = v
        .iter()
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
        .collect::<Vec<_>>();
    count(&positions, Y, X)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        assert_eq!(solve(&data, 7, 11), 12);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data, 103, 101), 229868730);
    }
}
