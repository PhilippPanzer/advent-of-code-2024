use std::cell::Cell;

use advent_of_code::template::ANSI_BOLD;

advent_of_code::solution!(6);


fn parse_simple(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().map(|c| {if vec!['.', '#', 'v', '<', '>', '^'].contains(&c) {c} else {
        panic!()
    }}).collect()).collect()
}

fn find_guard(field: &Vec<Vec<char>>) -> (usize, usize) {
    for (x, line) in field.iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            match c {
                '<' | '>' | '^' | 'v' => return (x, y),
                _ => continue,
            }
        }
    }
    panic!()
}

fn turn(dir: char) -> char {
    match dir {
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        '^' => '>',
        _ => panic!()
    }
}

fn next_coords(map: &Vec<Vec<char>>, coords: (usize, usize), dir: char) -> Option<(usize, usize)> {
    let mut next_x = coords.0 as i64;
    let mut next_y = coords.1 as i64;

    match dir {
        '<' => next_y -= 1,
        '>' => next_y += 1,
        '^' => next_x -= 1,
        'v' => next_x += 1,
        _ => panic!()
    }

    if !(0 <= next_x && (next_x as usize) < map.len() && 0 <= next_y && (next_y as usize) < map[0].len()) {
        return None;
    }

    Some((next_x as usize, next_y as usize))
}

fn next_pos(map: &mut Vec<Vec<char>>, guard_pos: (usize, usize), dir: char) -> Option<(usize, usize, char)> {
    let mut dir = dir;
    loop {
        let Some((next_x, next_y)) = next_coords(map, guard_pos, dir) else {
            return None;
        };
        if map[next_x][next_y] == '#' {
            dir = turn(dir);
        } else {
            return Some((next_x, next_y, dir));
        }
    }
}

fn count_visited(map: &Vec<Vec<char>>) -> u32 {
    map.iter().flatten().filter(|&&c| c == 'X').count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    
    let mut map = parse_simple(input);
    let mut guard_coords = find_guard(&map);
    let mut dir = map[guard_coords.0][guard_coords.1];

    loop {
        let Some(coords)= next_pos(&mut map, guard_coords, dir) else {
            map[guard_coords.0][guard_coords.1] = 'X';
            break;
        };

        map[guard_coords.0][guard_coords.1] = 'X';
        guard_coords = (coords.0, coords.1);
        dir = coords.2;
    }

    Some(count_visited(&map))
}

fn has_loop(map: &mut Vec<Vec<char>>, multi_map: &mut Vec<Vec<Vec<char>>>, guard: (usize, usize), dir: char) -> bool {

    let mut cur_cuard = guard;
    let mut cur_dir = dir;

    loop {
        let Some((next_x, next_y, next_dir)) = next_pos(map,cur_cuard, cur_dir) else {
            // Rached boundary
            return false;
        };
        if multi_map[next_x][next_y].contains(&next_dir) {
            // The field I want to go to and the direction have been done before -> loop
            return true;
        }

        map[cur_cuard.0][cur_cuard.1] = cur_dir;

        if !multi_map[cur_cuard.0][cur_cuard.1].contains(&cur_dir) {
            multi_map[cur_cuard.0][cur_cuard.1].push(cur_dir);
        }
        cur_cuard = (next_x, next_y);
        cur_dir = next_dir;
    }
}

fn add(map: &Vec<Vec<char>>, coords: (usize, usize), dir: char, mut n: usize) -> Option<(usize, usize)> {
    let (mut next_x, mut next_y) = coords;
    while n > 0 {
        let next_coords = next_coords(map, coords, dir);
        if next_coords.is_none() {
            return None;
        }
        (next_x, next_y) = next_coords.unwrap();
        n-=1;
    }
    Some((next_x, next_y))
}

pub fn part_two(input: &str) -> Option<u32> {
    
    let mut map = parse_simple(input);
    let mut multi_map: Vec<Vec<Vec<char>>> = map.iter().map(|vecochars| vecochars.iter().map(|&c| vec![c]).collect()).collect();

    let mut guard_coords = find_guard(&map);
    let orig_guard_coords = guard_coords;
    let mut dir = map[guard_coords.0][guard_coords.1];

    let mut results = vec![];

    let mut ret = 0;
    loop {
        let Some((next_x, next_y, next_dir)) = next_pos(&mut map, guard_coords, dir) else {
            break;
        };

        if (next_x, next_y) != orig_guard_coords && map[next_x][next_y] == '.' {
            // I could take a step. Lets enforce a turn
            let mut multi_map_clone = multi_map.clone();
            let mut map_clone = map.clone();
            map_clone[next_x][next_y] = '#';
            if has_loop(&mut map_clone, &mut multi_map_clone, guard_coords, dir) {
                if !results.contains(&(next_x, next_y)) {
                    results.push((next_x, next_y));
                }
            }
        }

        map[guard_coords.0][guard_coords.1] = next_dir; // or next_dir?
        if !multi_map[guard_coords.0][guard_coords.1].contains(&dir) {
            multi_map[guard_coords.0][guard_coords.1].push(dir);
        }
        if !multi_map[guard_coords.0][guard_coords.1].contains(&next_dir) {
            multi_map[guard_coords.0][guard_coords.1].push(next_dir);
        }
        guard_coords = (next_x, next_y);
        dir = next_dir;
    }

    Some(results.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
