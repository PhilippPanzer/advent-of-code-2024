use core::num;
use std::{result, vec};

advent_of_code::solution!(7);

fn parse_line(line: &str) -> (i64, Vec<i64>) {
    let left_right: Vec<&str> = line.split(":").collect();
    let left_num = left_right[0].parse().unwrap();

    let right_str: &str = left_right[1];
    let val_list = right_str.split(" ")
            .filter_map(|s|
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse().unwrap())
                }
            )
            .collect();

    println!("'{}' -> {:?}", line, (left_num, &val_list));

    (left_num, val_list)
}

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input.lines()
        .map(|line| parse_line(line))
        .collect()
}

fn solve(result: i64, numbers: &[i64]) -> Vec<Vec<char>> {


    println!("Solve({}, {:?})", result, numbers);
    
    if result < 0 {
        return vec![];
    }

    if result == 0 {
        if numbers.len() == 0 {
            return vec![vec![]];
        } else {
            return vec![];
        }
    }

    if numbers.len() == 0 {
        return vec![];
    }

    let first_number = numbers[0];
    let remaining_numbers = &numbers[1..];

    let mut ret = vec![];

    if result % first_number == 0 {
        let mut op_seqs = solve(result / first_number, remaining_numbers);
        for mut op_seq in op_seqs.into_iter() {
            op_seq.insert(0, '*');
            ret.push(op_seq);
        }
    }

    let mut op_seqs = solve(result - first_number, remaining_numbers);
    for mut op_seq in op_seqs.into_iter() {
        op_seq.insert(0, '+');
        ret.push(op_seq);
    }

    ret

}

fn solve2(result: i64, numbers: &[i64]) -> Vec<Vec<char>> {


    println!("Solve({}, {:?})", result, numbers);
    
    if result < 0 {
        return vec![];
    }

    if result == 0 {
        if numbers.len() == 0 {
            return vec![vec![]];
        } else {
            return vec![];
        }
    }

    if numbers.len() == 0 {
        return vec![];
    }

    let first_number = numbers[0];
    let remaining_numbers = &numbers[1..];

    let mut ret = vec![];

    if result % first_number == 0 {
        let mut op_seqs = solve2(result / first_number, remaining_numbers);
        for mut op_seq in op_seqs.into_iter() {
            op_seq.insert(0, '*');
            ret.push(op_seq);
        }
    }

    let mut op_seqs = solve2(result - first_number, remaining_numbers);
    for mut op_seq in op_seqs.into_iter() {
        op_seq.insert(0, '+');
        ret.push(op_seq);
    }
    
    println!(" ----- Solve({}, {:?})", result, numbers);
    let result_digits = first_number.to_string().len();
    let mut mul10 = 1;
    for _ in 0..result_digits {
        mul10 *= 10;
    }

    if result % mul10 == first_number {
        println!("{} % {} == {}", result, mul10, first_number);
        let mut op_seqs = solve2(result / mul10, remaining_numbers);
        for mut op_seq in op_seqs.into_iter() {
            op_seq.insert(0, '|');
            ret.push(op_seq);
        }
    } else {
        println!("{} % {} != {}", result, mul10, first_number);
    }

    ret

}


pub fn part_one(input: &str) -> Option<u64> {
    
    let mut equations = parse(input);
    println!("{:?}", equations);
    
    let mut ret = 0;
    
    for (result, numbers) in equations.iter_mut() {
        numbers.reverse();
        if solve(*result, numbers).len() > 0 {
            ret += *result as u64;
        }

    }
    Some(ret)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut equations = parse(input);
    println!("{:?}", equations);
    
    let mut ret = 0;
    
    for (result, numbers) in equations.iter_mut() {
        numbers.reverse();
        if solve2(*result, numbers).len() > 0 {
            ret += *result as u64;
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
