advent_of_code::solution!(1);

use std::collections::HashMap;
use itertools::Itertools;

pub fn get_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let col1 = input.lines().map(|l| {
        l.split_whitespace().nth(0).map(|l| l.parse::<i32>().unwrap()).unwrap()
    }).sorted().collect::<Vec<_>>();

    let col2 = input.lines().map(|l| {
        l.split_whitespace().nth(1).map(|l| l.parse::<i32>().unwrap()).unwrap()
    }).sorted().collect::<Vec<_>>();

    (col1, col2)
}

pub fn part_one(input: &str) -> Option<u32> {

    let (col1, col2) = get_lists(input);
    let ret = col1.iter().zip(col2.iter()).map(|(&a, &b)| {
        (a - b).abs() as u32
    }).sum();

    Some(ret)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (col1, col2) = get_lists(input);

    let mut counts = HashMap::new();
    for v in col2 {
        *counts.entry(v).or_insert(0) += 1;
    }

    let ret = col1.iter().map(|a| {
        counts.get(a).unwrap_or(&0) * a
    }).sum::<i32>();

    Some(ret as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
