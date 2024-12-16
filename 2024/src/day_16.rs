
const INPUT: &str = include_str!("../input/day_16.txt");

const EXAMPLE_A: &str = "###############\n\
                         #.......#....E#\n\
                         #.#.###.#.###.#\n\
                         #.....#.#...#.#\n\
                         #.###.#####.#.#\n\
                         #.#.#.......#.#\n\
                         #.#.#####.###.#\n\
                         #...........#.#\n\
                         ###.#.#####.#.#\n\
                         #...#.....#.#.#\n\
                         #.#.#.###.#.#.#\n\
                         #.....#...#.#.#\n\
                         #.###.#.#.#.#.#\n\
                         #S..#.....#...#\n\
                         ###############";

const EXAMPLE_B: &str = "#################\n\
                         #...#...#...#..E#\n\
                         #.#.#.#.#.#.#.#.#\n\
                         #.#.#.#...#...#.#\n\
                         #.#.#.#.###.#.#.#\n\
                         #...#.#.#.....#.#\n\
                         #.#.#.#.#.#####.#\n\
                         #.#...#.#.#.....#\n\
                         #.#.#####.#.###.#\n\
                         #.#.#.......#...#\n\
                         #.#.###.#####.###\n\
                         #.#.#...#.....#.#\n\
                         #.#.#.#####.###.#\n\
                         #.#.#.........#.#\n\
                         #.#.#.#########.#\n\
                         #S#.............#\n\
                         #################";

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

        pub fn debug(&self, to_char: impl Fn(Offset, &T) -> char) {

            for y in 0 .. self.cells.len() {

                for x in 0.. self.cells[y].len() {

                    let offset = Offset { x: x as isize, y: y as isize };

                    print!("{}", to_char(offset, &self.cells[y][x]));
                }

                println!()
            }
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

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let mut grid = Grid::parse(input, Ok).unwrap();

        let mut find = |char| {
            let offset = grid.iter().find(|(_, &c)| c == char).unwrap().0;
            *grid.get_mut(offset).unwrap() = '.';
            offset
        };

        let (start, end) = (find('S'), find('E'));

        let mut scores = grid.map(|_| usize::MAX);

        let turn = |d: Offset, m: isize| Offset { x: -d.y * m, y: d.x * m };

        let mut todo = vec![(start, Offset { x: 1, y: 0 }, 0)];

        while let Some((offset, d, score)) = todo.pop() {

            if grid.get(offset) != Some(&'.') { continue; }

            let cell = scores.get_mut(offset).unwrap();

            if score < *cell {

                *cell = score;

                todo.push((offset + d, d, score + 1));

                todo.push((offset + turn(d, 1), turn(d, 1), score + 1001));

                todo.push((offset + turn(d, -1), turn(d, -1), score + 1001));
            }
        }

        *scores.get(end).unwrap()
    }
   
    #[test]
    fn example_a() { assert_eq!(get_result(EXAMPLE_A), 7036); }

    #[test]
    fn example_b() { assert_eq!(get_result(EXAMPLE_B), 11048); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 88468); }
}