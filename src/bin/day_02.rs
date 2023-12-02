fn part1(input: &str) -> u32 {
    input.lines().fold(0, |sum, line| {
        let (game, rounds) = line.split_once(':').unwrap();
        for round in rounds.split(';') {
            for cube in round.split(',') {
                let (count, color) = cube.trim().split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();
                if !(match color {
                    "red" => count <= 12,
                    "green" => count <= 13,
                    "blue" => count <= 14,
                    _ => panic!("Invalid color"),
                }) {
                    return sum;
                }
            }
        }
        sum + game.split_once(' ').unwrap().1.parse::<u32>().unwrap()
    })
}

fn part2(input: &str) -> u32 {
    input.lines().fold(0, |sum, line| {
        let (_game, rounds) = line.split_once(':').unwrap();
        let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
        for round in rounds.split(';') {
            for cube in round.split(',') {
                let (count, color) = cube.trim().split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();
                match color {
                    "red" => min_red = min_red.max(count),
                    "green" => min_green = min_green.max(count),
                    "blue" => min_blue = min_blue.max(count),
                    _ => panic!("Invalid color"),
                };
            }
        }
        sum + min_red * min_green * min_blue
    })
}

fn main() {
    let input = include_str!("../../inputs/day_02");

    let now = std::time::Instant::now();
    let p1 = part1(input);
    println!("Time: {:?}", now.elapsed());
    println!("Part 1: {p1}");

    let now = std::time::Instant::now();
    let p2 = part2(input);
    println!("Time: {:?}", now.elapsed());
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green    
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_1), 2286);
    }
}
