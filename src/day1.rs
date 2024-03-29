use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_gen(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| i64::from_str_radix(l, 10).unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    input
        .iter()
        .fold(0, |sum, m| {
            sum + m / 3 - 2
        })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    input
        .iter()
        .fold(0, |sum, m| {
            let mut fuel = m / 3 - 2;
            let mut total = 0;
            
            sum + loop {
                if fuel > 0 {
                    total += fuel;
                    fuel = fuel / 3 - 2;
                } else {
                    break total;
                }
            }
        })
}
