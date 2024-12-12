
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

type Grid = grid::Grid<(char, bool)>; // (value, claimed)

type Region = std::collections::BTreeSet<Offset>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Orientation { vertical: bool, inward: bool }

type FencePanel = (Offset, Orientation);

type Perimeter = std::collections::BTreeSet<FencePanel>;

const DIRECTIONS: [Offset; 4] = [Offset { x: -1, y:  0 },
                                 Offset { x:  0, y: -1 },
                                 Offset { x:  1, y:  0 },
                                 Offset { x:  0, y:  1 }];

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

fn into_regions(grid: &mut Grid) -> Vec<Region> {

    let mut regions = Vec::new();

    while let Some((offset, _)) = grid.iter().find(|(_, &(_, c))| !c) {

        let mut region = Region::new();

        claim_reachable(grid, offset, &mut region);

        regions.push(region);
    }

    regions
}

fn get_bounds<'a, I>(iter: &I) -> ((isize, isize), (isize, isize))
    where I: Iterator<Item = &'a Offset> + Clone {

    ((iter.clone().map(|o| o.x).min().unwrap(),
      iter.clone().map(|o| o.x).max().unwrap()),
     (iter.clone().map(|o| o.y).min().unwrap(),
      iter.clone().map(|o| o.y).max().unwrap()))
}

fn perimeter(region: &Region) -> Perimeter {

    let ((x_min, x_max), (y_min, y_max)) = get_bounds(&region.iter());

    let mut fences = Perimeter::new();

    let mut add_fences = |vertical, range, to_offset: &dyn Fn(isize) -> Offset| {

        let mut inside = false;

        for index in range {

            let offset = to_offset(index);

            let in_region = region.contains(&offset);

            if inside != in_region {
                
                fences.insert((offset, Orientation { vertical, inward: !inside }));
            }

            inside = in_region;
        }
    };

    for y in y_min ..= y_max {

        add_fences(true, x_min ..= x_max + 1, &|x| Offset { x, y });
    }

    for x in x_min ..= x_max {

        add_fences(false, y_min ..= y_max + 1, &|y| Offset { x, y });
    }

    fences
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let mut grid = Grid::parse(input, |c| Ok((c, false))).unwrap();

        into_regions(&mut grid).iter()
                               .map(|r| r.len() * perimeter(r).len())
                               .sum()
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

mod part_2 {

    use super::*;

    fn sides(perimeter: &Perimeter) -> usize {

        let offsets = perimeter.iter().map(|(o, _)| o);

        let ((x_min, x_max), (y_min, y_max)) = get_bounds(&offsets);     

        let mut sides = 0;

        let mut add_sides = |range, to_fence: &dyn Fn(isize) -> FencePanel| {

            let mut had_fence = false;

            for index in range {

                let has_fence = perimeter.contains(&to_fence(index));

                if !had_fence && has_fence { sides += 1; }

                had_fence = has_fence;
            }
        };

        for y in y_min ..= y_max {

            let to_fence = |x, inward|
                (Offset { x, y }, Orientation { vertical: false, inward });

            add_sides(x_min .. x_max, &|x| to_fence(x, true));
            add_sides(x_min .. x_max, &|x| to_fence(x, false));
        }

        for x in x_min ..= x_max {

            let to_fence = |y, inward|
                (Offset { x, y }, Orientation { vertical: true, inward });

            add_sides(y_min .. y_max, &|y| to_fence(y, true));
            add_sides(y_min .. y_max, &|y| to_fence(y, false));
        }

        sides
    }

    fn get_result(input: &str) -> usize {

        let mut grid = Grid::parse(input, |c| Ok((c, false))).unwrap();

        into_regions(&mut grid).iter()
                               .map(|r| r.len() * sides(&perimeter(r)))
                               .sum()
    }

    #[test]
    fn example_a() { assert_eq!(get_result(EXAMPLE_A), 80); }

    #[test]
    fn example_b() { assert_eq!(get_result(EXAMPLE_B), 436); }

    #[test]
    fn example_c() { assert_eq!(get_result(EXAMPLE_C), 1206); }

    #[test]
    fn example_d() {

        const EXAMPLE: &str = "EEEEE\n\
                               EXXXX\n\
                               EEEEE\n\
                               EXXXX\n\
                               EEEEE";
        
        assert_eq!(get_result(EXAMPLE), 236);
    }

    #[test]
    fn example_e() {

        const EXAMPLE: &str = "AAAAAA\n\
                               AAABBA\n\
                               AAABBA\n\
                               ABBAAA\n\
                               ABBAAA\n\
                               AAAAAA";
        
        assert_eq!(get_result(EXAMPLE), 368);
    }

    #[test]
    fn real() { assert_eq!(get_result(INPUT), 805814); }
}