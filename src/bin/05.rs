use std::collections::{HashMap, HashSet};
use std::cmp::Ordering::*;
advent_of_code::solution!(5);

struct Input {
    rules: HashMap<u32, HashSet<u32>>,
    updates: Vec<Vec<u32>>,
}

fn parse(input: &str) -> Input {
    let (rule_section, update_section) = input.split_once("\n\n").unwrap();

    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    let rule_pairs = rule_section.lines()
        .map(|l| l.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()));

    for (x, y) in rule_pairs
    {
        rules.entry(x).or_insert_with(HashSet::new).insert(y);
    }

    let updates = update_section.lines().
        map(|l|
            l.split(",")
                .map(|v|
                    v.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    Input { rules: rules, updates: updates }
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse(input);

    Some(data.updates.iter()
        .filter(|update| !update.is_sorted_by(|a, b| data.rules[a].contains(b)))
        .map(|update| {
            update[update.len() / 2]
        })
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut data = parse(input);

    Some(data.updates.iter_mut()
        .filter(|update| !update.is_sorted_by(|a, b| data.rules[a].contains(b)))
        .map(|update| {
            update.sort_by(|a, b| if data.rules[a].contains(b) { Less } else { Greater });
            update[update.len() / 2]
        })
        .sum())
}

