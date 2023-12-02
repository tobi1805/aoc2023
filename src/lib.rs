#[macro_export]
macro_rules! main {
    ($filename:literal) => {
        static INPUT: &str = include_str!($filename);

        fn main() {
            let now = std::time::Instant::now();
            let p1 = part1(INPUT);
            println!("Time: {:?}", now.elapsed());
            println!("Part 1: {p1}");

            let now = std::time::Instant::now();
            let p2 = part2(INPUT);
            println!("Time: {:?}", now.elapsed());
            println!("Part 2: {p2}");
        }
    };
}
