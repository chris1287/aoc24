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
    pub positions: HashSet<IVec2>,
    pub plant: char,
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
            if pos == plant && !region.positions.contains(&step) {
                region.positions.insert(step);
                create_region(map, &step, plant, region);
            }
        }
    }
}

fn count_corners(x: &IVec2, positions: &HashSet<IVec2>) -> i32 {
    // corner type 1:
    // O X
    // O O
    //
    // corner type2:
    // X _
    // O X

    let dirs = [
        (IVec2::NEG_X, IVec2::Y),
        (IVec2::Y, IVec2::X),
        (IVec2::X, IVec2::NEG_Y),
        (IVec2::NEG_Y, IVec2::NEG_X),
    ];
    let mut corners = 0;
    for (dir1, dir2) in dirs {
        let dir3 = dir1 + dir2;
        if (positions.contains(&(x + dir1))
            && positions.contains(&(x + dir2))
            && !positions.contains(&(x + dir3)))
            || (!positions.contains(&(x + dir1)) && !positions.contains(&(x + dir2)))
        {
            corners += 1;
        }
    }
    corners
}

pub fn solve(s: &str) -> i32 {
    let (_, map) = parse(Span::new(s)).unwrap();

    let mut visited: HashSet<IVec2> = HashSet::new();
    map.iter()
        .filter_map(|i| {
            if !visited.contains(i.0) {
                let mut region = Region {
                    positions: HashSet::new(),
                    plant: *i.1,
                };
                region.positions.insert(*i.0);
                create_region(&map, i.0, i.1, &mut region);
                region.positions.iter().for_each(|pos| {
                    visited.insert(*pos);
                });
                Some(region)
            } else {
                None
            }
        })
        .map(|region| {
            let mut corners = 0;
            for pos in &region.positions {
                corners += count_corners(pos, &region.positions);
            }
            corners * region.positions.len() as i32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(solve(&data), 80);

        let data = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(solve(&data), 436);

        let data = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(solve(&data), 236);

        let data = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(solve(&data), 368);

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
        assert_eq!(solve(&data), 1206);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 805880);
    }
}
