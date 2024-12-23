use std::collections::HashMap;

advent_of_code::solution!(8);

type Coord = (i64, i64);

fn parse(input: &str) -> (HashMap<char, Vec<Coord>>, (usize, usize)) {
    let mut ret: HashMap<char, Vec<Coord>> = HashMap::new();
    for (l, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' && c != '#' {
                let cur_coord = (l as i64, col as i64);
                if let Some(coord_list) = ret.get_mut(&c) {
                    coord_list.push(cur_coord);
                } else {
                    ret.insert(c, vec![cur_coord]);
                }
            }
        }
    }
    (ret, (input.lines().count(), input.lines().take(1).collect::<Vec<&str>>()[0].chars().count()))

}

fn add_coords(a: &Coord, b: &Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1)
}

fn sub_coords(a: &Coord, b: &Coord) -> Coord {
    (a.0 - b.0, a.1 - b.1)
}

fn coord_valid(a: &Coord, dims: (usize, usize)) -> bool {
    0 <= a.0 && a.0 < dims.0 as i64 && 0 <= a.1 && a.1 < dims.1 as i64 as i64
}

fn compute_antinodes_for_pair(a: &Coord, b: &Coord, dims: (usize, usize)) -> (Option<Coord>, Option<Coord>) {
    let mut an1 = None;
    let mut an2 = None;

    let delta_ba = sub_coords(a, b);
    let coords = add_coords(a, &delta_ba);
    if coord_valid(&coords, dims) {
        an1 = Some(coords)
    }

    let delta_ab = sub_coords(b, a);
    let coords = add_coords(b, &delta_ab);
    if coord_valid(&coords, dims) {
        an2 = Some(coords)
    }

    (an1, an2)
}

fn compute_antinodes_for_pair_part2(a: &Coord, b: &Coord, dims: (usize, usize)) -> Vec<Coord> {
    let mut ret = vec![];

    let delta_ab = sub_coords(b, a);
    let mut cur_coord = *a;
    while coord_valid(&cur_coord, dims) {
        ret.push(cur_coord);
        cur_coord = add_coords(&cur_coord, &delta_ab);
    }

    cur_coord = sub_coords(&a, &delta_ab);
    while coord_valid(&cur_coord, dims) {
        ret.push(cur_coord);
        cur_coord = sub_coords(&cur_coord, &delta_ab);
    }

    ret
}

fn compute_antinodes(coord_list: &Vec<Coord>, dims: (usize, usize)) -> Vec<Coord> {
    let mut ret = vec![];
    for a in coord_list.iter() {
        for b in coord_list.iter() {
            if a != b {
                let (an1, an2) = compute_antinodes_for_pair(a, b, dims);
                if let Some(coord) = an1 {
                    if !ret.contains(&coord) {
                        ret.push(coord);
                    }
                }
                if let Some(coord) = an2 {
                    if !ret.contains(&coord) {
                        ret.push(coord);
                    }
                }
            }
        }
    }
    ret
}

fn compute_antinodes_part2(coord_list: &Vec<Coord>, dims: (usize, usize)) -> Vec<Coord> {
    let mut ret = vec![];
    for a in coord_list.iter() {
        for b in coord_list.iter() {
            if a != b {
                for &an in  compute_antinodes_for_pair_part2(a, b, dims).iter() {
                    if !ret.contains(&an) {
                        ret.push(an);
                    }
                }
            }
        }
    }
    ret
}

pub fn part_one(input: &str) -> Option<u32> {
    let (antenna_map, dims) = parse(input);
    
    let mut antinodes: Vec<Coord> = vec![];
    for (freq, coord_list) in antenna_map.iter() {
        let anodes = compute_antinodes(coord_list, dims);
        for anode in anodes.iter() {
            if !antinodes.contains(anode) {
                antinodes.push(*anode);
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (antenna_map, dims) = parse(input);
    
    let mut antinodes: Vec<Coord> = vec![];
    for (freq, coord_list) in antenna_map.iter() {
        let anodes = compute_antinodes_part2(coord_list, dims);
        for anode in anodes.iter() {
            if !antinodes.contains(anode) {
                antinodes.push(*anode);
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
