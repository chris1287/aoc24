use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::u32;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::ops::Range;

fn parse(s: &str) -> IResult<&str, (Vec<Range<u32>>, Vec<Vec<u32>>)> {
    let (s, rules) = separated_list1(
        line_ending,
        map(separated_pair(u32, tag("|"), u32), |(start, end)| Range {
            start,
            end,
        }),
    )(s)?;

    let (s, _) = multispace0(s)?;

    let (s, pages) = separated_list1(line_ending, separated_list1(tag(","), u32))(s)?;

    Ok((s, (rules, pages)))
}

fn rule_ok(rules: &[Range<u32>], pages: &[u32]) -> bool {
    for i in 0..pages.len() - 1 {
        for j in i + 1..pages.len() {
            if !rules
                .iter()
                .any(|rule| pages[i] == rule.start && pages[j] == rule.end)
            {
                return false;
            }
        }
    }
    true
}

pub fn solve(s: &str) -> u32 {
    let (_, (rules, pages)) = parse(s).unwrap();
    pages
        .iter()
        .filter(|&p| rule_ok(&rules, p))
        .map(|p| {
            let idx = p.len() / 2;
            p[idx]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(solve(data), 143);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 6051);
    }
}
