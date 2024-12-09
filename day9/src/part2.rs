use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Type {
    Full(usize),
    Free,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub t: Type,
    pub s: u32,
    pub checked: bool,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (pre, post) = match self.checked {
            true => ("\x1b[91m", "\x1b[0m"),
            false => ("", ""),
        };
        match self.t {
            Type::Full(id) => {
                (0..self.s).for_each(|_| write!(f, "{}{}{}", pre, id, post).expect("write error"));
            }
            Type::Free => {
                (0..self.s).for_each(|_| write!(f, ".").expect("write error"));
            }
        }
        Ok(())
    }
}

fn parse(s: &str) -> Vec<Block> {
    let mut v = Vec::new();
    let mut id = 0;
    for (pos, c) in s.chars().enumerate() {
        if let Some(n) = c.to_digit(10) {
            if pos % 2 == 0 {
                v.push(Block {
                    t: Type::Full(id),
                    s: n,
                    checked: false,
                });
                id += 1;
            } else {
                v.push(Block {
                    t: Type::Free,
                    s: n,
                    checked: false,
                });
            }
        }
    }
    v
}

fn squash(v: &mut Vec<Block>) {
    let mut again = true;
    while again {
        again = false;
        for i in 0..v.len() - 1 {
            if matches!(v[i].t, Type::Free) && matches!(v[i + 1].t, Type::Free) {
                v[i].s += v[i + 1].s;
                v.remove(i + 1);
                again = true;
                break;
            }
        }
    }
}

fn reorder(v: &mut Vec<Block>) -> bool {
    for i in (0..v.len()).rev() {
        if v[i].checked {
            continue;
        }
        match v[i].t {
            Type::Free => {}
            Type::Full(_id) => {
                for j in 0..i {
                    match v[j].t {
                        Type::Full(_) => {}
                        Type::Free => {
                            if v[j].s == v[i].s {
                                v[i].checked = true;
                                v.swap(i, j);
                                break;
                            } else if v[j].s > v[i].s {
                                v[i].checked = true;
                                v[j].s -= v[i].s;
                                v.push(Block {
                                    t: Type::Free,
                                    s: v[i].s,
                                    checked: false,
                                });
                                let b = v.swap_remove(i);
                                v.insert(j, b);
                                squash(v);
                                return false;
                            } else {
                                v[i].checked = true;
                            }
                        }
                    }
                }
            }
        }
    }
    squash(v);

    true
}

fn get_block_types(v: &[Block]) -> Vec<Type> {
    let mut res = Vec::new();
    v.iter().for_each(|b| {
        for _ in 0..b.s {
            res.push(b.t.clone());
        }
    });
    res
}

pub fn solve(s: &str) -> usize {
    let mut v = parse(s);
    loop {
        if reorder(&mut v) {
            break;
        }
    }
    let v = get_block_types(&v);
    v.into_iter()
        .enumerate()
        .filter(|(_, b)| matches!(b, Type::Full(_)))
        .fold(0, |acc, (pos, b)| match b {
            Type::Full(id) => acc + pos * id,
            _ => acc,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "2333133121414131402";
        assert_eq!(solve(data), 2858);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 6360363199987);
    }
}
