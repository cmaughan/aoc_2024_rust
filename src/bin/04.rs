advent_of_code::solution!(4);

fn get(grid: &Vec<Vec<u8>>, x: i64, y: i64) -> u8
{
    if x < 0 || x >= grid[0].len() as i64 || y < 0 || y >= grid.len() as i64
    {
        return 0;
    }

    grid[y as usize][x as usize]
}

fn search(grid: &Vec<Vec<u8>>, search_term: &[u8], x: i64, y: i64, x_dir: i64, y_dir: i64) -> bool
{
    if search_term.is_empty()
    {
        return true;
    }

    let v = get(grid, x, y);

    if v != search_term[0] {
        return false;
    }

    search(grid, &search_term[1..], x + x_dir, y + y_dir, x_dir, y_dir)
}


pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut count = 0;
    for y in 0..lines.len() as i64 {
        for x in 0..lines[0].len() as i64 {
            if lines[y as usize][x as usize] == b'X' {
                for yy in -1..=1 as i64 {
                    for xx in -1..=1 as i64 {
                        if xx == 0 && yy == 0 {
                            continue;
                        }
                        count += search(&lines, b"MAS", x + xx, y + yy, xx, yy) as u32;
                    }
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut total = 0;
    for y in 0..lines.len() as i64 {
        for x in 0..lines[0].len() as i64 {
            if lines[y as usize][x as usize] == b'A' {
                let mut crosses = 0;
                for offset in [(-1, -1), (1, 1), (-1, 1), (1, -1)] {
                    if get(&lines, x + offset.0, y + offset.1) == b'M' &&
                        get(&lines, x - offset.0, y - offset.1) == b'S' {
                        crosses = crosses + 1;
                    }
                }

                if crosses > 1 {
                    total = total + 1;
                }
            }
        }
    }
    Some(total)
}

