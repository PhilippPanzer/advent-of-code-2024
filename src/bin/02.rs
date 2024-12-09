
advent_of_code::solution!(2);

fn parse_report(line: &str) -> Vec<i64> {
    line
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect()
}

fn report_legal(report: &Vec<i64>) -> bool {
    let decreasing = report.windows(2).all(|wdw| 1 <= wdw[0] - wdw[1] && wdw[0] - wdw[1] <= 3);
    let increasing = report.windows(2).all(|wdw| 1 <= wdw[1] - wdw[0] && wdw[1] - wdw[0] <= 3);
    decreasing || increasing
}

pub fn part_one(input: &str) -> Option<u32> {
    let num_legal_reports = input
        .lines()
        .map(|line| parse_report(line))
        .filter(|report| report_legal(&report))
        .count();

    Some(num_legal_reports as u32)
}

fn report_legal_under(report: &Vec<i64>, f: fn(i64, i64) -> bool) -> bool {
    let invalid_windows: Vec<usize> = report.windows(2)
        .enumerate()
        .filter(|&(idx, wdw)| {!f(wdw[0], wdw[1])})
        .map(|(idx, _wdw)| idx)
        .collect();
    
    match invalid_windows.len() {
        0 => true,
        1 => {
            if invalid_windows[0] == 0 || invalid_windows[0] == report.len() - 2 {
                true
            } else {
                let i = invalid_windows[0];
                if let [left_adjacent, left, right, right_adjacent] = report[i-1..i+3] {
                    f(left, right_adjacent) || f(left_adjacent, right)
                } else {
                    true
                }
            }
        },
        2 => {
            if invalid_windows[0] != invalid_windows[1] - 1 {
                false
            } else {
                let left_num = report[invalid_windows[0]];
                let right_num = report[invalid_windows[1]+1];
                f(left_num, right_num)
            }
        },
        _ => false
    }
}


fn report_legal_part2(report: &Vec<i64>) -> bool {
    let decreasing = report_legal_under(report, {|x, y| 1 <= x - y && x - y <= 3});
    let increasing = report_legal_under(report, {|x, y| 1 <= y - x && y - x <= 3});
    decreasing || increasing
}

pub fn part_two(input: &str) -> Option<u32> {
    let resut = input
        .lines()
        .map(|line| parse_report(line))
        .filter(|report| report_legal_part2(&report))
        .count();
    
    Some(resut as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
