use std::collections::{HashSet, VecDeque};

use util::Beam;

pub fn part1(input: &str) -> usize {
    let map: Vec<Vec<Option<util::Element>>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.try_into().ok())
                .collect::<Vec<Option<util::Element>>>()
        })
        .collect();

    let mut beams = VecDeque::new();
    beams.push_back(util::Beam::default());

    let mut energized = HashSet::new();
    let mut processed_beams = HashSet::new();

    let width = unsafe { map.get_unchecked(0).len() };
    let height = map.len();

    let lookup = |point: util::Point| {
        if let Some(v) = map.get(point.1).and_then(|row| row.get(point.0)) {
            return v;
        }
        &None
    };

    while let Some(beam) = beams.pop_front() {
        if beam.position.0 >= width || beam.position.1 >= height {
            continue;
        }
        if !processed_beams.insert(beam.clone()) {
            continue;
        }
        energized.insert(beam.position);
        let next = beam.move_forward(&lookup);

        for beam in next {
            beams.push_back(beam)
        }
    }
    energized.len()
}

pub fn part2(input: &str) -> usize {
    let map: Vec<Vec<Option<util::Element>>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.try_into().ok())
                .collect::<Vec<Option<util::Element>>>()
        })
        .collect();

    let width = unsafe { map.get_unchecked(0).len() };
    let height = map.len();

    get_start_beams(width, height)
        .map(|start| {
            let mut beams = VecDeque::new();
            beams.push_back(start);

            let mut energized = HashSet::new();
            let mut processed_beams = HashSet::new();

            let lookup = |point: util::Point| {
                if let Some(v) = map.get(point.1).and_then(|row| row.get(point.0)) {
                    return v;
                }
                &None
            };

            while let Some(beam) = beams.pop_front() {
                if beam.position.0 >= width || beam.position.1 >= height {
                    continue;
                }
                if !processed_beams.insert(beam.clone()) {
                    continue;
                }
                energized.insert(beam.position);
                let next = beam.move_forward(&lookup);

                for beam in next {
                    beams.push_back(beam)
                }
            }
            energized.len()
        })
        .max()
        .unwrap()
}

fn get_start_beams(width: usize, height: usize) -> impl Iterator<Item = Beam> {
    (0..width)
        .flat_map(move |x| {
            [
                util::Beam {
                    position: util::Point(x, 0),
                    direction: util::Direction::South,
                },
                util::Beam {
                    position: util::Point(x, height - 1),
                    direction: util::Direction::North,
                },
            ]
        })
        .chain((0..height).flat_map(move |y| {
            [
                util::Beam {
                    position: util::Point(0, y),
                    direction: util::Direction::East,
                },
                util::Beam {
                    position: util::Point(width - 1, y),
                    direction: util::Direction::West,
                },
            ]
        }))
}

mod util {
    #[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct Point(pub usize, pub usize);

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Beam {
        pub position: Point,
        pub direction: Direction,
    }

    impl Default for Beam {
        fn default() -> Self {
            Self {
                position: Default::default(),
                direction: Direction::East,
            }
        }
    }

    impl Beam {
        pub fn move_forward<'a, F>(mut self, lookup: &'a F) -> Vec<Self>
        where
            F: Fn(Point) -> &'a Option<Element>,
        {
            let mut beams = Vec::new();

            match lookup(self.position) {
                Some(x) => match x {
                    Element::MirrorF => {
                        self.direction = match self.direction {
                            Direction::North => Direction::East,
                            Direction::South => Direction::West,
                            Direction::West => Direction::South,
                            Direction::East => Direction::North,
                        };
                        if let Ok(b) = self.move_self() {
                            beams.push(b);
                        }
                    }
                    Element::MirrorB => {
                        self.direction = match self.direction {
                            Direction::North => Direction::West,
                            Direction::South => Direction::East,
                            Direction::West => Direction::North,
                            Direction::East => Direction::South,
                        };
                        if let Ok(b) = self.move_self() {
                            beams.push(b);
                        }
                    }
                    Element::SplitH => match self.direction {
                        Direction::South | Direction::North => {
                            beams.push(Beam {
                                position: self.position,
                                direction: Direction::West,
                            });
                            beams.push(Beam {
                                position: self.position,
                                direction: Direction::East,
                            })
                        }
                        _ => {
                            if let Ok(b) = self.move_self() {
                                beams.push(b);
                            }
                        }
                    },
                    Element::SplitV => match self.direction {
                        Direction::West | Direction::East => {
                            beams.push(Beam {
                                position: self.position,
                                direction: Direction::North,
                            });
                            beams.push(Beam {
                                position: self.position,
                                direction: Direction::South,
                            })
                        }
                        _ => {
                            if let Ok(b) = self.move_self() {
                                beams.push(b);
                            }
                        }
                    },
                },
                None => {
                    if let Ok(b) = self.move_self() {
                        beams.push(b);
                    }
                }
            }
            beams
        }

        fn move_self(mut self) -> Result<Self, ()> {
            self.direction
                .move_point(self.position)
                .and_then(|p| {
                    self.position = p;
                    Some(self)
                })
                .ok_or(())
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Direction {
        North,
        South,
        West,
        East,
    }

    impl Direction {
        fn move_point(&self, p: Point) -> Option<Point> {
            let mut x = p.0 as isize;
            let mut y = p.1 as isize;
            match self {
                Self::North => y -= 1,
                Self::South => y += 1,
                Self::West => x -= 1,
                Self::East => x += 1,
            };
            (x >= 0 && y >= 0).then_some(Point(x as usize, y as usize))
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Element {
        MirrorF,
        MirrorB,
        SplitH,
        SplitV,
    }

    impl TryFrom<char> for Element {
        type Error = ();

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '/' => Ok(Self::MirrorF),
                '\\' => Ok(Self::MirrorB),
                '-' => Ok(Self::SplitH),
                '|' => Ok(Self::SplitV),
                _ => Err(()),
            }
        }
    }
}

aoc2023::main!("../../inputs/day_16");

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 46);
    }

    const EXAMPLE_INPUT_2: &str = EXAMPLE_INPUT_1;

    #[test]
    fn test_part_2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 51);
    }
}
