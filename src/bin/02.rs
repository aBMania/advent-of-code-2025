use rayon::prelude::*;

advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
pub struct ParseInputError;

fn parse_input(input: &str) -> Result<Vec<(u64, u64)>, ParseInputError> {
    input
        .trim()
        .split(',')
        .map(|min_max| {
            min_max
                .split_once('-')
                .ok_or(ParseInputError)
                .and_then(|(p1, p2)| {
                    Ok((
                        p1.parse().map_err(|_| ParseInputError)?,
                        p2.parse().map_err(|_| ParseInputError)?,
                    ))
                })
        })
        .collect()
}

fn is_invalid(n: u64) -> bool {
    let str = n.to_string();

    if str.len().is_multiple_of(2) {
        str[0..str.len() / 2].eq(&str[str.len() / 2..])
    } else {
        false
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    parse_input(input).ok().map(|input| {
        input
            .par_iter()
            .map(|&(p1, p2)| (p1..=p2).filter(|x| is_invalid(*x)).sum::<u64>())
            .sum::<u64>()
    })
}

fn is_invalid_part_2(n: u64) -> bool {
    let n_str = n.to_string();

    // 1..4
    'outer: for repeating_pattern_length in 1..=(n_str.len() / 2) {
        // 8 is multiple of 1 -> true
        // 8 is multiple of 2 -> true
        // 8 is multiple of 3 -> false
        // 8 is multiple of 4 -> true
        if !n_str.len().is_multiple_of(repeating_pattern_length) {
            continue;
        }

        // pattern to be repeated
        let first_segment = &n_str[0..repeating_pattern_length];

        // skip if pattern is not repeated
        for next_segment_start_index in
            (repeating_pattern_length..n_str.len()).step_by(repeating_pattern_length)
        {
            if &n_str[next_segment_start_index..next_segment_start_index + repeating_pattern_length]
                != first_segment
            {
                continue 'outer;
            }
        }

        // repetition detected
        return true;
    }

    false
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    parse_input(input).ok().map(|input| {
        input
            .par_iter()
            .map(|&(p1, p2)| (p1..=p2).filter(|x| is_invalid_part_2(*x)).sum::<u64>())
            .sum::<u64>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let result = parse_input("11-22,95-115");
        assert_eq!(result, Ok(vec![(11, 22), (95, 115)]));
    }

    #[test]
    fn test_is_invalid() {
        assert!(is_invalid(11));
        assert!(is_invalid(1010));
        assert!(is_invalid(446_446));

        assert!(!is_invalid(12));
        assert!(!is_invalid(1011));
    }

    #[test]
    fn test_is_invalid_part_2() {
        assert!(is_invalid_part_2(11));
        assert!(is_invalid_part_2(1010));
        assert!(is_invalid_part_2(446_446));

        assert!(!is_invalid_part_2(12));
        assert!(!is_invalid_part_2(1011));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1_227_775_554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4_174_379_265));
    }
}
