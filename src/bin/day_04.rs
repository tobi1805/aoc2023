use std::collections::VecDeque;

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.split_once(':')
                .and_then(|(_, r)| {
                    r.split_once(" |")
                        .map(|(a, b)| {
                            (
                                a.as_bytes().chunks_exact(3),
                                b.as_bytes().chunks_exact(3).collect::<Vec<&[u8]>>(),
                            )
                        })
                        .map(|(a, b)| {
                            let count = a.filter(|c| b.contains(c)).count();
                            if count > 0 {
                                1 << (count - 1)
                            } else {
                                0
                            }
                        })
                })
                .unwrap()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut additional_cards = VecDeque::new();
    input
        .lines()
        .map(|l| {
            l.split_once(':')
                .and_then(|(_, r)| {
                    r.split_once(" |")
                        .map(|(a, b)| {
                            (
                                a.as_bytes().chunks_exact(3),
                                b.as_bytes().chunks_exact(3).collect::<Vec<&[u8]>>(),
                            )
                        })
                        .map(|(a, b)| a.filter(|c| b.contains(c)).count())
                })
                .unwrap()
        })
        .map(|n| {
            let cards: u32 = additional_cards.pop_front().unwrap_or_default() + 1;
            for i in 0..n {
                additional_cards
                    .get_mut(i)
                    .map(|v| *v += cards)
                    .unwrap_or_else(|| additional_cards.push_back(cards));
            }
            cards
        })
        .sum()
}

aoc2023::main!("../../inputs/day_04");

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11    
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 13);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 30);
    }
}
