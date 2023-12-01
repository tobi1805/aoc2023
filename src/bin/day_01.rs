fn part1(input: &str) -> u32 {
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

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line
                .chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if let d @ Some(_) = c.to_digit(10) {
                        return d;
                    }
                    for (value, num_str) in [
                        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        if line[i..].starts_with(num_str) {
                            return Some(value as u32 + 1);
                        }
                    }
                    None
                })
                .collect();
            numbers.first().unwrap() * 10 + numbers.last().unwrap()
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/day_01");

    let p1 = part1(input);
    println!("Part 1: {p1}");

    let p2 = part2(input);
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

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
    fn test_part_1() {
        let sum = part1(EXAMPLE_INPUT_1);
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_part_2() {
        let sum = part2(EXAMPLE_INPUT_2);
        assert_eq!(sum, 281);
    }
}
