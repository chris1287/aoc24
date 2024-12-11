use std::collections::HashMap;

fn parse(s: &str) -> HashMap<usize, usize> {
    let s = s.trim();
    let v = s
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut map = HashMap::new();
    for n in v {
        map.entry(n).and_modify(|v| *v += 1).or_insert(1);
    }
    map
}

fn step(cache: &mut HashMap<usize, usize>, n: usize, count: usize) {
    if n == 0 {
        cache.entry(1).and_modify(|v| *v += count).or_insert(count);
    } else {
        let srepr = n.to_string();
        if srepr.len() % 2 == 0 {
            let a = &srepr[..srepr.len() / 2];
            let b = &srepr[srepr.len() / 2..];
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            cache.entry(a).and_modify(|v| *v += count).or_insert(count);
            cache.entry(b).and_modify(|v| *v += count).or_insert(count);
        } else {
            cache
                .entry(n * 2024)
                .and_modify(|v| *v += count)
                .or_insert(count);
        }
    }
}

pub fn solve(s: &str, blinks: usize) -> usize {
    let mut map = parse(s);
    for _blink in 0..blinks {
        let mut cache = HashMap::new();
        map.iter().for_each(|(value, count)| {
            step(&mut cache, *value, *count);
        });
        map = cache;
    }
    map.values().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "125 17";
        assert_eq!(solve(&data, 25), 55312);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data, 75), 207961583799296);
    }
}
