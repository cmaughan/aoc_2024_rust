advent_of_code::solution!(2);

pub fn get_numbers(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        }).collect::<Vec<Vec<_>>>()
}

#[derive(PartialEq)]
enum Dir { Up, Down, None }

pub fn part_one(input: &str) -> Option<u32> {
    let nums = get_numbers(input);

    let mut safe = 0;
    for l in nums {
        let mut dir: Dir = Dir::None;
        let mut lastDiff : i32 = 0;
        let mut bad = false;
        for win in l.windows(2)
        {
            let diff = win[1] - win[0];
            if (lastDiff != 0) &&
                (lastDiff.signum() != diff.signum()) {
                bad = true;
                break;
            }
            lastDiff = diff;

            if diff.abs() < 1 || diff.abs() > 3 {
                bad = true;
                break;
            }
        }

        if bad == false {
            safe = safe + 1;
        }
    }

    Some(safe)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}
