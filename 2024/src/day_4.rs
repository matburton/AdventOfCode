use grid::Offset;


const INPUT: &str = include_str!("../input/day_4.txt");

const EXAMPLE: &str = "MMMSXXMASM\n\
                       MSAMXMSMSA\n\
                       AMXSXMAAMM\n\
                       MSAMASMSMX\n\
                       XMASAMXAMM\n\
                       XXAMMXXAMA\n\
                       SMSMSASXSS\n\
                       SAXAMASAAA\n\
                       MAMMMXMMMM\n\
                       MXMXAXMASX";
mod grid {

    #[derive(Clone, Copy, PartialEq, Eq)]
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

fn is_match(grid: &Grid<char>, target: &[char], offset: Offset, direction: Offset)
    -> bool { 
    
    target.iter()
          .enumerate()
          .all(|(i, c)| grid.get(offset + direction * i as isize) == Some(c))
}

mod part_1 {

    use super::{ *, grid::* };

    fn get_result(input: &str) -> usize {

        let grid = Grid::parse(input, Ok).unwrap();

        let directions =
            [-1, 0, 1].iter()
                      .flat_map(|&x| [-1, 0, 1].map(|y| Offset { x, y }))
                      .filter(|&o| o != Offset { x: 0, y: 0 })
                      .collect::<Vec<_>>();

        let count_matches = |target, offset|
            directions.iter()
                      .map(|&d| is_match(&grid, target, offset, d))
                      .filter(|&b| b)
                      .count();
        grid.iter()
            .map(|(offset, _)| count_matches(&['X', 'M', 'A', 'S'], offset))
            .sum()
    }
  
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 18); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 2358); }
}

mod part_2 {

    use super::{ *, grid::* };

    fn get_result(input: &str) -> usize {

        let grid = Grid::parse(input, Ok).unwrap();

        let is_mas = |offset, direction|
            is_match(&grid, &['M', 'A', 'S'],
                     offset + direction * -1,
                     direction);

        let is_mas_x = |offset|
               (   is_mas(offset, Offset { x:  1, y:  1 })
                || is_mas(offset, Offset { x: -1, y: -1 }))
            && (   is_mas(offset, Offset { x: -1, y:  1 })
                || is_mas(offset, Offset { x:  1, y: -1 }));

        grid.iter()
            .map(|(offset, _)| is_mas_x(offset))
            .filter(|&b| b)
            .count()
    }
  
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 9); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 1737); }
}