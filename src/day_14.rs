
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
#[derive(Hash)]
struct Platform { rocks: Vec<Vec<char>> }

#[derive(Clone, Copy)]
enum Direction { North, South, East, West }

use Direction::*;

impl Platform {

    fn parse(input: &str) -> Self {

        Self {
            rocks: input.split('\n')
                        .map(|l| l.chars().collect::<Vec<_>>())
                        .collect()
        }
    }

    fn tilt(&mut self, diretion: Direction) -> bool {

        let mut any_moved = false;

        let (len_x, len_y) = (self.rocks[0].len(), self.rocks.len());

        let (len_a, len_b) = match diretion { North | South => (len_y, len_x),
                                              East  | West  => (len_x, len_y) };
        for a in 0 .. len_a - 1 {

            for b in 0 .. len_b {

                let (x, x_offset, y, y_offset) = match diretion {
                    North => (b, b, a, a + 1),
                    South => (b, b, len_a - a - 1, len_a - a - 2),
                    East  => (len_a - a - 1, len_a - a - 2, b, b),
                    West  => (a, a + 1, b, b)
                };

                if self.rocks[y][x] == '.'
                && self.rocks[y_offset][x_offset] == 'O' {

                    self.rocks[y][x] = 'O';

                    self.rocks[y_offset][x_offset] = '.';

                    any_moved = true;
                }
            }
        }

        any_moved
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

        while platform.tilt(North) {}

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

        for direction in [North, West, South, East] {

            while platform.tilt(direction) {}
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