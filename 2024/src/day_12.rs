
const INPUT: &str = include_str!("../input/day_12.txt");

const EXAMPLE_A: &str = "AAAA\n\
                         BBCD\n\
                         BBCC\n\
                         EEEC";

const EXAMPLE_B: &str = "OOOOO\n\
                         OXOXO\n\
                         OOOOO\n\
                         OXOXO\n\
                         OOOOO";

const EXAMPLE_C: &str = "RRRRIICCFF\n\
                         RRRRIICCCF\n\
                         VVRRRCCFFF\n\
                         VVRCCCJFFF\n\
                         VVVVCJJCFE\n\
                         VVIVCCJJEE\n\
                         VVIIICJJEE\n\
                         MIIIIIJJEE\n\
                         MIIISIJEEE\n\
                         MMMISSJEEE";
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

const DIRECTIONS: [Offset; 4] = [Offset { x: -1, y:  0 },
                                 Offset { x:  0, y: -1 },
                                 Offset { x:  1, y:  0 },
                                 Offset { x:  0, y:  1 }];
mod part_1 {

    use super::*;

    type Grid = super::Grid<(char, bool)>; // (value, claimed)

    type Region = std::collections::BTreeSet<Offset>;

    fn claim_reachable(grid: &mut Grid, offset: Offset, region: &mut Region) {

        let &(plant, _) = grid.get(offset).unwrap();

        let mut todo = vec![offset];

        while let Some(o) = todo.pop() {

            if let Some((p, claimed)) = grid.get_mut(o) {

                if !*claimed && *p == plant {

                    *claimed = true;

                    region.insert(o);

                    for direction in DIRECTIONS { todo.push(o + direction); }
                }
            }
        }
    }

    fn perimiter(region: &Region) -> usize {

        let x_min = region.iter().map(|o| o.x).min().unwrap();
        let x_max = region.iter().map(|o| o.x).max().unwrap();
        let y_min = region.iter().map(|o| o.y).min().unwrap();
        let y_max = region.iter().map(|o| o.y).max().unwrap();

        let mut fences = 0;

        for y in y_min ..= y_max {

            let mut inside = false;

            for x in x_min ..= x_max + 1 {

                let in_region = region.contains(&Offset { x, y });

                if inside != in_region { fences += 1; }

                inside = in_region;
            }
        }

        for x in x_min ..= x_max {

            let mut inside = false;

            for y in y_min ..= y_max + 1 {

                let in_region = region.contains(&Offset { x, y });

                if inside != in_region { fences += 1; }

                inside = in_region;
            }
        }

        fences
    }

    fn get_result(input: &str) -> usize {

        let mut grid = Grid::parse(input, |c| Ok((c, false))).unwrap();

        let mut regions = Vec::new();

        while let Some((offset, _)) = grid.iter().find(|(_, &(_, c))| !c) {

            let mut region = Region::new();

            claim_reachable(&mut grid, offset, &mut region);

            regions.push(region);
        }

        regions.iter().map(|r| r.len() * perimiter(r)).sum()
    }

    #[test]
    fn example_a() { assert_eq!(get_result(EXAMPLE_A), 140); }

    #[test]
    fn example_b() { assert_eq!(get_result(EXAMPLE_B), 772); }

    #[test]
    fn example_c() { assert_eq!(get_result(EXAMPLE_C), 1930); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 1370258); }
}