use std::collections::HashMap;
use std::collections::HashSet;

use glam::IVec2;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

#[derive(Debug)]
pub struct Region {
    pub positions: Vec<IVec2>,
    pub plant: char,
    pub perim: i32,
}

fn token(s: Span) -> IResult<Span, (IVec2, char)> {
    let x = s.location_line() as i32 - 1;
    let y = s.get_column() as i32 - 1;
    let (s, c) = one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(s)?;
    Ok((s, (IVec2::new(x, y), c)))
}

fn parse(s: Span) -> IResult<Span, HashMap<IVec2, char>> {
    let (s, v) = separated_list1(line_ending, many1(token))(s)?;
    Ok((s, v.into_iter().flatten().collect()))
}

fn create_region(map: &HashMap<IVec2, char>, start: &IVec2, plant: &char, region: &mut Region) {
    for dir in DIRECTIONS {
        let step = start + dir;
        if let Some(pos) = map.get(&step) {
            if pos == plant {
                if !region.positions.contains(&step) {
                    region.positions.push(step);
                    create_region(map, &step, plant, region);
                }
            } else {
                region.perim += 1;
            }
        } else {
            region.perim += 1;
        }
    }
}

pub fn solve(s: &str) -> i32 {
    let (_, map) = parse(Span::new(s)).unwrap();

    let mut visited: HashSet<IVec2> = HashSet::new();
    map.iter()
        .filter_map(|i| {
            if !visited.contains(i.0) {
                let mut region = Region {
                    positions: Vec::new(),
                    plant: *i.1,
                    perim: 0,
                };
                region.positions.push(*i.0);
                create_region(&map, i.0, i.1, &mut region);
                region.positions.iter().for_each(|pos| {
                    visited.insert(*pos);
                });
                Some(region)
            } else {
                None
            }
        })
        .map(|x| x.perim * x.positions.len() as i32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(solve(&data), 772);

        let data = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(solve(&data), 1930);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 1371306);
    }
}
