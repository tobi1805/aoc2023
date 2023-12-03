use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> u32 {
    let mut map = HashMap::new();
    let mut result = 0;
    'outer: for (value, start, length) in
        input.lines().enumerate().fold(Vec::new(), |mut v, (y, l)| {
            let mut current_number = None;
            v.extend(l.chars().enumerate().filter_map(|(x, c)| {
                let p = Point2d(x as isize, y as isize);

                if let Some(d) = c.to_digit(10) {
                    current_number = current_number.map_or(Some(d), |n| Some(n * 10 + d));
                    return None;
                } else if c != '.' {
                    map.insert(p.clone(), Content::Symbol);
                }
                current_number.take().map(|n| {
                    let length = n.ilog(10);
                    (n, Point2d(p.0 - length as isize - 1, p.1), length as usize)
                })
            }));
            // Insert if the line ends with a number
            if let Some(n) = current_number {
                let length = n.ilog(10);
                v.push((
                    n,
                    Point2d((l.len() - length as usize - 1) as isize, y as isize),
                    length as usize,
                ));
            }
            v
        })
    {
        #[inline(always)]
        fn check(p: &Point2d, map: &HashMap<Point2d, Content>) -> bool {
            map.get(p).is_some_and(|c| c == &Content::Symbol)
        }

        // Check all upper and lower + diagonal points
        for x in start.0 - 1..=start.0 + length as isize + 1 {
            if check(&Point2d(x, start.1 - 1), &map) || check(&Point2d(x, start.1 + 1), &map) {
                result += value;
                continue 'outer;
            }
        }
        // Check all left and right points
        if check(&Point2d(start.0 - 1, start.1), &map)
            || check(&Point2d(start.0 + length as isize + 1, start.1), &map)
        {
            result += value;
            continue;
        }
    }
    result
}

fn part2(input: &str) -> u32 {
    let mut map = HashMap::new();
    let mut result = 0;
    for point in input.lines().enumerate().fold(Vec::new(), |mut v, (y, l)| {
        let mut current_number_start = None;
        let mut current_number = None;
        v.extend(l.chars().enumerate().filter_map(|(x, c)| {
            let p = Point2d(x as isize, y as isize);

            if let Some(d) = c.to_digit(10) {
                let start = current_number_start.get_or_insert(p.clone());
                current_number = current_number.map_or(Some(d), |n| Some(n * 10 + d));
                map.insert(p.clone(), Content::Number(start.clone()));
                return None;
            }
            // Insert NumberStart if the number ends
            if let Some(n) = current_number.take() {
                let length = n.ilog(10) + 1;
                current_number_start = None;
                map.insert(Point2d(p.0 - length as isize, p.1), Content::NumberStart(n));
            };
            (c == '*').then_some(p.clone())
        }));
        // Insert NumberStart if the line ends with a number
        if let Some(n) = current_number {
            let length = n.ilog(10) + 1;
            map.insert(
                Point2d((l.len() - length as usize) as isize, y as isize),
                Content::NumberStart(n),
            );
        }
        v
    }) {
        #[inline(always)]
        fn check(
            p: &Point2d,
            map: &HashMap<Point2d, Content>,
            checked: &mut HashSet<Point2d>,
        ) -> Option<u32> {
            if checked.contains(p) {
                return None;
            }
            if let Some(Content::NumberStart(n)) = map.get(p) {
                checked.insert(p.clone());
                return Some(*n);
            } else if let Some(Content::Number(start)) = map.get(p) {
                return check(start, map, checked);
            }
            None
        }

        let mut checked = HashSet::new();
        let mut count = 0;
        let mut ratio = 1;

        // Check all upper and lower + diagonal points
        for x in point.0 - 1..=point.0 + 1 {
            if let Some(value) = check(&Point2d(x, point.1 - 1), &map, &mut checked) {
                count += 1;
                ratio *= value;
            }
            if let Some(value) = check(&Point2d(x, point.1 + 1), &map, &mut checked) {
                count += 1;
                ratio *= value;
            }
        }
        // Check left and right points
        if let Some(value) = check(&Point2d(point.0 - 1, point.1), &map, &mut checked) {
            count += 1;
            ratio *= value;
        }
        if let Some(value) = check(&Point2d(point.0 + 1, point.1), &map, &mut checked) {
            count += 1;
            ratio *= value;
        }

        if count == 2 {
            result += ratio;
        }
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point2d(isize, isize);

#[derive(Debug, PartialEq, Eq)]
enum Content {
    Symbol,
    Number(Point2d),
    NumberStart(u32),
}

#[derive(Debug, PartialEq, Eq)]
struct Gear {
    adj: u32,
    value: u32,
}

aoc2023::main!("../../inputs/day_03");

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 4361);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 467835);
    }
}
