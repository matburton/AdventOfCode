
const INPUT: &str = include_str!("../input/day_15.txt");

const EXAMPLE_A: &str = include_str!("../examples/day_15_a.txt");

const EXAMPLE_B: &str = include_str!("../examples/day_15_b.txt");

const EXAMPLE_C: &str = include_str!("../examples/day_15_c.txt");

mod grid {

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Offset { pub x: isize, pub y: isize } // Can be used as a coord

    pub struct Grid<T> { pub cells: Vec<Vec<T>> } // Can be jagged

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

fn parse_direction(char: char) -> Offset {

    match char { '^' => Offset { x:  0, y: -1 },
                 'v' => Offset { x:  0, y:  1 },
                 '<' => Offset { x: -1, y:  0 },
                 _   => Offset { x:  1, y:  0 } }
}

mod part_1 {

    use super::*;

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

mod part_2 {

    use super::*;

    fn parse(input: &str) -> Warehouse {

        let expand_char = |c| match c { '#' => ['#', '#'],
                                        'O' => ['[', ']'],
                                        '@' => ['@', '.'],
                                         c  => [ c,   c ] };
        let mut grid = Grid { // TODO: Make parse which take iter of char?

            cells: input.split('\n')
                        .map(|l| l.chars().flat_map(expand_char).collect())
                        .collect()
        };

        let robot = grid.iter().find(|(_, &char)| char == '@').unwrap().0;

        *grid.get_mut(robot).unwrap() = '.';

        Warehouse { robot, grid }
    }

    const LEFT:  Offset = Offset { x: -1, y: 0 };
    const RIGHT: Offset = Offset { x:  1, y: 0 };

    fn other_edge(char: Option<&char>) -> Option<Offset> {

        match char { Some('[') => Some(RIGHT),
                     Some(']') => Some(LEFT),
                     _         => None }
    }

    fn move_boxes(grid: &mut Grid<char>, offset: Offset, direction: Offset) {

        if direction.y == 0 {

            let &char = grid.get(offset).unwrap();

            if char == '[' || char == ']' {

                move_boxes(grid, offset + direction, direction);

                *grid.get_mut(offset + direction).unwrap() = char;

                *grid.get_mut(offset).unwrap() = '.';
            }
        }
        else if let Some(other_edge) = other_edge(grid.get(offset)) {

            move_boxes(grid, offset + direction, direction);
            move_boxes(grid, offset + direction + other_edge, direction);

            *grid.get_mut(offset + direction).unwrap() =
                *grid.get(offset).unwrap();

            *grid.get_mut(offset + direction + other_edge).unwrap() =
                *grid.get(offset + other_edge).unwrap();

            *grid.get_mut(offset).unwrap() = '.';

            *grid.get_mut(offset + other_edge).unwrap() = '.';
        }
    }

    fn can_move(grid: &Grid<char>, offset: Offset, direction: Offset)
        -> bool {

        match (direction.y, grid.get(offset)) {

            (_, Some('.')) => true,

            (0, Some('[' | ']')) =>
                can_move(grid, offset + direction, direction),

            (_, Some('[')) =>
                can_move(grid, offset + direction, direction)
                && can_move(grid, offset + direction + RIGHT, direction),

            (_, Some(']')) =>
                can_move(grid, offset + direction, direction)
                && can_move(grid, offset + direction + LEFT, direction),

            _ => false
        }
    }

    fn move_robot(warehouse: &mut Warehouse, direction: Offset) {

        if can_move(&warehouse.grid, warehouse.robot + direction, direction) {
            
            warehouse.robot = warehouse.robot + direction;

            move_boxes(&mut warehouse.grid, warehouse.robot, direction);
        }
    }

    fn get_result(input: &str) -> isize {

        let mut parts = input.split("\n\n");

        let mut warehouse = parse(parts.next().unwrap());

        let direction_chars =
            parts.next().unwrap().chars().filter(|&c| c != '\n');

        for direction in direction_chars.map(parse_direction) {

            move_robot(&mut warehouse, direction);
        }

        warehouse.grid.iter()
                      .filter(|(_, &c)| c == '[')
                      .map(|(o, _)| o.y * 100 + o.x)
                      .sum()
    }

    #[test]
    fn example_c() { assert_eq!(get_result(EXAMPLE_C), 618); }

    #[test]
    fn example_b() { assert_eq!(get_result(EXAMPLE_B), 9021); }

    #[test]
    fn real() { assert_eq!(get_result(INPUT), 1432898); }
}