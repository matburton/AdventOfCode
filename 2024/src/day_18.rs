
const INPUT: &str = include_str!("../input/day_18.txt");

const EXAMPLE: &str = include_str!("../examples/day_18.txt");

mod grid {

    #[derive(Clone, Copy)]
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
       
        pub fn new(end: Offset, value: T)
            -> Result<Self, Box<dyn std::error::Error>>
            where T: Clone {

            Ok(Self {

                cells: vec![vec![value; usize::try_from(end.x + 1)?];
                            usize::try_from(end.y + 1)?]
            })
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
    }
}

use grid::*;

const DIRECTIONS: [Offset; 4] = [Offset { x:  0, y: -1 },
                                 Offset { x:  1, y:  0 },
                                 Offset { x:  0, y:  1 },
                                 Offset { x: -1, y:  0 }];

struct Maze { grid: Grid<bool>, start: Offset, end: Offset }

impl Maze {

    fn new(grid: Grid<bool>, start: Offset, end: Offset) -> Self {

        Self { grid, start, end }
    }

    fn score(&self) -> usize {

        let mut scores = self.grid.map(|_| usize::MAX);

        let mut todo = vec![(self.start, 0)];

        while let Some((offset, score)) = todo.pop() {

            if !self.grid.get(offset).is_some_and(|&b| b) { continue; }

            let cell = scores.get_mut(offset).unwrap();

            if score < *cell {

                *cell = score;

                for d in DIRECTIONS { todo.push((offset + d, score + 1)); }
            }
        }

        *scores.get(self.end).unwrap()
    }
}

mod part_1 {

    use super::*;

    fn get_result(input: &str, end: Offset, take: usize) -> usize {

        let falling =
            input.split('\n')
                 .map(|l| l.split(',').map(|f| f.parse::<isize>().unwrap()))
                 .map(|mut i| Offset { x: i.next().unwrap(),
                                       y: i.next().unwrap() })
                 .take(take);

        let mut grid = Grid::new(end, true).unwrap();

        for offset in falling { *grid.get_mut(offset).unwrap() = false; }

        Maze::new(grid, Offset { x: 0, y: 0 }, end).score()
    }
   
    #[test]
    fn example() {
        
        assert_eq!(get_result(EXAMPLE, Offset { x: 6, y: 6 }, 12), 22);
    }

   
    #[test]
    fn real() {
        
        assert_eq!(get_result(INPUT, Offset { x: 70, y: 70 }, 1024), 292);
    }
}