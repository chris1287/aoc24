use std::collections::HashMap;

use glam::IVec2;
use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Type {
    Wall,
    Box,
    Bot,
    Air,
}

#[derive(Debug)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

type Span<'a> = LocatedSpan<&'a str>;

fn parse_cell(s: Span) -> IResult<Span, (IVec2, Type)> {
    let x = s.location_line() as i32 - 1;
    let y = s.get_column() as i32 - 1;
    let (s, t) = map(one_of("#.O@"), |c| match c {
        '#' => Type::Wall,
        'O' => Type::Box,
        '@' => Type::Bot,
        '.' => Type::Air,
        _ => unreachable!(),
    })(s)?;
    Ok((s, (IVec2::new(x, y), t)))
}

fn parse(s: Span) -> IResult<Span, (HashMap<IVec2, Type>, Vec<Direction>)> {
    let (s, cells) = separated_list1(line_ending, many1(parse_cell))(s)?;
    let cells = cells
        .into_iter()
        .flatten()
        .collect::<HashMap<IVec2, Type>>();
    let (s, _) = many1(line_ending)(s)?;
    let (s, moves) = many1(map(one_of("<v>^\n"), |c| match c {
        '<' => Some(Direction::W),
        '>' => Some(Direction::E),
        '^' => Some(Direction::N),
        'v' => Some(Direction::S),
        _ => None,
    }))(s)?;
    let moves = moves.into_iter().flatten().collect::<Vec<Direction>>();

    Ok((s, (cells, moves)))
}

pub fn solve(s: &str) -> i32 {
    let (_, (mut cells, moves)) = parse(Span::new(s)).unwrap();

    let (bot, _) = cells.iter_mut().find(|c| *c.1 == Type::Bot).unwrap();
    let mut bot = *bot;

    for m in moves {
        let dir = match m {
            Direction::N => IVec2::NEG_X,
            Direction::S => IVec2::X,
            Direction::E => IVec2::Y,
            Direction::W => IVec2::NEG_Y,
        };
        let mut next = bot + dir;
        if let Some(cell) = cells.get(&next) {
            match cell {
                Type::Wall => {}
                Type::Bot => unreachable!("more than one bot"),
                Type::Air => {
                    cells.remove(&bot);
                    cells.insert(bot, Type::Air);
                    cells.insert(next, Type::Bot);
                    bot = next;
                }
                Type::Box => {
                    let mut v = vec![next];
                    while let Some(cell) = cells.get(&next) {
                        match cell {
                            Type::Wall => break,
                            Type::Bot => unreachable!("more than one bot"),
                            Type::Air => {
                                cells.insert(*v.last().unwrap(), Type::Box);
                                cells.remove(&bot);
                                cells.insert(bot, Type::Air);
                                cells.insert(*v.first().unwrap(), Type::Bot);
                                bot = *v.first().unwrap();
                                break;
                            }
                            Type::Box => {
                                next += dir;
                                v.push(next);
                            }
                        }
                    }
                }
            }
        }
    }

    cells
        .iter()
        .filter(|c| *c.1 == Type::Box)
        .map(|(c, _)| c.x * 100 + c.y)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!(solve(&data), 2028);

        let data = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(solve(&data), 10092);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 1294459);
    }
}
