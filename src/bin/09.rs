advent_of_code::solution!(9);

type Input = Vec<(Option<usize>, usize)>;

fn parse(input: &str) -> Input {
    input.chars().fold((true, 0, vec![]), |(file, f_id, mut fragments), c| {
        if !c.is_digit(10) {
            return (file, f_id, fragments);
        }
        let digit: usize = c.to_string().parse().unwrap();
        if file {
            fragments.push((Some(f_id), digit));
            (false, f_id+1, fragments)
        } else {
            fragments.push((None, digit));
            (true, f_id, fragments)
        }
    }).2
}

fn compute_checksum(fs: &Input) -> u64 {
    fs.iter().fold((0usize, 0usize), {|(sum, block_id), frag|
        if frag.0.is_none() {
            (sum, block_id+frag.1)
        } else {
            let sum = sum + (block_id..block_id+frag.1).map(|b_id| b_id * frag.0.unwrap()).sum::<usize>();
            (sum, block_id+frag.1)
        }
    }).0 as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = parse(input);
    
    let mut i_empty = 0;
    let mut i_file = input.len() - 1;

    while i_empty < i_file {
        if input[i_empty].0.is_some() {
            i_empty += 1;
            continue;
        }
        if input[i_file].0.is_none() {
            i_file -= 1;
            continue;
        }


        input[i_empty].0 = input[i_file].0; // empty fragment gets file's file_id
        if input[i_empty].1 > input[i_file].1 {
            // create new empty fragment
            input.insert(i_empty+1, (None, input[i_empty].1 - input[i_file].1));
            i_file += 1;
            // shorten empty fragment
            input[i_empty].1 = input[i_file].1;
            // remove file fragment
            input[i_file].0 = None;
            i_file -= 1;
        } else if input[i_empty].1 == input[i_file].1 {
            input[i_file].0 = None;
            i_file -= 1;
            i_empty += 1;
        } else {
            input[i_file].1 -= input[i_empty].1;
            i_empty += 1;
        }
    }
    
    Some(compute_checksum(&input))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = parse(input);

    let mut i_file = input.len() - 1;

    while i_file > 0 {
        if input[i_file].0.is_none() {
            i_file -= 1;
            continue;
        }

        for i_frag in 0..i_file {
            if input[i_frag].0.is_none() && input[i_frag].1 >= input[i_file].1 {
                input[i_frag].0 = input[i_file].0;
                if input[i_frag].1 > input[i_file].1 {
                    input.insert(i_frag+1, (None, input[i_frag].1 - input[i_file].1));
                    i_file += 1;
                }
                input[i_frag].1 = input[i_file].1;
                input[i_file].0 = None;
                break;
            }
        }
        i_file -= 1;
    }

    Some(compute_checksum(&input))


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
