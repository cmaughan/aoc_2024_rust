use std::collections::{HashMap, HashSet};
use std::cmp::Ordering::*;
advent_of_code::solution!(5);

struct Input {
    rules: HashMap<u32, HashSet<u32>>,
    updates: Vec<Vec<u32>>
}

fn parse(input: &str) -> Input {
    let rules_updates = input.split("\n\n").collect::<Vec<_>>();
    let rs = rules_updates[0].lines().map(|l| {
        let mut nums = l.split("|");
        (nums.next().unwrap().parse::<u32>().unwrap(), nums.next().unwrap().parse::<u32>().unwrap())
    }).collect::<Vec<(u32, u32)>>();

    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for r in &rs {
        rules.entry(r.0).or_insert(HashSet::new()).insert(r.1);
    }

    let updates = rules_updates[1].lines().
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

