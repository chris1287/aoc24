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

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

fn parse_cell(s: Span) -> IResult<Span, (IVec2, char)> {
    let y = s.location_line() as i32 - 1;
    let x = s.get_column() as i32 - 1;
    let (s, c) = one_of("#.SE")(s)?;
    Ok((s, (IVec2::new(x, y), c)))
}

fn parse(s: Span) -> IResult<Span, HashMap<IVec2, char>> {
    let (s, v) = separated_list1(line_ending, many1(parse_cell))(s)?;
    let mut m = v.into_iter().flatten().collect::<HashMap<IVec2, char>>();
    m.retain(|_, c| *c != '.');
    Ok((s, m))
}

fn race(walls: &HashSet<IVec2>, start: &IVec2, end: &IVec2, ignore: &IVec2) -> i32 {
    let xbounds = 0..walls.iter().map(|p| p.x).max().unwrap();
    let ybounds = 0..walls.iter().map(|p| p.y).max().unwrap();
    dijkstra(
        start,
        |&pos| {
            let mut v = Vec::new();
            for dir in DIRECTIONS {
                let next = pos + dir;
                if xbounds.contains(&next.x)
                    && ybounds.contains(&next.y)
                    && (*ignore == next || !walls.contains(&next))
                {
                    v.push((next, 1));
                }
            }
            v
        },
        |&pos| pos == *end,
    )
    .expect("path should exist")
    .1
}

pub fn solve(s: &str) -> usize {
    let (_, mut walls) = parse(Span::new(s)).unwrap();
    let start = *walls
        .iter()
        .find(|(_, c)| **c == 'S')
        .expect("start should exist")
        .0;
    let end = *walls
        .iter()
        .find(|(_, c)| **c == 'E')
        .expect("end should exist")
        .0;
    walls.retain(|_, c| *c != 'S' && *c != 'E');
    let walls = walls.keys().copied().collect::<HashSet<_>>();
    let max = race(&walls, &start, &end, &start);
    let xmax = walls.iter().map(|p| p.x).max().unwrap();
    let ymax = walls.iter().map(|p| p.y).max().unwrap();
    walls
        .iter()
        .filter(|&wall| {
            if wall.x == 0 || wall.y == 0 || wall.x == xmax || wall.y == ymax {
                return false;
            }
            for dir in DIRECTIONS {
                if !walls.contains(&(wall + dir)) {
                    return true;
                }
            }
            false
        })
        .map(|wall| race(&walls, &start, &end, wall))
        .filter(|t| *t <= (max - 100))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!(solve(data), 0);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 1296);
    }
}
