
const EXAMPLE_A: &str = ".....\n\
                         .S-7.\n\
                         .|.|.\n\
                         .L-J.\n\
                         .....";

const EXAMPLE_B: &str = "..F7.\n\
                         .FJ|.\n\
                         SJ.L7\n\
                         |F--J\n\
                         LJ...";

const INPUT: &str = include_str!("../input/day_10.txt");

 mod part_1 {

    use super::*;

    #[derive(Clone, Copy)]
    enum Direction { North, East, South, West }

    use Direction::*;

    #[derive(Clone, Copy)]
    struct Pipe { connects: [Direction; 2] }

    #[derive(Clone, Copy, PartialEq, Eq)]
    struct Coord { x: usize, y: usize }

    impl Coord {

        fn offset(&self, direction: Direction) -> Option<Coord> {

            let (x, y) = match direction { North => (0, -1),
                                           East  => (1, 0),
                                           South => (0, 1),
                                           West  => (-1, 0) };
    
            match (self.x.checked_add_signed(x), self.y.checked_add_signed(y)) {
    
                (Some(x), Some(y)) => Some(Coord { x, y }),
    
                _ => None
            }
        }
    }

    struct Grid { start: Coord, cells: Vec<Vec<Option<Pipe>>> }

    impl Grid {

        fn get_pipe(&self, coord: Coord) -> Option<Pipe> {

            self.cells.get(coord.y)?.get(coord.x)?.clone()
        }
    }

    fn parse_pipe(char: char) -> Option<Pipe> {

        if char == '.' || char == 'S' { return None; }

        let connects = match char { '|' => [North, South],
                                    '-' => [East,  West],
                                    'L' => [North, East],
                                    'J' => [North, West],
                                    '7' => [South, West],
                                    'F' => [South, East],
                                    _ => panic!("Bad pipe char") };
        Some(Pipe { connects })
    }

    fn parse_grid(input: &str) -> Grid {

        let lines = input.split('\n').collect::<Vec<_>>();

        let cells = lines.iter()
                         .map(|l| l.chars().map(parse_pipe).collect::<Vec<_>>())
                         .collect::<Vec<_>>();

        let start =
            lines.into_iter()
                 .enumerate()
                 .flat_map(|(y, l)| l.chars()
                                     .enumerate()
                                     .map(move|(x, c)| (Coord { x, y }, c)))
                 .find(|(_, c)| *c == 'S')
                 .unwrap()
                 .0;
                 
        Grid { start, cells }
    }

    fn step(grid: &Grid, coord: Coord, last: Coord) -> Coord {

        if let Some(pipe) = grid.get_pipe(coord) {

            return pipe.connects
                       .iter()
                       .filter_map(|&d| coord.offset(d))
                       .filter(|&c| c != last)
                       .next()
                       .unwrap()
        }

        for direction in [North, East, South, West] {

            if let Some(neighbour) = coord.offset(direction) {

                if let Some(pipe) = grid.get_pipe(neighbour) {

                    let connects = pipe.connects
                                       .iter()
                                       .filter_map(|&d| neighbour.offset(d))
                                       .any(|c| c == coord);

                    if connects { return neighbour; }
                }
            }
        }

        panic!()
    }

    fn get_result(input: &str) -> usize {

        let grid = parse_grid(input);

        let (mut coord, mut last) = (grid.start, grid.start);

        let mut steps = 0;

        loop {

            let new_coord = step(&grid, coord, last);

            last = coord;

            coord = new_coord;

            steps += 1;

            if coord == grid.start { break; }
        }

        steps / 2
    }

    #[test]
    fn example_a() { assert_eq!(get_result(EXAMPLE_A), 4); }

    #[test]
    fn example_b() { assert_eq!(get_result(EXAMPLE_B), 8); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 6927); }
 }