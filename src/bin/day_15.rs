pub fn part1(input: &str) -> usize {
    input.trim_end().split(',').map(util::hash).sum()
}

pub fn part2(input: &str) -> usize {
    let mut map = util::HashMap::new();
    for ele in input.trim_end().split(',') {
        if let Some(pos) = ele.find('=') {
            let (label, number) = ele.split_at(pos);
            let focal_length = number[1..].parse().unwrap();
            map.insert(util::Lens::new(label.to_string(), focal_length))
        } else if let Some(pos) = ele.find('-') {
            let label = &ele[..pos];
            map.remove(label);
        }
    }
    map.focusing_power()
}

aoc2023::main!("../../inputs/day_15");

mod util {
    use std::{borrow::BorrowMut, collections::VecDeque};

    #[inline(always)]
    pub fn hash(s: &str) -> usize {
        s.chars()
            .map(|c| c as u32)
            .fold(0, |current, char_code| ((current + char_code) * 17) % 256) as usize
    }

    pub struct HashMap {
        boxes: [Box; 256],
    }

    impl HashMap {
        pub fn new() -> Self {
            let boxes = [0; 256].map(|_| Box::default());
            Self { boxes }
        }

        pub fn insert(&mut self, lens: Lens) {
            self.box_mut(&lens.label).insert(lens);
        }

        pub fn remove(&mut self, label: &str) -> Option<Lens> {
            self.box_mut(&label).remove(label)
        }

        pub fn focusing_power(&self) -> usize {
            self.boxes
                .iter()
                .enumerate()
                .map(|(i, b)| b.focusing_power() * (i + 1))
                .sum()
        }

        fn box_mut(&mut self, label: &str) -> &mut Box {
            let index = hash(label);
            self.boxes[index].borrow_mut()
        }
    }

    #[derive(Debug, Default, Clone)]
    struct Box {
        lenses: VecDeque<Lens>,
    }

    impl Box {
        fn insert(&mut self, lens: Lens) {
            if let Some(position) = self.lenses.iter().position(|l| l.label == lens.label) {
                self.lenses[position] = lens;
            } else {
                self.lenses.push_back(lens);
            }
        }

        fn remove(&mut self, label: &str) -> Option<Lens> {
            self.lenses
                .iter()
                .position(|l| l.label == label)
                .and_then(|i| self.lenses.remove(i))
        }

        fn focusing_power(&self) -> usize {
            self.lenses
                .iter()
                .enumerate()
                .map(|(i, l)| (i + 1) * l.focal_length as usize)
                .sum()
        }
    }

    #[derive(Debug, Clone)]
    pub struct Lens {
        label: String,
        focal_length: u32,
    }

    impl Lens {
        pub fn new(label: String, focal_length: u32) -> Self {
            Lens {
                label,
                focal_length,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(util::hash("HASH"), 52);
    }

    const EXAMPLE_INPUT_1: &str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 1320);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 145);
    }
}
