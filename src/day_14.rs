
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

use super::grid::{ *, Direction::* };

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Rock { Round, Square }

use Rock::*;

struct Platform { grid: Grid<Option<Rock>> }

impl Platform {

    fn parse(input: &str) -> Self {

        let to_rock = |c| match c { '.' => Some(None),
                                    'O' => Some(Some(Rock::Round)),
                                    '#' => Some(Some(Rock::Square)),
                                    _   => None };

        Self { grid: Grid::parse(input, to_rock).unwrap() }
    }

    fn tilt(&mut self, direction: Direction) {

        let mut outer = Some(Coord::from(match direction {
            Down  => (0, self.grid.height() - 1),
            Right => (self.grid.width() - 1, 0),
            _     => (0, 0)
        }));

        while outer.map_or(false, |c| self.grid.in_bounds(c)) {

            let (mut to_coord, mut from_coord) = (outer, outer + !direction);

            while let Some((to, from)) = self.grid.get_two_at_mut(to_coord,
                                                                  from_coord) {
                match (*to, *from) {

                    (None, Some(Round)) => { std::mem::swap(to, from);
                                             to_coord += !direction;
                                             from_coord += !direction; },

                    (None, None) => { from_coord += !direction; },

                    (_, Some(Square)) => { to_coord = from_coord + !direction;
                                           from_coord = to_coord + !direction; },

                    (Some(Round | Square), _) => {
                        to_coord += !direction;
                        from_coord = to_coord + !direction;
                    }
                }

                from_coord = from_coord.filter(|&c| self.grid.in_bounds(c));
            }

            outer += match direction { Up | Down  => Right, _ => Down };
        }
    }

    fn total_north_load(&self) -> usize {

        self.grid.iter()
                 .filter(|(_, v)| **v == Some(Round))
                 .map(|(c, _)| self.grid.height() - c.y)
                 .sum()
    }
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

        let mut cycle = 0;

        loop {

            let hash = platform.grid.get_hash();

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