
const INPUT: &str = include_str!("../input/day_14.txt");

const EXAMPLE: &str = "O....#....\n\
                       O.OO#....#\n\
                       .....##...\n\
                       OO.#O....O\n\
                       .O.....O#.\n\
                       O.#..O.#.#\n\
                       ..O..#O..O\n\
                       .......O..\n\
                       #....###..\n\
                       #OO..#....";
use super::grid::*;

use Direction::*;

#[derive(Hash)]
struct Platform { rocks: Vec<Vec<char>> }

impl Platform {

    fn parse(input: &str) -> Self {

        Self {
            rocks: input.split('\n')
                        .map(|l| l.chars().collect::<Vec<_>>())
                        .collect()
        }
    }

    fn in_bounds(&self, coord: Coord) -> bool {

        coord.x < self.rocks[0].len() && coord.y < self.rocks.len()
    }

    fn tilt(&mut self, direction: Direction) {

        let mut outer = Some(match direction {
            Up    => Coord { x: 0, y: 0 },
            Down  => Coord { x: 0, y: self.rocks.len() - 1 },
            Left  => Coord { x: 0, y: 0 },
            Right => Coord { x: self.rocks[0].len() - 1, y: 0 }
        });

        while let Some(edge) = outer {

            let (mut to, mut from) = (outer,
                                      edge.stepped(direction.reversed()).ok());

            while let (Some(t), Some(f)) = (to, from) {

                match (self.rocks[t.y][t.x], self.rocks[f.y][f.x]) {

                    ('.', 'O') => { self.rocks[t.y][t.x] = 'O';
                                    self.rocks[f.y][f.x] = '.';
                                    to = t.stepped(direction.reversed()).ok();
                                    from = f.stepped(direction.reversed()).ok(); },

                    ('.', '.') => { from = f.stepped(direction.reversed()).ok(); },

                    (_, '#') => {
                        to = f.stepped(direction.reversed()).ok();
                        from = to.and_then(|c| c.stepped(direction.reversed()).ok());
                    },

                    _ => {
                        to = t.stepped(direction.reversed()).ok();
                        from = to.and_then(|c| c.stepped(direction.reversed()).ok());
                    }
                }

                from = from.filter(|&c| self.in_bounds(c));
            }

            outer = edge.stepped(match direction { Up   | Down  => Right,
                                                   Left | Right => Down })
                        .ok()
                        .filter(|&c| self.in_bounds(c));
        }
    }

    fn total_north_load(&self) -> usize {

        let round_rock_count = |line: &[char]|
            line.iter().copied().filter(|&c| c == 'O').count();

        let index_to_load = |i| self.rocks.len() - i;

        self.rocks.iter()
                  .enumerate()
                  .map(|(i, v)| round_rock_count(v) * index_to_load(i))
                  .sum()
    }
}

fn get_hash<T: std::hash::Hash>(x: &T) -> u64 {

    use std::hash::Hasher;

    let mut hasher = std::hash::DefaultHasher::new();

    x.hash(&mut hasher);

    hasher.finish()
}

mod part_1 {

    use super::*;    

    fn get_result(input: &str) -> usize {

        let mut platform = Platform::parse(input);

        platform.tilt(Up);

        platform.total_north_load()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 136); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 106990); }
}

mod part_2 {

    use super::*;

    use std::collections::BTreeMap;

    fn cycle_platform(platform: &mut Platform) {

        for direction in [Up, Left, Down, Right] {

            platform.tilt(direction);
        }
    }

    fn cycle_period(platform: &mut Platform) -> (usize, usize) {

        let mut hash_to_cycle = BTreeMap::new();

        let mut cycle = 0usize;

        loop {

            let hash = get_hash(&platform);

            if let Some(c) = hash_to_cycle.get(&hash) {

                return (cycle - c, cycle);
            }
            else {

                hash_to_cycle.insert(hash, cycle);
            }

            cycle_platform(platform);

            cycle += 1;
        }
    }

    fn get_result(input: &str) -> usize {

        let mut platform = Platform::parse(input);

        let (period, cycles) = cycle_period(&mut platform);

        for _ in 0 .. (1_000_000_000 - cycles) % period {

            cycle_platform(&mut platform);
        }

        platform.total_north_load()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 64); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 100531); }
}