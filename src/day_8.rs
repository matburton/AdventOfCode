
const EXAMPLE_A: &str = include_str!("../examples/day_8_a.txt");

const EXAMPLE_B: &str = include_str!("../examples/day_8_b.txt");

const INPUT: &str = include_str!("../input/day_8.txt");

use std::collections::BTreeMap;

type Nodes<'a> = BTreeMap<&'a str, (&'a str, &'a str)>;

fn parse<'a>(input: &'a str) ->
    (Box<dyn Iterator<Item = bool> + 'a>, Nodes<'a>) {

    let lines = input.split('\n').collect::<Vec<_>>();

    let nodes =
        lines[2 ..].iter().map(|l| (&l[0 .. 3], (&l[7 .. 10], &l[12 .. 15])));

    (Box::new(lines[0].chars().map(|c| c == 'L').cycle()),
     BTreeMap::from_iter(nodes))
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let (directions, nodes) = parse(input);

        let mut node = "AAA";

        let mut steps = 0;

        for direction in directions {

            steps += 1;

            node = if direction { nodes[node].0 } else { nodes[node].1 };

            if node == "ZZZ" { break; }
        };

        steps
    }

    #[test]
    fn example_a() { assert_eq!(get_result(EXAMPLE_A), 2); }

    #[test]
    fn example_b() { assert_eq!(get_result(EXAMPLE_B), 6); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 12169); }
}