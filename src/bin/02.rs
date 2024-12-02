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

pub fn safe_1(vals: &&Vec<i32>) -> bool {
    let mut last_sign: i32 = 0;
    vals.windows(2).all(|v| {
        let diff = v[1] - v[0];
        let s = ((diff.signum() == last_sign) || (last_sign == 0)) &&
            diff.abs() >=1 && diff.abs() <= 3;
        last_sign = diff.signum();
        s
    })
}

pub fn safe_2(vals: &&Vec<i32>) -> bool {
    (0..vals.len()).any(|index| {
        let mut v = vals.to_vec();
        v.remove(index);
        safe_1(&&v)
    })
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(get_numbers(input).iter().filter(safe_1).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(get_numbers(input).iter().filter(safe_2).count() as u32)
}
