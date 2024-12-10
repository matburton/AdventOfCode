const INPUT: &str = include_str!("../input/day_8.txt");

const EXAMPLE: &str = "............\n\
                       ........0...\n\
                       .....0......\n\
                       .......0....\n\
                       ....0.......\n\
                       ......A.....\n\
                       ............\n\
                       ............\n\
                       ........A...\n\
                       .........A..\n\
                       ............\n\
                       ............";
mod grid {

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Offset { pub x: isize, pub y: isize } // Can be used as a coord

    pub struct Grid<T> { cells: Vec<Vec<T>> } // Can be jagged

    pub struct GridIterator<'a, T> { grid: &'a Grid<T>, offset: Offset }

    impl std::ops::Sub<Offset> for Offset {

        type Output = Self;

        fn sub(self, offset: Self) -> Self {

            Self { x: self.x - offset.x, y: self.y - offset.y }
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

fn get_result(input: &str,
              add_antis: impl Fn(&mut Grid<bool>, Offset, Offset)) -> usize {

    let antenna_grid = Grid::parse(input, Ok).unwrap();

    let mut antinode_grid = antenna_grid.map(|_| false);

    for (offset_a, &freq) in antenna_grid.iter().filter(|(_, &f)| f != '.') {

        for (offset_b, _) in antenna_grid.iter()
                                         .filter(|(o, &f)| o != &offset_a
                                                        && f == freq) {

            add_antis(&mut antinode_grid, offset_a, offset_b - offset_a);
            add_antis(&mut antinode_grid, offset_b, offset_a - offset_b);
        }
    }

    antinode_grid.iter().filter(|(_, &b)| b).count()
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        super::get_result(input, |grid, start, diff| {

            if let Some(c) = grid.get_mut(start - diff) { *c = true; }
        })
    }
  
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 14); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 371); }
}

mod part_2 {

    use super::*;

    fn get_result(input: &str) -> usize {

        super::get_result(input, |grid, mut start, diff| {

            while let Some(c) = grid.get_mut(start) {

                *c = true;

                start = start - diff;
            }
        })
    }
  
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 34); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 1229); }
}