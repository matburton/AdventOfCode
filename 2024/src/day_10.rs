
const INPUT: &str = include_str!("../input/day_10.txt");

const EXAMPLE: &str = "89010123\n\
                       78121874\n\
                       87430965\n\
                       96549874\n\
                       45678903\n\
                       32019012\n\
                       01329801\n\
                       10456732";
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
                                 .collect::<Result<_, _>>()? })
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

mod part_1 {

    use std::collections::BTreeSet;

    use super::{ *, grid::* };

    const DIRECTIONS: [Offset; 4] = [Offset { x: -1, y:  0 },
                                     Offset { x:  0, y: -1 },
                                     Offset { x:  1, y:  0 },
                                     Offset { x:  0, y:  1 }];

    fn add_trail_ends(trail_ends: &mut BTreeSet<Offset>,
                      offset: Offset,
                      next_height: u32,
                      grid: &Grid<u32>) {

        if next_height == 10 { trail_ends.insert(offset); return; }

        for direction in DIRECTIONS {

            let next_offset = offset + direction;

            if grid.get(next_offset) == Some(&next_height) {

                add_trail_ends(trail_ends, next_offset, next_height + 1, grid);
            }
        }
    }

    fn get_result(input: &str) -> usize {

        let grid = Grid::parse(input, |c| Ok(c.to_digit(10).unwrap())).unwrap();

        let score_offset = |offset| {

            let mut trail_ends = BTreeSet::new();

            add_trail_ends(&mut trail_ends, offset, 1, &grid);

            trail_ends.len()
        };

        grid.iter()
            .map(|(o, c)| match c { 0 => score_offset(o), _ => 0 })
            .sum()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 36); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 582); }
}