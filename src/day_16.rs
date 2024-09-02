
const INPUT: &str = include_str!("../input/day_16.txt");

const EXAMPLE: &str = include_str!("../examples/day_16.txt");

use super::grid::{ *, Direction::* };

#[derive(Clone, Copy)]
enum Mirror { Vertical, Horizontal, LeftLean, RightLean }

use Mirror::*;

#[derive(Clone)]
struct Cell { mirror: Option<Mirror>, beams: [bool; 4] }

impl Direction {

    fn to_index(self) -> usize {

        match self { Up     => 0,
                     Down   => 1,
                     Left   => 2,
                     Right  => 3 }
    }
}

#[derive(Clone)]
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

    fn add_beams(&mut self, mut coord: Option<Coord>, mut direction: Direction) {

        while let Some(cell) = self.grid.get_at_mut(coord) {

            match &mut cell.beams[direction.to_index()] { true => break,
                                                          b => *b = true }

            direction = match (cell.mirror, direction) {

                  (None, _)
                | (Some(Vertical),   Up   | Down)
                | (Some(Horizontal), Left | Right) => direction,

                (Some(LeftLean), Up)    | (Some(RightLean), Down)  => Left,
                (Some(LeftLean), Down)  | (Some(RightLean), Up)    => Right,
                (Some(LeftLean), Left)  | (Some(RightLean), Right) => Up,
                (Some(LeftLean), Right) | (Some(RightLean), Left)  => Down,

                (Some(Vertical), Left | Right) => {
                    self.add_beams(coord + Down, Down);
                    Up
                },

                (Some(Horizontal), Up | Down) => {
                    self.add_beams(coord + Left, Left);
                    Right
                }
            };

            coord += direction;
        }
    }

    fn energized_count(&self) -> usize {

        self.grid.iter()
                 .filter(|(_, c)| c.beams.iter().any(|&b| b))
                 .count()
    }
}

mod part_1 {

    use super::*;    

    fn get_result(input: &str) -> usize {

        let mut contraption = Contraption::parse(input);

        contraption.add_beams(Some(Coord::new(0, 0)), Right);
        
        contraption.energized_count()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 46); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 7860); }
}

mod part_2 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let contraption = Contraption::parse(input);

        let (max_x, max_y) = (contraption.grid.width()  - 1,
                              contraption.grid.height() - 1);

        let energized_count = |(c, d)| {
            let mut clone = contraption.clone();
            clone.add_beams(Some(c), d);
            clone.energized_count()
        };

        let horizontal_entries =
            (0 ..= max_x).flat_map(|x| [(Coord::new(x, 0), Down),
                                        (Coord::new(x, max_y), Up)]);
        let vertical_entries =
            (0 ..= max_y).flat_map(|y| [(Coord::new(0, y), Right),
                                        (Coord::new(max_x, y), Left)]);

        horizontal_entries.chain(vertical_entries)
                          .map(energized_count)
                          .max()
                          .unwrap()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 51); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 8331); }
}