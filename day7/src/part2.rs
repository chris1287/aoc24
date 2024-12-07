use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::u64;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

fn parse(s: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    let (s, v) = separated_list1(
        line_ending,
        separated_pair(u64, tag(": "), separated_list1(tag(" "), u64)),
    )(s)?;
    Ok((s, v))
}

fn recurse(v: &[u64], i: usize, res: u64, partial: u64) -> u64 {
    if i == v.len() {
        if res == partial {
            return res;
        } else {
            return 0;
        }
    }
    if partial > res {
        return 0;
    }
    if recurse(v, i + 1, res, partial + v[i]) == res {
        return res;
    }
    if recurse(v, i + 1, res, partial * v[i]) == res {
        return res;
    }
    let s = partial.to_string() + v[i].to_string().as_str();
    if recurse(v, i + 1, res, s.parse::<u64>().unwrap()) == res {
        return res;
    }
    0
}

pub fn solve(s: &str) -> u64 {
    let (_, data) = parse(s).unwrap();
    data.iter().map(|(res, v)| recurse(v, 1, *res, v[0])).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(solve(&data), 11387);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 145149066755184);
    }
}
