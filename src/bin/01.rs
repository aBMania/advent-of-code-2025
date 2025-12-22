advent_of_code::solution!(1);

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug)]
struct Instruction {
    direction: Direction,
    amount: i32,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction {
            direction: match line.chars().next() {
                Some('L') => Direction::Left,
                Some('R') => Direction::Right,
                _ => panic!("unknown direction"),
            },
            amount: line.chars().skip(1).collect::<String>().parse().unwrap(),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let input = parse_input(input);
    let (_, n) = input
        .iter()
        .fold((50, 0), |(acc, n), Instruction { direction, amount }| {
            let new_acc = match direction {
                Direction::Right => (acc + amount).rem_euclid(100),
                Direction::Left => (acc - amount).rem_euclid(100),
            };
            if new_acc == 0 {
                (new_acc, n + 1)
            } else {
                (new_acc, n)
            }
        });
    Some(n)
}

pub fn part_two(input: &str) -> Option<i32> {
    let input = parse_input(input);
    let (_, n) = input.iter().fold((50i32, 0), apply_part_2_instruction);
    Some(n)
}

fn apply_part_2_instruction(
    (acc, n): (i32, i32),
    Instruction { direction, amount }: &Instruction,
) -> (i32, i32) {
    let mut new_acc = match direction {
        Direction::Right => acc + amount,
        Direction::Left => acc - amount,
    };

    let n_rotation = if new_acc > 0 {
        new_acc / 100
    } else if new_acc < 0 {
        (acc != 0) as i32 - (new_acc) / 100
    } else {
        1 + (new_acc) / 100
    };

    new_acc = new_acc.rem_euclid(100);

    (new_acc.rem_euclid(100), n + n_rotation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_instruction_1() {
        assert_eq!(
            apply_part_2_instruction(
                (50, 0),
                &Instruction {
                    direction: Direction::Left,
                    amount: 100,
                }
            ),
            (50, 1)
        );
    }

    #[test]
    fn test_part_two_instruction_2() {
        assert_eq!(
            apply_part_2_instruction(
                (50, 0),
                &Instruction {
                    direction: Direction::Left,
                    amount: 200,
                }
            ),
            (50, 2)
        );
    }

    #[test]
    fn test_part_two_instruction_3() {
        assert_eq!(
            apply_part_2_instruction(
                (50, 0),
                &Instruction {
                    direction: Direction::Right,
                    amount: 200,
                }
            ),
            (50, 2)
        );
    }

    #[test]
    fn test_part_two_instruction_4() {
        assert_eq!(
            apply_part_2_instruction(
                (0, 0),
                &Instruction {
                    direction: Direction::Right,
                    amount: 100,
                }
            ),
            (0, 1)
        );
    }

    #[test]
    fn test_part_two_instruction_5() {
        assert_eq!(
            apply_part_2_instruction(
                (1, 0),
                &Instruction {
                    direction: Direction::Left,
                    amount: 1,
                }
            ),
            (0, 1)
        );
    }

    #[test]
    fn test_part_two_example_1() {
        assert_eq!(
            apply_part_2_instruction(
                (50, 0),
                &Instruction {
                    direction: Direction::Left,
                    amount: 68,
                }
            ),
            (82, 1)
        );
    }
    #[test]
    fn test_part_two_example_2() {
        assert_eq!(
            apply_part_2_instruction(
                (82, 1),
                &Instruction {
                    direction: Direction::Left,
                    amount: 30,
                }
            ),
            (52, 1)
        );
    }
    #[test]
    fn test_part_two_example_3() {
        assert_eq!(
            apply_part_2_instruction(
                (52, 1),
                &Instruction {
                    direction: Direction::Right,
                    amount: 48,
                }
            ),
            (0, 2)
        );
    }
    #[test]
    fn test_part_two_example_4() {
        assert_eq!(
            apply_part_2_instruction(
                (0, 2),
                &Instruction {
                    direction: Direction::Left,
                    amount: 5,
                }
            ),
            (95, 2)
        );
    }
}
