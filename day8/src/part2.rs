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

fn token(s: Span) -> IResult<Span, (IVec2, char)> {
    let r = s.location_line();
    let c = s.get_column();
    let (s, token) = one_of(".0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")(s)?;
    Ok((s, (IVec2::new(r as i32 - 1, c as i32 - 1), token)))
}

fn parse(s: Span) -> IResult<Span, (HashMap<IVec2, char>, i32, i32)> {
    let (s, grid) = separated_list1(line_ending, many1(token))(s)?;
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let nodes = grid
        .into_iter()
        .flatten()
        .filter(|(_, c)| *c != '.')
        .collect::<HashMap<IVec2, char>>();
    Ok((s, (nodes, rows, cols)))
}

pub fn solve(s: &str) -> usize {
    let (_, (nodes, rows, cols)) = parse(Span::new(s)).unwrap();
    let mut antinodes = HashSet::new();
    let xrange = 0..rows;
    let yrange = 0..cols;
    for a in &nodes {
        for b in nodes.iter().filter(|&b| a.0 != b.0 && a.1 == b.1) {
            let mut inside = true;
            let mut mul = 1;
            while inside {
                inside = false;
                let diff = (a.0 - b.0) * mul;
                let anti1 = a.0 + diff;
                let anti2 = b.0 - diff;
                if xrange.contains(&anti1.x) && yrange.contains(&anti1.y) {
                    antinodes.insert(anti1);
                    inside = true;
                }
                if xrange.contains(&anti2.x) && yrange.contains(&anti2.y) {
                    antinodes.insert(anti2);
                    inside = true;
                }
                antinodes.insert(*a.0);
                antinodes.insert(*b.0);
                mul += 1;
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(solve(&data), 34);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 1169);
    }
}
