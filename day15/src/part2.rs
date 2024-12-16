use std::collections::{HashMap, HashSet};

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
    LBox,
    RBox,
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

fn parse_cell(s: Span) -> IResult<Span, Vec<(IVec2, Type)>> {
    let x = s.location_line() as i32 - 1;
    let y = (s.get_column() as i32 - 1) * 2;
    let (s, (t1, t2)) = map(one_of("#.O@"), |c| match c {
        '#' => (Type::Wall, Type::Wall),
        'O' => (Type::LBox, Type::RBox),
        '@' => (Type::Bot, Type::Air),
        '.' => (Type::Air, Type::Air),
        _ => unreachable!(),
    })(s)?;
    let mut v = Vec::new();
    v.push((IVec2::new(x, y), t1));
    v.push((IVec2::new(x, y + 1), t2));
    Ok((s, v))
}

fn parse(s: Span) -> IResult<Span, (HashMap<IVec2, Type>, Vec<Direction>)> {
    let (s, cells) = separated_list1(line_ending, many1(parse_cell))(s)?;
    let m = cells
        .into_iter()
        .flatten()
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

    Ok((s, (m, moves)))
}

fn handle_horizontal_movement(cells: &mut HashMap<IVec2, Type>, bot: &mut IVec2, dir: &IVec2) {
    let mut next = *bot + dir;
    let mut v = Vec::new();
    while let Some(t) = cells.get(&next) {
        match t {
            Type::Wall => return,
            Type::LBox => v.push(next),
            Type::RBox => v.push(next),
            Type::Bot => panic!("more than one bot"),
            Type::Air => {
                cells.remove(&next);
                for cell in v.iter().rev() {
                    let x = cells.remove(cell).expect("there was a box here");
                    cells.insert(next, x);
                    next -= dir;
                }
                cells.insert(next, Type::Bot);
                *bot = next;
                next -= dir;
                cells.insert(next, Type::Air);
                return;
            }
        }
        next += dir;
    }
}

fn step_vertical(
    cells: &mut HashMap<IVec2, Type>,
    left: &IVec2,
    right: &IVec2,
    dir: &IVec2,
    boxes: &mut HashSet<IVec2>,
) -> bool {
    boxes.insert(*left);
    boxes.insert(*right);

    if let (Some(Type::Air), Some(Type::Air)) =
        (cells.get(&(*left + dir)), cells.get(&(*right + dir)))
    {
        return true;
    }

    if let Some(Type::Wall) = cells.get(&(*left + dir)) {
        return false;
    }
    if let Some(Type::Wall) = cells.get(&(*right + dir)) {
        return false;
    }

    if let (Some(Type::LBox), Some(Type::RBox)) =
        (cells.get(&(*left + dir)), cells.get(&(*right + dir)))
    {
        return step_vertical(cells, &(*left + dir), &(*right + dir), dir, boxes);
    }

    if let Some(Type::RBox) = cells.get(&(*left + dir)) {
        if !step_vertical(
            cells,
            &(*left + IVec2::NEG_Y + dir),
            &(*left + dir),
            dir,
            boxes,
        ) {
            return false;
        }
    }

    if let Some(Type::LBox) = cells.get(&(*right + dir)) {
        if !step_vertical(
            cells,
            &(*right + dir),
            &(*right + IVec2::Y + dir),
            dir,
            boxes,
        ) {
            return false;
        }
    }

    return true;
}

fn handle_vertical_movement(cells: &mut HashMap<IVec2, Type>, bot: &mut IVec2, dir: &IVec2) {
    let up = *bot + dir;
    let upleft = *bot + dir + IVec2::NEG_Y;
    let upright = *bot + dir + IVec2::Y;
    let mut group = HashSet::new();
    let mut shift = false;

    if let (Some(Type::LBox), Some(Type::RBox)) = (cells.get(&up), cells.get(&upright)) {
        if step_vertical(cells, &up, &upright, dir, &mut group) {
            shift = true;
        }
    } else if let (Some(Type::RBox), Some(Type::LBox)) = (cells.get(&up), cells.get(&upleft)) {
        if step_vertical(cells, &upleft, &up, dir, &mut group) {
            shift = true;
        }
    } else if let Some(Type::Air) = cells.get(&up) {
        shift = true;
        group.insert(up);
    }

    if shift {
        let rows = cells.keys().map(|c| c.x).max().unwrap();
        let cols = cells.keys().map(|c| c.y).max().unwrap();
        if dir.x == -1 {
            for x in 0..=rows {
                for y in 0..=cols {
                    if let Some(cell) = group.get(&IVec2::new(x, y)) {
                        let x = cells.remove(cell).expect("there was a box here");
                        cells.insert(*cell + dir, x);
                        cells.insert(*cell, Type::Air);
                    }
                }
            }
        } else {
            for x in (0..=rows).rev() {
                for y in 0..=cols {
                    if let Some(cell) = group.get(&IVec2::new(x, y)) {
                        let x = cells.remove(cell).expect("there was a box here");
                        cells.insert(*cell + dir, x);
                        cells.insert(*cell, Type::Air);
                    }
                }
            }
        }
        cells.insert(*bot + dir, Type::Bot);
        cells.insert(*bot, Type::Air);
        *bot += dir;
    }
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
        let next = bot + dir;
        if let Some(t) = cells.get(&next) {
            match t {
                Type::Wall => {}
                Type::LBox => {
                    if dir.x == 0 {
                        handle_horizontal_movement(&mut cells, &mut bot, &dir);
                    } else {
                        handle_vertical_movement(&mut cells, &mut bot, &dir);
                    }
                }
                Type::RBox => {
                    if dir.x == 0 {
                        handle_horizontal_movement(&mut cells, &mut bot, &dir);
                    } else {
                        handle_vertical_movement(&mut cells, &mut bot, &dir);
                    }
                }
                Type::Bot => panic!("more than one bot"),
                Type::Air => {
                    cells.remove(&bot);
                    cells.insert(bot, Type::Air);
                    cells.insert(next, Type::Bot);
                    bot = next;
                }
            }
        }
    }

    cells
        .iter()
        .filter(|c| *c.1 == Type::LBox)
        .map(|(c, _)| c.x * 100 + c.y)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
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
        assert_eq!(solve(&data), 9021);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 1319212);
    }
}
