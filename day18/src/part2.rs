use glam::IVec2;
use pathfinding::prelude::*;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

fn parse(s: &str) -> Vec<IVec2> {
    let mut v = Vec::new();
    for line in s.lines() {
        let mut t = line.split(",");
        let x = t
            .next()
            .expect("x should exist")
            .parse::<i32>()
            .expect("x should be a number");
        let y = t
            .next()
            .expect("y should exist")
            .parse::<i32>()
            .expect("y should be a number");
        v.push(IVec2::new(x, y));
    }
    v
}

fn path_exist(cells: &[IVec2], width: i32, height: i32) -> bool {
    let start = IVec2::new(0, 0);
    let end = IVec2::new(width - 1, height - 1);
    let xbounds = 0..height;
    let ybounds = 0..height;
    let result = dijkstra(
        &start,
        |&pos| {
            let mut v = Vec::new();
            for dir in DIRECTIONS {
                let next = pos + dir;
                if xbounds.contains(&next.x) && ybounds.contains(&next.y) && !cells.contains(&next)
                {
                    v.push((next, 1));
                }
            }
            v
        },
        |&pos| pos == end,
    );
    result.is_some()
}

pub fn solve(s: &str, width: i32, height: i32) -> String {
    let cells = parse(s);
    let mut right = cells.len();
    let mut left = 0;
    let mut c = 0;
    while right - left != 1 {
        c = (right + left) / 2;
        if path_exist(&cells[0..c], width, height) {
            left = c;
        } else {
            right = c;
        }
    }
    format!("{},{}", cells[c].x, cells[c].y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
        assert_eq!(solve(data, 7, 7), "6,1");
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data, 71, 71), "56,29");
    }
}
