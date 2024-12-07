use itertools::Itertools;

advent_of_code::solution!(7);

const TestData: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

pub fn part_one(input: &str) -> Option<u32> {
    let vals = input.lines().map(|l| l.split_once(": ").unwrap())
        .map(|(a, b)| {
            (
                a.parse::<i128>().unwrap(),
                b.split_ascii_whitespace()
                    .map(|s| s.parse::<i128>().unwrap())
                    .collect_vec(),
            )
        }).collect::<Vec<(i128, Vec<i128>)>>();

    let mut result: i128 = 0;

    // Walk all the numbers
    for (expected, nums) in vals {
        // Get every combination for the operators (which are for a * b, 1, or 2 for a * b * c
        // The combos will be 0 or 1.
        for mut combo in 0..(2 as u32).pow(nums.len() as u32 - 1) {
            let mut tot: i128 = nums[0] ;
            let mut found = false;
            for index in 1..nums.len() {
                if combo & 1 == 0 {
                    tot = tot + nums[index];
                }
                else {
                    tot = tot * nums[index];
                }
                if tot > expected {
                    break;
                }
                if index == (nums.len() - 1) && tot == expected as i128 {
                    result = result + expected as i128;
                    found = true;
                }
                combo = combo >> 1;
            }
            if found {
                break;
            }
        }
    }
    println!("{result}");
    Some(result as u32)
}
//12615: 804 5 2 1 5 7 7 7 1 3 3 5

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
