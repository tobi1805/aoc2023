fn part1(input: &str) -> u32 {
    input
        .lines()
        .into_iter()
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
    todo!()
}

fn main() {
    let input = include_str!("../../inputs/day_01");
    let p1 = part1(input);
    println!("Part 1: {p1}");
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
