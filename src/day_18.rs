
const INPUT: &str = include_str!("../input/day_18.txt");

const EXAMPLE: &str = "R 6 (#70c710)\n\
                       D 5 (#0dc571)\n\
                       L 2 (#5713f0)\n\
                       D 2 (#d2c081)\n\
                       R 2 (#59c680)\n\
                       D 2 (#411b91)\n\
                       L 5 (#8ceee2)\n\
                       U 2 (#caa173)\n\
                       L 1 (#1b58a2)\n\
                       U 2 (#caa171)\n\
                       R 2 (#7807d2)\n\
                       U 3 (#a77fa3)\n\
                       L 2 (#015232)\n\
                       U 2 (#7a21e3)";

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord { x: isize, y: isize }

use std::collections::BTreeMap;

use crate::grid::{ Direction, Direction::* };

fn parse_direction(text: &str) -> Direction {
    
    match text { "U" => Up,
                 "D" => Down,
                 "L" => Left,
                 "R" => Right,
                 _   => panic!() }
}

fn parse(text: &str) -> BTreeMap<Coord, [Direction; 2]> {

    let mut map = BTreeMap::new();

    let mut coord = Coord { x: 0, y: 0 };

    let mut previous_direction = Direction::Up;

    let directions =
        text.split('\n')
            .map(|l| l.split(' ').collect::<Vec<_>>())
            .map(|v| (parse_direction(v[0]), v[1].parse::<usize>().unwrap()))
            .collect::<Vec<_>>();

    for (direction, distance) in directions.iter().copied() {

        for _ in 0 .. distance {

            map.insert(coord, coersed([previous_direction, direction]));

            previous_direction = direction;

            coord = match direction {
                Up    => Coord { y: coord.y - 1, ..coord },
                Down  => Coord { y: coord.y + 1, ..coord },
                Left  => Coord { x: coord.x - 1, ..coord },
                Right => Coord { x: coord.x + 1, ..coord }
            };
        }
    }

    *map.get_mut(&coord).unwrap() = coersed([directions.last().unwrap().0,
                                             directions.first().unwrap().0]);
    map
}

fn coersed(directions: [Direction; 2]) -> [Direction; 2] {

    match directions { [Up,   Left] => [Right, Down],
                       [Down, Down] => [Up,    Up],
                       [Down, Left] => [Right, Up],
                       [Left, Up]   => [Down,  Right],
                       [Left, Down] => [Up,    Right],
                       [Left, Left] => [Right, Right],
                       _            => directions }
}

fn score(map: &BTreeMap<Coord, [Direction; 2]>) -> usize {

    let mut score = 0;

    let (min_x, max_x, min_y, max_y) = (map.keys().map(|c| c.x).min().unwrap(),
                                        map.keys().map(|c| c.x).max().unwrap(),
                                        map.keys().map(|c| c.y).min().unwrap(),
                                        map.keys().map(|c| c.y).max().unwrap());
    for y in min_y ..= max_y {

        let mut inside = false;

        let mut toggle_if = None;

        for x in min_x ..= max_x {

            if let Some(directions) = map.get(&Coord { x, y }) {

                match directions {
                    [Up, Up] => inside = !inside,
                    [Right, d] if Some(*d) == toggle_if => { inside = !inside; },
                    [d, Right] if *d != Right => toggle_if = Some(*d),
                    _ => {}
                }

                score += 1;                
            }
            else if inside { score += 1; }
        }
    }

    score
}

mod part_1 {

    use super::*;    

    fn get_result(input: &str) -> usize { score(&parse(input)) }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 62); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 49061); }
}