use glam::IVec2;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;
use pathfinding::prelude::*;
use std::collections::{HashMap, HashSet};

type Span<'a> = LocatedSpan<&'a str>;

fn parse_cell(s: Span) -> IResult<Span, (IVec2, char)> {
    let x = s.location_line() as i32 - 1;
    let y = s.get_column() as i32 - 1;
    let (s, c) = one_of("#.SE")(s)?;
    Ok((s, (IVec2::new(x, y), c)))
}

fn parse(s: Span) -> IResult<Span, (HashSet<IVec2>, IVec2, IVec2)> {
    let (s, v) = separated_list1(line_ending, many1(parse_cell))(s)?;
    let mut v = v.into_iter().flatten().collect::<HashMap<IVec2, char>>();
    let start = *v
        .iter()
        .find(|(_, c)| **c == 'S')
        .expect("start exists")
        .0;
    let end = *v
        .iter()
        .find(|(_, c)| **c == 'E')
        .expect("end exists")
        .0;
    v.retain(|_, c| *c != '#');
    let v = v.into_keys().collect::<HashSet<IVec2>>();
    Ok((s, (v, start, end)))
}

pub fn solve(s: &str) -> i32 {
    let (_, (cells, start, end)) = parse(Span::new(s)).unwrap();
    let direction = IVec2::Y;
    let result = dijkstra(
        &(start, direction),
        |&(pos, direction)| {
            let mut v = Vec::new();
            let straight = ((pos + direction, direction), 1);
            let turn_a = ((pos, direction.perp()), 1000);
            let turn_b = ((pos, -direction.perp()), 1000);
            if cells.contains(&straight.0 .0) {
                v.push(straight);
            }
            if cells.contains(&turn_a.0 .0) {
                v.push(turn_a);
            }
            if cells.contains(&turn_b.0 .0) {
                v.push(turn_b);
            }
            v
        },
        |&(pos, _)| pos == end,
    );
    let result = result.expect("a path should exist");
    result.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(solve(data), 7036);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 91464);
    }
}
