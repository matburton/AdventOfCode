
const INPUT: &str = include_str!("../input/day_16.txt");

const EXAMPLE: &str = include_str!("../examples/day_16.txt");

use super::grid::{ *, Direction::* };

#[derive(Clone, Copy)]
enum Mirror { Vertical, Horizontal, LeftLean, RightLean }

use Mirror::*;

struct Cell { mirror: Option<Mirror>, beams: [bool; 4] }

impl Direction {

    fn to_index(self) -> usize {

        match self { Up     => 0,
                     Down   => 1,
                     Left   => 2,
                     Right  => 3 }
    }
}

struct Contraption { grid: Grid<Cell> }

impl Contraption {

    fn parse(input: &str) -> Self {

        let parse_char = |c| {

            let mirror = match c { '.'  => None,
                                   '|'  => Some(Vertical),
                                   '-'  => Some(Horizontal),
                                   '\\' => Some(LeftLean),
                                   '/'  => Some(RightLean),
                                   _    => { return None; } };

            Some(Cell { beams: Default::default(), mirror })
        };

        Contraption { grid: Grid::parse(input, parse_char).unwrap() }
    }

    fn add_beams(&mut self, start_at: Coord, direction: Direction) {

        let mut beam_queue = vec![(start_at, direction)];

        while let Some((mut coord, mut direction)) = beam_queue.pop() {

            while let Some(cell) = self.grid.get_at_mut(coord) {

                cell.beams[direction.to_index()] = true;

                match cell.mirror {

                    None => { coord += direction; },
                    Some(Vertical) => {},
                    Some(Horizontal) => {},
                    Some(LeftLean) => {},
                    Some(RightLean) => {}
                }
            }
        }
    }
}

mod part_1 {

    use super::*;    

    fn get_result(input: &str) -> usize {

        let mut contraption = Contraption::parse(input);

        contraption.add_beams(Coord::new(0, 0), Right);
        
        contraption.grid.iter()
                        .filter(|(_, c)| c.beams.iter().any(|&b| b))
                        .count()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 46); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 0); }
}

mod part_2 {

    use super::*;

    fn get_result(input: &str) -> usize { 0 }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 0); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 0); }
}