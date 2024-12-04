advent_of_code::solution!(3);

fn extract_num(line: &[u8]) -> u32 {
    let mut total = 0;
    let mut mul = 1;
    for num in line.iter().rev() {
        total += (num - b'0') as u32 * mul;
        mul *= 10;
    }
    total
}
pub fn consume_digits(vals: &[u8]) -> usize {
    let mut count = 0;
    while vals[count].is_ascii_digit() {
        count = count + 1;
    }
    count
}

pub fn tokenize(input: &str, skip: bool) -> u32 {
    let tokens = input.as_bytes();
    let mut sum = 0;
    let mut ignore = false;

    let mut i = 0;
    while i < (tokens.len() - 4)
    {
        match &tokens[i..i + 4]
        {
            b"don'" => {
                if skip {
                    ignore = true;
                }
                i = i + 4;
            }
            b"do()" => {
                ignore = false;
                i = i + 4;
            }
            b"mul(" => {
                if ignore
                {
                    i = i + 1;
                    continue;
                }

                let start_digits = i + 4;
                let first_end = start_digits + consume_digits(&tokens[start_digits..]);
                if first_end == start_digits || tokens[first_end] != b',' {
                    i = start_digits;
                    continue;
                }
                let second_start = first_end + 1;
                let second_end = second_start + consume_digits(&tokens[second_start..]);
                if second_end == second_start || tokens[second_end] != b')' {
                    i = second_start;
                    continue;
                }

                sum += extract_num(&tokens[start_digits..first_end]) * extract_num(&tokens[second_start..second_end]);
                i = second_end;
            }
            _ => {
                i = i + 1;
            }
        }
    }
    sum as u32
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(tokenize(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(tokenize(input, true))
}

