advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let ret = re.captures_iter(input).map(|cap| {
        cap[1].parse::<i64>().unwrap() * cap[2].parse::<i64>().unwrap()
    }).sum::<i64>();

    Some(ret as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();

    let mut ret = 0;
    let mut ignore = false;
    for cap in re.captures_iter(input) {
        if let Some(thing) = cap.get(1) {
            if thing.as_str().starts_with("do(") {
                ignore = false;
            }
            else if thing.as_str().starts_with("don't") {
                ignore = true;
            }
            else {
                if ignore
                {
                    continue;
                }
                if let Some(x) = cap.get(2) {
                    if let Some(y) = cap.get(3) {
                        ret += x.as_str().parse::<i64>().unwrap() * y.as_str().parse::<i64>().unwrap();
                    }
                }
            }
        }
    }

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
