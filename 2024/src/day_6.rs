
const INPUT: &str = include_str!("../input/day_6.txt");

const EXAMPLE: &str = "....#.....\n\
                       .........#\n\
                       ..........\n\
                       ..#.......\n\
                       .......#..\n\
                       ..........\n\
                       .#..^.....\n\
                       ........#.\n\
                       #.........\n\
                       ......#...";
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
   
    impl<T> Grid<T> {
        
        pub fn parse(text: &str, parse_char: impl Fn(char) -> Result<T, String>)
            -> Result<Self, String> {

            Ok(Self { cells: text.split('\n')
                                 .map(|l| l.chars().map(&parse_char).collect())
                                 .collect::<Result<Vec<_>, _>>()? })
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

        pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Grid<U> {

            Grid::<U> { cells: self.cells.iter()
                                         .map(|v| v.iter().map(&f).collect())
                                         .collect() }
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

#[derive(Clone, Copy)]
struct Position { offset: Offset, direction: Offset }

impl Position {

    fn next(&self) -> Self {

        Self { offset: self.offset + self.direction, ..*self }
    }
}

struct Route { positions: Vec<Position>, loops: bool }

fn get_route(grid: &Grid<char>, mut position: Position) -> Route {

    let mut positions = vec![position];

    let mut visted_mask = grid.map(|_| [false; 4]); // Flag per direction

    let to_index = |direction: Offset| // Returns 0..=3 for sane directions
        match direction.x { 0 => direction.y + 1, x => x + 2 } as usize;

    while let Some(&char) = grid.get(position.next().offset) {

        if char == '#' { // Turn right (clockwise)

            let direction = Offset { x: -position.direction.y,
                                     y:  position.direction.x };

            position = Position { direction, ..position };
        }
        else { // Move forward (in current direction)

            position = position.next();

            let visited = visted_mask.get_mut(position.offset)
                                     .unwrap()
                                     .get_mut(to_index(position.direction))
                                     .unwrap();

            if *visited { return Route { positions, loops: true }; }

            *visited = true;
        }

        positions.push(position);
    }

    Route { positions, loops: false }
}

fn get_start_at(grid: &Grid<char>) -> Position {

    Position {
        offset: grid.iter().find(|(_, &c)| c == '^').unwrap().0,
        direction: Offset { x: 0, y: -1 } // Up
    }
}

mod part_1 {

    use super::*;

    use std::collections::BTreeSet;

    fn get_result(input: &str) -> usize {

        let grid = Grid::parse(input, Ok).unwrap();

        let positions = get_route(&grid, get_start_at(&grid)).positions;

        BTreeSet::from_iter(positions.iter().map(|p| p.offset)).len()
    }
  
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 41); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 5239); }
}

mod part_2 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let mut grid = Grid::parse(input, Ok).unwrap();

        let mut been_blocked = grid.map(|_| false);

        let mut set_blocked = |o| // Returns false if already set
            !std::mem::replace(been_blocked.get_mut(o).unwrap(), true);

        let (mut total_loops, mut last_blocked) = (0, None);

        for position in get_route(&grid, get_start_at(&grid)).positions {

            let block_at = position.next().offset;

            match grid.get_mut(block_at) {
                Some(c) if *c != '#' && set_blocked(block_at) => *c = '#',
                _ => continue
            };

            if let Some(o) = last_blocked { *grid.get_mut(o).unwrap() = '.'; }

            last_blocked = Some(block_at);

            if get_route(&grid, position).loops { total_loops += 1; }
        }

        total_loops
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 6); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 1753); }
}