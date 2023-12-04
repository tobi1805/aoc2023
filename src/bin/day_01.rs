pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first, last) = (
                line.chars().find_map(|e| e.to_digit(10)).unwrap(),
                line.chars().rev().find_map(|e| e.to_digit(10)).unwrap(),
            );
            first * 10 + last
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            #[inline(always)]
            fn find_digit<T: Iterator<Item = (usize, char)>>(
                chars: T,
                predicate: impl Fn(usize, &str) -> bool,
            ) -> Option<u32> {
                for (i, c) in chars {
                    if let d @ Some(_) = c.to_digit(10) {
                        return d;
                    }
                    for (value, num_str) in [
                        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        if predicate(i, num_str) {
                            return Some(value as u32 + 1);
                        }
                    }
                }
                None
            }

            let (first, last) = (
                find_digit(line.char_indices(), |i, s| line[i..].starts_with(s)).unwrap(),
                find_digit(line.char_indices().rev(), |i, s| line[..=i].ends_with(s)).unwrap(),
            );

            first * 10 + last
        })
        .sum()
}

aoc2023::main!("../../inputs/day_01");

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 142);
    }

    const EXAMPLE_INPUT_2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 281);
    }
}
