use std::{ops::Index, process::id};

advent_of_code::solution!(5);

fn parse_rule(line: &str) -> (i64, i64) {
    let tmp: Vec<i64> = line
                        .split("|")
                        .map(|s| s.parse().unwrap())
                        .collect();
    (tmp[0], tmp[1])
}

fn parse_update(line: &str) -> Vec<i64> {
    line.split(",")
        .map(|e| e.parse().unwrap())
        .collect()
}

fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let mut updates = vec![];
    let mut rules = vec![];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        } else if line.contains("|") {
            rules.push(parse_rule(line));
        } else {
            updates.push(parse_update(line));
        }
    }

    (rules, updates)
}

fn update_obeys_rule(update: &Vec<i64>, rule: (i64, i64)) -> bool {
    let index1 = update.iter().position(|&e| e == rule.0);
    let index2 = update.iter().position(|&e| e == rule.1);

    !(index1.is_some() && index2.is_some() && index2.unwrap() < index1.unwrap())
}

fn update_okay(update: &Vec<i64>, rules: &Vec<(i64, i64)>) -> bool {
    rules.iter()
        .fold(
            true,
            |acc, &r| acc && update_obeys_rule(&update, r)
        )
}

fn mid_element(update: &Vec<i64>) -> i64 {
    let idx = update.len() / 2;
    update[idx]
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);

    let mut ret = 0;

    for update in updates {
        if update_okay(&update, &rules) {
            ret += mid_element(&update) as u32;
        }
    }

    Some(ret)
}

fn fix_update(rules: &Vec<(i64, i64)>, update: &mut Vec<i64>) -> Vec<i64> {
    while !update_okay(update, rules) {
        for &rule in rules.iter() {
            if update_obeys_rule(update, rule) {
                continue;
            }
            let index1 = update.iter().position(|&e| e == rule.0).unwrap();
            let index2 = update.iter().position(|&e| e == rule.1).unwrap();
            let el1 = update[index1];
            update[index1] = update[index2];
            update[index2] = el1;
        }
    }
    update.clone()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, mut updates) = parse(input);

    let mut ret = 0;

    for update in updates.iter_mut() {
        if !update_okay(&update, &rules) {
            println!("Fixing {:?}", update);
            let fixed_update = fix_update(&rules, update);
            println!("Fixed: {:?}", fixed_update);
            ret += mid_element(&fixed_update) as u32;
            println!("Mid element: {:?}", mid_element(update));
            println!("");
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
