
const INPUT: &str = include_str!("../input/day_20.txt");

const EXAMPLE: &str = "###############\n\
                       #...#...#.....#\n\
                       #.#.#.#.#.###.#\n\
                       #S#...#.#.#...#\n\
                       #######.#.#.###\n\
                       #######.#.#...#\n\
                       #######.#.###.#\n\
                       ###..E#...#...#\n\
                       ###.#######.###\n\
                       #...###...#...#\n\
                       #.#####.#.###.#\n\
                       #.#...#.#.#...#\n\
                       #.#.#.#.#.#.###\n\
                       #...#...#...###\n\
                       ###############";

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

fn scores(grid: &Grid<bool>, start: Offset) -> Grid<usize> {

    let mut scores = grid.map(|_| usize::MAX);

    let mut todo = vec![(start, 0)];

    *scores.get_mut(start).unwrap() = usize::MAX;

    while let Some((offset, score)) = todo.pop() {

        if !grid.get(offset).is_some_and(|&b| b) { continue; }

        let cell = scores.get_mut(offset).unwrap();

        if score < *cell {

            *cell = score;

            for d in DIRECTIONS { todo.push((offset + d, score + 1)); }
        }
    }

    scores
}

mod part_1 {

    use super::*;

    fn get_result(input: &str, min_time_save: usize) -> usize {
        
        let char_grid = Grid::parse(input, Ok).unwrap();

        let start = char_grid.iter().find(|(_, &c)| c == 'S').unwrap().0;
        let end   = char_grid.iter().find(|(_, &c)| c == 'E').unwrap().0;

        let grid = char_grid.map(|&c| c != '#');

        let scores_from_start = scores(&grid, start);

        let scores_from_end = scores(&grid, end);

        let no_cheat_time = *scores_from_start.get(end).unwrap();

        let wall_offsets = grid.iter()
                               .filter(|&(_, b)| !b)
                               .map(|(o, _)| o)
                               .collect::<Vec<_>>();

        let mut cheat_times = Vec::new();

        for offset in wall_offsets {

            let mut cheat_time = None;

            let mut try_cheat = |d| {

                let score = scores_from_start
                           .get(offset + d)
                           .and_then(|&t| scores_from_end.get(offset - d)
                                                         .map(|&f| (t, f)))
                           .and_then(|(t, f)| t.checked_add(f))
                           .and_then(|s| s.checked_add(2));

                if let Some(s) = score {

                    if cheat_time.is_none_or(|c| s < c) { cheat_time = Some(s); }
                }
            };

            for direction in DIRECTIONS { try_cheat(direction); }

            if let Some(t) = cheat_time { cheat_times.push(t); }
        }

        cheat_times.into_iter()
                   .filter(|&t| t + min_time_save <= no_cheat_time)
                   .count()
    }
   
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE, 12), 8); }

   
    #[test]
    fn real() { assert_eq!(get_result(INPUT, 100), 1422); }
}