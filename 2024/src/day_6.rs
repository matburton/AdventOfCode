
use std::collections::BTreeSet;

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

            Ok(Self {
                
                cells: text.split('\n')
                           .map(|l| l.chars().map(&parse_char).collect())
                           .collect::<Result<Vec<_>, _>>()?
            })
        }

        pub fn get(&self, offset: Offset) -> Option<&T> {

            usize::try_from(offset.x)
                .ok()
                .zip(usize::try_from(offset.y).ok())
                .and_then(|(x, y)| self.cells.get(y).and_then(|v| v.get(x)))
        }

        pub fn set(&mut self, offset: Offset, value: T) {

            self.cells[offset.y as usize][offset.x as usize] = value;
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

fn get_visited(grid: &Grid<char>, start_at: Offset) -> (bool, BTreeSet<Offset>) {

    let mut position = start_at;

    let mut direction = Offset { x: 0, y: -1 }; // Up

    let mut visited = BTreeSet::from([(position, direction)]);

    while let Some(&char) = grid.get(position + direction) {

        if char == '#' { // Turn right (clockwise)

            direction = Offset { x: -direction.y, y: direction.x }
        }
        else { // Move forward (in current direction)

            position = position + direction;

            if !visited.insert((position, direction)) {

                return (true, BTreeSet::new()); // Looped
            }
        }
    }

    (false, BTreeSet::from_iter(visited.iter().map(|(p, _)| *p)))
}

mod part_1 {

    use super::* ;

    fn get_result(input: &str) -> usize {

        let grid = Grid::parse(input, Ok).unwrap();

        let start_at = grid.iter().find(|(_, &c)| c == '^').unwrap().0;

        get_visited(&grid, start_at).1.len()
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

        let start_at = grid.iter().find(|(_, &c)| c == '^').unwrap().0;

        let (mut total_loops, mut last_block_added) = (0, None);

        for offset in get_visited(&grid, start_at).1 {

            if offset == start_at { continue; }

            if let Some(o) = last_block_added { grid.set(o, '.'); }

            grid.set(offset, '#');

            last_block_added = Some(offset);

            if get_visited(&grid, start_at).0 { total_loops +=1 ; }
        }

        total_loops
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 6); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 1753); }
}