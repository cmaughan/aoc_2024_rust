use std::collections::{HashMap, HashSet};

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

    let mut count = 0;
    // a,b : a must be before b.
    for l in data.updates {
        let mut invalid = false;
        for i in 0..l.len() {
            let num = l[i];
            let rule = data.rules.get(&num);

            // If our number has a rule
            if let Some(r) = rule {
                for j in (0..i).rev() {
                    let prev_num = l[j];
                    if r.contains(&prev_num)
                    {
                       invalid = true;
                    }
                }
            }
            if invalid {
                break;
            }
        }
        if !invalid {
            count = count + l[l.len() / 2];
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse(input);

    let mut count = 0;
    // a,b : a must be before b.
    for mut l in data.updates {
        let mut add_it = false;
        loop {
            let mut invalid = false;
            for i in 0..l.len() {
                let num = l[i];
                let rule = data.rules.get(&num);

                // If our number has a rule
                if let Some(r) = rule {
                    let mut min_swap = i;
                    for j in (0..i).rev() {
                        let prev_num = l[j];
                        if r.contains(&prev_num)
                        {
                            min_swap = j;
                            add_it = true;
                            break;
                        }
                    }

                    if min_swap != i {
                        let removed = l.remove(i);
                        l.insert(min_swap, removed);
                        add_it = true;
                        invalid = true;
                    }
                }
                if invalid {
                    break;
                }
            }
            if !invalid {
                if add_it {
                    count += l[l.len() / 2];
                }
                break;
            }
        }
    }

    Some(count)
}

