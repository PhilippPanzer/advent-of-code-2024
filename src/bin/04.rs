use std::{char, iter, slice};

advent_of_code::solution!(4);

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn find_all_occurances(field: &Vec<Vec<char>>, c: char) -> Vec<Coord> {
    field.iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter().enumerate().filter(|(_, &t)| t == c).map(|(y, _)| Coord{x: x as i64, y: y as i64}).collect::<Vec<Coord>>()
        })
        .flatten()
        .collect()
}

fn find_in_vicinity(field: &Vec<Vec<char>>, coord: &Coord, c: char) -> Vec<Coord> {
    let mut coords = vec![];
    if coord.x > 0 {
        if coord.y > 0 {
            coords.push(Coord{x: coord.x-1, y: coord.y-1});
        }
        coords.push(Coord{x: coord.x-1, ..*coord});
        if coord.y < field[0].len() as i64 - 1 {
            coords.push(Coord{x: coord.x-1, y: coord.y+1});
        }
    }

    if coord.y > 0 {
        coords.push(Coord{x: coord.x, y: coord.y-1});
    }
    if coord.y < field[0].len() as i64 - 1 {
        coords.push(Coord{x: coord.x, y: coord.y+1});
    }
    
    if coord.x < field.len() as i64 - 1 {
        if coord.y > 0 {
            coords.push(Coord{x: coord.x+1, y: coord.y-1});
        }
        coords.push(Coord{x: coord.x+1, ..*coord});
        if coord.y < field[0].len() as i64 - 1 {
            coords.push(Coord{x: coord.x+1, y: coord.y+1});
        }
    }
    coords.into_iter().filter(|coord| {at(field,coord) == Some(c)}).collect()

}

fn at(field: &Vec<Vec<char>>, coord: &Coord) -> Option<char> {
    if 0 <= coord.x && coord.x < field.len() as i64 && 0 <= coord.y && coord.y < field[0].len() as i64 {
        Some(field[coord.x as usize][coord.y as usize])
    } else {
        None
    }
}

fn get_direction(c1: &Coord, c2: &Coord) -> Coord {
    Coord{x: c2.x - c1.x, y: c2.y - c1.y }
}

fn add(c1: &Coord, c2: &Coord) -> Coord {
    Coord{x: c1.x + c2.x, y: c1.y + c2.y }
}

pub fn part_one(input: &str) -> Option<u32> {
    let field = &parse(input);
    let x_coords = find_all_occurances(field, 'X');
    println!("{:?}", x_coords);
    let mut ret = 0u32;
    for x_coord in x_coords.iter() {
        for m_coord in find_in_vicinity(field, x_coord, 'M').iter() {
            println!("{:?}({:?}){:?}({:?})", x_coord, at(field, &x_coord), m_coord, at(field, &m_coord));
            let direction = get_direction(x_coord, m_coord);
            let a_coord = add(m_coord, &direction);
            if at(field, &a_coord) == Some('A') {
                let s_coord = add(&a_coord, &direction);
                if at(field, &s_coord) == Some('S') {
                    println!("{:?}({:?}){:?}({:?}){:?}({:?}){:?}({:?})", x_coord, at(field, &x_coord), m_coord, at(field, &m_coord), a_coord, at(field, &a_coord), s_coord, at(field, &s_coord));
                    ret += 1;
                }
            }
        }
    }
    Some(ret)
}

pub fn part_two(input: &str) -> Option<u32> {
    let field = &parse(input);

    let a_occurances = find_all_occurances(field, 'A');
    let mut ret = 0;
    for a_occurance in a_occurances.iter() {
        let top_left = at(field, &add(a_occurance, &Coord{x: -1, y: -1}));
        let top_right = at(field, &add(a_occurance, &Coord{x: 1, y: -1}));
        let bottom_left = at(field, &add(a_occurance, &Coord{x: -1, y: 1}));
        let bottom_right = at(field, &add(a_occurance, &Coord{x: 1, y: 1}));

        if ([top_left, bottom_right] == [Some('M'), Some('S')] || [top_left, bottom_right] == [Some('S'), Some('M')]) && ([top_right, bottom_left] == [Some('M'), Some('S')] || [top_right, bottom_left] == [Some('S'), Some('M')]){
            ret += 1;
        }

    }
    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
