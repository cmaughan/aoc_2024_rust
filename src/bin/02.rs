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
            diff.abs() >= 1 && diff.abs() <= 3;
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

pub fn part_one_clean(input: &str) -> Option<u32> {
    Some(get_numbers(input).iter().filter(safe_1).count() as u32)
}

// Fast version
pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    for l in input.lines() {
        let mut ok = true;
        let mut last: Option<i32> = None;
        let mut last_sign = 0;
        for b in l.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).into_iter() {
            if let Some(a) = last {
                let diff = b - a;
                let sig = diff.signum();
                let diff = diff.abs();
                if diff < 1 || diff > 3
                {
                    ok = false;
                    break;
                }

                if sig != last_sign && last_sign != 0
                {
                    ok = false;
                    break;
                }
                last_sign = sig;
            }
            last = Some(b);
        }
        if ok {
            count = count + 1;
        }
    }
    Some(count)
}

pub fn part_two_clean(input: &str) -> Option<u32> {
    Some(get_numbers(input).iter().filter(safe_2).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    for l in input.lines() {
        let mut ok = true;
        let mut ignore = 0;
        let mut max = 0;

        while ignore <= max
        {
            let mut last: Option<i32> = None;
            let mut last_sign = 0;
            let mut err_lock = 0;
            ok = true;
            for (index, b) in l.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).enumerate() {
                max = max.max(index);
                if index == ignore {
                    continue;
                }
                err_lock = index;
                if let Some(a) = last {
                    let diff = b - a;
                    let sig = diff.signum();
                    let diff = diff.abs();
                    if diff < 1 || diff > 3
                    {
                        ok = false;
                        break;
                    }

                    if sig != last_sign && last_sign != 0
                    {
                        ok = false;
                        break;
                    }
                    last_sign = sig;
                }
                last = Some(b);
            }

            if ok
            {
                break;
            }
            else if ignore > err_lock
            {
                break;
            }

            ignore = ignore + 1;
        }
        if ok {
            count = count + 1;
        }
    }
    Some(count)
}
