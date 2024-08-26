
const INPUT: &str = include_str!("../input/day_11.txt");

const EXAMPLE: &str = "...#......\n\
                       .......#..\n\
                       #.........\n\
                       ..........\n\
                       ......#...\n\
                       .#........\n\
                       .........#\n\
                       ..........\n\
                       .......#..\n\
                       #...#.....";

use std::collections::BTreeSet;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Coord { x: usize, y: usize }

fn parse(input: &str) -> BTreeSet<Coord> {

    BTreeSet::from_iter
        (input.split('\n')
              .enumerate()
              .flat_map(|(y, l)| l.chars()
                                  .enumerate()
                                  .filter(|(_, c)| *c == '#')
                                  .map(move |(x, _)| Coord { x, y })))
}

fn expanded(galaxies: &BTreeSet<Coord>, factor: usize) -> BTreeSet<Coord> {

    let min_x = galaxies.iter().map(|c| c.x).min().unwrap();
    let max_x = galaxies.iter().map(|c| c.x).max().unwrap();
    let min_y = galaxies.iter().map(|c| c.y).min().unwrap();
    let max_y = galaxies.iter().map(|c| c.y).max().unwrap();

    let empty_xs = (min_x .. max_x)
                   .filter(|&x| galaxies.iter().all(|c| c.x != x))
                   .collect::<Vec<_>>();

    let empty_ys = (min_y .. max_y)
                   .filter(|&y| galaxies.iter().all(|c| c.y != y))
                   .collect::<Vec<_>>();
        
    BTreeSet::from_iter(galaxies.iter().map(|c| Coord {
        x: c.x + empty_xs.iter().filter(|&x| x < &c.x).count() * (factor - 1),
        y: c.y + empty_ys.iter().filter(|&y| y < &c.y).count() * (factor - 1),
    }))
}

fn get_result(input: &str, expansion_factor: usize) -> usize {

    let galaxies = expanded(&parse(input), expansion_factor);

    galaxies.iter()
            .flat_map(|a| galaxies.iter()
                                  .filter(|&b| b > a)
                                  .map(move |b| (a, b)))
            .map(|(a, b)| a.x.abs_diff(b.x) + a.y.abs_diff(b.y))
            .sum()
}

mod part_1 {

    use super::*;

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE, 2), 374); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT, 2), 9556712); }
}

mod part_2 {

    use super::*;

    #[test]
    fn example_a() { assert_eq!(get_result(EXAMPLE, 10), 1030); }

    #[test]
    fn example_b() { assert_eq!(get_result(EXAMPLE, 100), 8410); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT, 1_000_000), 678626199476); }
}