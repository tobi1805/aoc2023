#[macro_export]
macro_rules! main {
    ($filename:literal) => {
        static INPUT: &str = include_str!($filename);

        fn main() {
            let p1 = part1(INPUT);
            println!("Part 1: {p1}");

            let p2 = part2(INPUT);
            println!("Part 2: {p2}");
        }
    };
}
