use glam::IVec2;
use pathfinding::prelude::*;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

fn parse(s: &str, n: usize) -> Vec<IVec2> {
    let mut v = Vec::new();
    for (i, line) in s.lines().enumerate() {
        if i == n {
            break;
        }
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

pub fn solve(s: &str, width: i32, height: i32, n: usize) -> i32 {
    let cells = parse(s, n);
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
    let result = result.expect("a path should exist");
    result.1
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
        assert_eq!(solve(data, 7, 7, 12), 22);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data, 71, 71, 1024), 318);
    }
}
