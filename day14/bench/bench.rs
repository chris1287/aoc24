use day14::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let data = std::fs::read_to_string("input/input.txt").unwrap();
    part1::solve(&data, 103, 101);
}

#[divan::bench]
fn part2() {
    let data = std::fs::read_to_string("input/input.txt").unwrap();
    part2::solve(&data, 103, 101);
}
