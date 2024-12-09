use regex::Regex;
advent_of_code::solution!(3);


pub fn part_one(input: &str) -> Option<u32> {
    println!("{:?}", input);
    let re = Regex::new(
        r"mul\((?P<X>\d{1,3}),(?P<Y>\d{1,3})\)"
    ).unwrap();

    let res = re.captures_iter(input)
        .map(|cap| {
            (cap.name("X").unwrap().as_str().parse().unwrap(),
            cap.name("Y").unwrap().as_str().parse().unwrap())
        }
        )
        .map(|t: (i64, i64)| t.0 * t.1)
        .fold(0, |acc, prod| acc + prod);

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(
        r"mul\((?P<X>\d{1,3}),(?P<Y>\d{1,3})\)|(?<do_marker>do\(\))|(?<dont_marker>don't\(\))"
    ).unwrap();

    let res = re.captures_iter(input)
        .fold((true, 0i64), |(do_mul, sum), cap| {
            if let Some(_) = cap.name("do_marker") {
                (true, sum)
            } else {
                if let Some(_) = cap.name("dont_marker") {
                    (false, sum)
                } else {
                    let mut new_sum = sum;
                    if do_mul {
                        new_sum += cap.name("X").unwrap().as_str().parse::<i64>().unwrap() * cap.name("Y").unwrap().as_str().parse::<i64>().unwrap();
                    }
                    (do_mul, new_sum)
                }
            }
        });

    Some(res.1 as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
