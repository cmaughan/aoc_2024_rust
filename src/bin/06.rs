use std::collections::HashSet;

advent_of_code::solution!(6);

enum Dir {
    Up,
    Down,
    Left,
    Right
}

fn move_pos(pos: (i32, i32), direction : &Dir) -> (i32, i32) {
    match direction
    {
        Dir::Up => (pos.0 - 1, pos.1),
        Dir::Down => (pos.0 + 1, pos.1),
        Dir::Left => (pos.0, pos.1 - 1),
        Dir::Right => (pos.0, pos.1 + 1)
    }
}

fn next_dir(direction : &Dir) -> Dir {
    match direction
    {
        Dir::Up => Dir::Right,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
        Dir::Right => Dir::Down
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut guard = lines.iter().enumerate().find_map(|(y, line)| {
        line.iter().enumerate().find(|(_, &c)| c == b'^')
            .map(|(x, _)| (y as i32, x as i32))
    }).unwrap();

    let mut direction = Dir::Up;
    let mut visits: HashSet<(i32, i32)> = HashSet::new();
    visits.insert(guard);
    loop {
        let next = move_pos(guard, &direction);

        // Off the map, done
        if next.0 < 0 || next.1 < 0 || next.0 >= height || next.1 >= width {
            break;
        }

        if lines[next.0 as usize][next.1 as usize] == b'#' {
            direction = next_dir(&direction);
            continue;
        }

        guard = next;

        visits.insert(guard);
    }
    Some(visits.len() as u32)
}

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
