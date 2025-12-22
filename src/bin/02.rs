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

fn is_invalid(n: &u64) -> bool {
    let str = n.to_string();

    if !str.len().is_multiple_of(2) {
        false
    } else {
        str[0..str.len() / 2].eq(&str[str.len() / 2..])
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    parse_input(input).ok().map(|input| {
        input
            .iter()
            .map(|&(p1, p2)| (p1..=p2).filter(is_invalid).sum::<u64>())
            .sum::<u64>()
    })
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
    fn test_is_invvalid() {
        assert_eq!(is_invalid(&11), true);
        assert_eq!(is_invalid(&12), false);
        assert_eq!(is_invalid(&1010), true);
        assert_eq!(is_invalid(&1011), false);
        assert_eq!(is_invalid(&446446), true);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
