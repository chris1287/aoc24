use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::{many1, separated_list1},
    IResult,
};

fn parse(s: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (s, towels) = separated_list1(tag(", "), alpha1)(s)?;
    let (s, _) = many1(line_ending)(s)?;
    let (s, patterns) = separated_list1(line_ending, alpha1)(s)?;
    Ok((s, (towels, patterns)))
}

fn recurse(pattern: &str, towels: &Vec<&str>, cache: &mut HashMap<String, usize>) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(value) = cache.get(pattern) {
        return *value;
    }

    let mut count = 0;

    for towel in towels {
        if let Some(rem) = pattern.strip_prefix(towel) {
            count += recurse(rem, towels, cache);
        }
    }
    cache.insert(pattern.to_string(), count);
    count
}

pub fn solve(s: &str) -> usize {
    let (_, (towels, patterns)) = parse(s).unwrap();
    let mut cache = HashMap::new();
    patterns
        .iter()
        .map(|&pattern| recurse(pattern, &towels, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(solve(&data), 16);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 705756472327497);
    }
}
