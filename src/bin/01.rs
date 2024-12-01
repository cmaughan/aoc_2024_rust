advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn get_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for l in input.lines() {
        let (a, b) = l.split_once("   ").unwrap();
        col1.push(a.parse().unwrap());
        col2.push(b.parse().unwrap());
    }

    (col1, col2)
}

pub fn part_one(input: &str) -> Option<u32> {

    let (mut col1, mut col2) = get_lists(input);
    col1.sort_unstable();
    col2.sort_unstable();
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

