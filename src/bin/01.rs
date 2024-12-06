advent_of_code::solution!(1);

fn parse_line(line: &str) -> (i64, i64) {
    let v: Vec<i64> = line.split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| 
            match s.parse() {
                Ok(x) => x,
                Err(_) => {panic!("Failet to parse '{}'", s);}
            }
        )
        .collect();
    (v[0], v[1])
}

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .map(|l| parse_line(l))
        .fold(
            (vec![], vec![]), 
            |mut acc: (Vec<i64>, Vec<i64>), x: (i64, i64)| {
                acc.0.push(x.0); 
                acc.1.push(x.1); 
                acc
            }
        )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list1, mut list2): (Vec<i64>, Vec<i64>) = parse(input);

    list1.sort();
    list2.sort();

    let sum = list1.iter()
        .zip(list2.iter())
        .fold(0, |acc, (x1, x2)| acc + (x1 - x2).abs());

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut list1, list2): (Vec<i64>, Vec<i64>) = parse(input);

    list1.sort();
    
    let ret = list1.iter()
                .fold((0, None, 0), |(sum, last_elem, occurances), &x| {
                        if Some(x) == last_elem {
                            (sum + occurances * x, last_elem, occurances)
                        } else {
                            let occ = list2.iter().filter(|&&y| y == x).count() as i64;
                            (sum + occ * x, Some(x), occ)
                        }
                    }
                ).0;

    Some(ret as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
