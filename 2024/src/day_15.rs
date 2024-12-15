
const INPUT: &str = include_str!("../input/day_15.txt");

const EXAMPLE_A: &str = include_str!("../examples/day_15_a.txt");

const EXAMPLE_B: &str = include_str!("../examples/day_15_b.txt");

mod grid {

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Offset { pub x: isize, pub y: isize } // Can be used as a coord

    pub struct Grid<T> { cells: Vec<Vec<T>> } // Can be jagged

    pub struct GridIterator<'a, T> { grid: &'a Grid<T>, offset: Offset }

    impl std::ops::Add<Offset> for Offset {

        type Output = Self;

        fn add(self, offset: Self) -> Self {

            Self { x: self.x + offset.x, y: self.y + offset.y }
        }
    }

    impl std::ops::Mul<isize> for Offset {

        type Output = Self;

        fn mul(self, scalar: isize) -> Self {

            Self { x: self.x * scalar, y: self.y * scalar}
        }
    }
   
    impl<T> Grid<T> {
        
        pub fn parse(text: &str, parse_char: impl Fn(char) -> Result<T, String>)
            -> Result<Self, String> {

            Ok(Self { cells: text.split('\n')
                                 .map(|l| l.chars().map(&parse_char).collect())
                                 .collect::<Result<_, _>>()? })
        }

        pub fn get(&self, offset: Offset) -> Option<&T> {

            usize::try_from(offset.x)
                .ok()
                .zip(usize::try_from(offset.y).ok())
                .and_then(|(x, y)| self.cells.get(y).and_then(|v| v.get(x)))
        }

        pub fn get_mut(&mut self, offset: Offset) -> Option<&mut T> {

            usize::try_from(offset.x)
                .ok()
                .zip(usize::try_from(offset.y).ok())
                .and_then(|(x, y)| self.cells.get_mut(y)
                                             .and_then(|v| v.get_mut(x)))
        }

        pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Grid<U> {

            Grid::<U> { cells: self.cells.iter()
                                         .map(|v| v.iter().map(&f).collect())
                                         .collect() }
        }
        
        pub fn iter(&self) -> GridIterator<T> {

            GridIterator { grid: self, offset: Offset { x: -1, y: 0 } }
        }
    }

    impl<'a, T> Iterator for GridIterator<'a, T> {
        
        type Item = (Offset, &'a T);

        fn next(&mut self) -> Option<Self::Item> {

            if self.offset.y as usize >= self.grid.cells.len() { return None; }

            self.offset.x += 1;

            match self.grid.get(self.offset) {

                Some(v) => Some((self.offset, v)),

                _ => { self.offset = Offset { x: -1, y: self.offset.y + 1 };
                       self.next() }
            }
        }
    }
}

use grid::*;

struct Warehouse { grid: Grid<char>, robot: Offset }

impl Warehouse {

    fn parse(input: &str) -> Self {

        let mut grid = Grid::parse(input, Ok).unwrap();

        let robot = grid.iter().find(|(_, &char)| char == '@').unwrap().0;

        *grid.get_mut(robot).unwrap() = '.';

        Warehouse { robot, grid }
    }

    fn move_robot(&mut self, direction: Offset) {

        let find_non_box = || {

            let mut offset = self.robot + direction;

            while let Some(&c) = self.grid.get(offset) {

                if c == '.' || c == '#' { return Some((offset, c)); }

                offset = offset + direction;
            }

            None
        };

        if let Some((offset, '.')) = find_non_box() {

            self.robot = self.robot + direction;

            if self.robot != offset {

                *self.grid.get_mut(self.robot).unwrap() = '.';

                *self.grid.get_mut(offset).unwrap() = 'O';
            }
        }
    }
}

mod part_1 {

    use super::*;

    fn parse_direction(char: char) -> Offset {

        match char { '^' => Offset { x:  0, y: -1 },
                     'v' => Offset { x:  0, y:  1 },
                     '<' => Offset { x: -1, y:  0 },
                     _   => Offset { x:  1, y:  0 } }
    }

    fn get_result(input: &str) -> isize {

        let mut parts = input.split("\n\n");

        let mut warehouse = Warehouse::parse(parts.next().unwrap());

        let direction_chars =
            parts.next().unwrap().chars().filter(|&c| c != '\n');

        for direction in direction_chars.map(parse_direction) {

            warehouse.move_robot(direction);
        }

        warehouse.grid.iter()
                      .filter(|(_, &c)| c == 'O')
                      .map(|(o, _)| o.y * 100 + o.x)
                      .sum()
    }

    #[test]
    fn example_a() { assert_eq!(get_result(EXAMPLE_A), 2028); }

    #[test]
    fn example_b() { assert_eq!(get_result(EXAMPLE_B), 10092); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 1415498); }
}