
const INPUT: &str = include_str!("../input/day_10.txt");

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Direction { North, South, East, West }

use Direction::*;

#[derive(Clone, Copy)]
struct Pipe { connects: [Direction; 2] }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

fn infer_connects(grid: &Grid, coord: Coord) -> [Direction; 2] {

    let mut connects = Vec::new();

    let mut connect_if_has = |direction: Direction, has: Direction| {
        
        if let Some(pipe) = coord.offset(direction)
                                 .and_then(|c| grid.get_pipe(c)) {
        
            if pipe.connects.contains(&has) {

                connects.push(direction);
            }
        }
    };

    connect_if_has(North, South);
    connect_if_has(East,  West);
    connect_if_has(South, North);
    connect_if_has(West,  East);

    connects.sort();

    connects.try_into().unwrap()
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
             
    let mut grid = Grid { start, cells };

    grid.cells[grid.start.y][grid.start.x] =
        Some(Pipe { connects: infer_connects(&grid, grid.start) });

    grid
}

fn step(grid: &Grid, coord: Coord, last: Coord) -> Coord {

    grid.get_pipe(coord)
        .unwrap()
        .connects
        .iter()
        .filter_map(|&d| coord.offset(d))
        .filter(|&c| c != last)
        .next()
        .unwrap()
}

mod part_1 {

    use super::*;

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

    fn get_result(input: &str) -> usize {

        let grid = parse_grid(input);

        let mut trail = vec![grid.start];

        loop {

            let last = match trail.len() { 1 => grid.start,
                                           _ => trail[trail.len() - 2] };

            trail.push(step(&grid, trail[trail.len() - 1], last));

            if trail.last().unwrap() == &grid.start { break; }
        }

        trail.len() / 2
    }

    #[test]
    fn example_a() { assert_eq!(get_result(EXAMPLE_A), 4); }

    #[test]
    fn example_b() { assert_eq!(get_result(EXAMPLE_B), 8); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 6927); }
 }

 mod part_2 {

    use std::{collections::BTreeSet, vec};

    use super::*;

    const EXAMPLE_A: &str = "...........\n\
                             .S-------7.\n\
                             .|F-----7|.\n\
                             .||.....||.\n\
                             .||.....||.\n\
                             .|L-7.F-J|.\n\
                             .|..|.|..|.\n\
                             .L--J.L--J.\n\
                             ...........";

    const EXAMPLE_B: &str = ".F----7F7F7F7F-7....\n\
                             .|F--7||||||||FJ....\n\
                             .||.FJ||||||||L7....\n\
                             FJL7L7LJLJ||LJ.L-7..\n\
                             L--J.L7...LJS7F-7L7.\n\
                             ....F-J..F7FJ|L7L7L7\n\
                             ....L7.F7||L7|.L7L7|\n\
                             .....|FJLJ|FJ|F7|.LJ\n\
                             ....FJL-7.||.||||...\n\
                             ....L---J.LJ.LJLJ...";

    fn get_result(input: &str) -> usize {

        let grid = parse_grid(input);

        let mut trail = vec![grid.start];

        loop {

            let last = match trail.len() { 1 => grid.start,
                                           _ => trail[trail.len() - 2] };

            trail.push(step(&grid, trail[trail.len() - 1], last));

            if trail.last().unwrap() == &grid.start { break; }
        }

        let visited = BTreeSet::from_iter(trail);

        let mut in_count = 0;

        for y in 0 .. grid.cells.len() {

            let mut out = true;

            let mut primer: Option<Direction> = None;

            for x in 0 .. grid.cells[y].len() {

                let coord = Coord { x, y };

                if visited.contains(&coord) {
                   
                    match grid.get_pipe(coord).unwrap().connects {
                        [North, South] => { out = !out; },
                        [North, East] => { primer = Some(South); },
                        [South, East] => { primer = Some(North) },
                        [d, West] => { if primer == Some(d) { out = !out } },
                        _ => {}
                    };
                }
                else if !out { in_count += 1; }
            }
        }

        in_count
    }

    #[test]
    fn example_a() { assert_eq!(get_result(EXAMPLE_A), 4); }

    #[test]
    fn example_b() { assert_eq!(get_result(EXAMPLE_B), 8); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 467); }
 }