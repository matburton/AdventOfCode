
const INPUT: &str = include_str!("../input/day_18.txt");

const EXAMPLE: &str = include_str!("../examples/day_18.txt");

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

const DIRECTIONS: [Offset; 4] = [Offset { x:  0, y: -1 },
                                 Offset { x:  1, y:  0 },
                                 Offset { x:  0, y:  1 },
                                 Offset { x: -1, y:  0 }];

fn min_path(grid: &Grid<bool>, start: Offset, end: Offset)
    -> Option<Grid<bool>> {

    let mut scores = grid.map(|_| usize::MAX);

    let mut todo = vec![(start, 0)];

    while let Some((offset, score)) = todo.pop() {

        if !grid.get(offset).is_some_and(|&b| b) { continue; }

        let cell = scores.get_mut(offset).unwrap();

        if score < *cell {

            *cell = score;

            for d in DIRECTIONS { todo.push((offset + d, score + 1)); }
        }
    }

    if *scores.get(end).unwrap() == usize::MAX { return None; }

    let mut min_path = grid.map(|_| false);

    *min_path.get_mut(end).unwrap() = true;

    let mut position = end;

    while position != start {

        position = DIRECTIONS
                  .map(|d| position + d)
                  .into_iter()
                  .min_by_key(|&o| scores.get(o).unwrap_or(&usize::MAX))
                  .unwrap();

        *min_path.get_mut(position).unwrap() = true;
    }

    Some(min_path)
}

mod part_1 {

    use super::*;

    fn get_result(input: &str, end: Offset, take: usize) -> usize {

        let falling =
            input.split('\n')
                 .map(|l| l.split(',').map(|f| f.parse().unwrap()))
                 .map(|mut i| Offset { x: i.next().unwrap(),
                                       y: i.next().unwrap() })
                 .take(take);

        let mut grid = Grid::new(end, true).unwrap();

        for offset in falling { *grid.get_mut(offset).unwrap() = false; }

        min_path(&grid, Offset { x: 0, y: 0 }, end)
            .unwrap()
            .iter()
            .filter(|&(_, &b)| b)
            .count() - 1
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

mod part_2 {

    use super::*;

    fn get_result(input: &str, end: Offset) -> String {

        let falling =
            input.split('\n')
                 .map(|l| l.split(',').map(|f| f.parse().unwrap()))
                 .map(|mut i| Offset { x: i.next().unwrap(),
                                       y: i.next().unwrap() });

        let mut grid = Grid::new(end, true).unwrap();

        let get_min_path = |grid: &Grid<bool>|
            min_path(grid, Offset { x: 0, y: 0 }, end);

        let mut min_path = get_min_path(&grid).unwrap();

        for offset in falling {

            *grid.get_mut(offset).unwrap() = false;

            if !min_path.get(offset).unwrap() { continue; }

            if let Some(path) = get_min_path(&grid) {

                min_path = path;
            }
            else {

                return format!("{},{}", offset.x, offset.y);
            }
        }        

        panic!();
    }
   
    #[test]
    fn example() {
        
        assert_eq!(get_result(EXAMPLE, Offset { x: 6, y: 6 }), "6,1");
    }

   
    #[test]
    fn real() {
        
        assert_eq!(get_result(INPUT, Offset { x: 70, y: 70 }), "58,44");
    }
}