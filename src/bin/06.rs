use std::collections::HashSet;
use std::hash::Hash;
use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn move_pos(pos: (i32, i32), direction: &Dir) -> (i32, i32) {
    match direction
    {
        Dir::Up => (pos.0 - 1, pos.1),
        Dir::Down => (pos.0 + 1, pos.1),
        Dir::Left => (pos.0, pos.1 - 1),
        Dir::Right => (pos.0, pos.1 + 1)
    }
}

fn next_dir(direction: &Dir) -> Dir {
    match direction
    {
        Dir::Up => Dir::Right,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
        Dir::Right => Dir::Down
    }
}

struct Input
{
    guard: (i32, i32),
    width: i32,
    height: i32,
    data: Vec<Vec<u8>>,
}

fn parse(input: &str) -> Input {
    let lines: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let guard = lines.iter().enumerate().find_map(|(y, line)| {
        line.iter().enumerate().find(|(_, &c)| c == b'^')
            .map(|(x, _)| (y as i32, x as i32))
    }).unwrap();

    Input { guard: guard, width: width, height: height, data: lines }
}

fn search(data: &Input) -> HashSet<(i32, i32)> {

    let mut direction = Dir::Up;
    let mut guard = data.guard;
    let mut visits: HashSet<(i32, i32)> = HashSet::new();
    loop {
        let next = move_pos(guard, &direction);

        // Off the map, done
        if next.0 < 0 || next.1 < 0 || next.0 >= data.height || next.1 >= data.width {
            break;
        }

        if data.data[next.0 as usize][next.1 as usize] == b'#' {
            direction = next_dir(&direction);
            continue;
        }

        guard = next;

        visits.insert(guard);
    }
    visits
}
pub fn part_one(input: &str) -> Option<u32> {
    let data = parse(input);
    Some(search(&data).len() as u32 + 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut data = parse(input);

    let paths = search(&data);

    let detect_loop = |data: &Input| -> bool {
        let mut direction = Dir::Up;
        let mut guard = data.guard;
        let mut visits: HashSet<((i32, i32), Dir)> = HashSet::new();
        visits.insert((guard, direction));
        loop {
            let next = move_pos(guard, &direction);

            // Off the map, done
            if next.0 < 0 || next.1 < 0 || next.0 >= data.height || next.1 >= data.width {
                return false;
            }

            if data.data[next.0 as usize][next.1 as usize] == b'#' {
                direction = next_dir(&direction);
                continue;
            }

            guard = next;
            if visits.iter().contains(&(guard, direction))
            {
                return true;
            }
            visits.insert((guard, direction));
        }
    };

    let mut count: u32 = 0;
    for v in paths {
        data.data[v.0 as usize][v.1 as usize] = b'#';
        if detect_loop(&data) {
            count = count + 1;
        }
        data.data[v.0 as usize][v.1 as usize] = b'.';
    }

    Some(count)
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