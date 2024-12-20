
const INPUT: &str = include_str!("../input/day_19.txt");

const EXAMPLE: &str = include_str!("../examples/day_19.txt");

use std::collections::BTreeMap;

fn count_arrangements<'a>(design: &'a [char],
                          towels: &[Vec<char>],
                          cache: &mut BTreeMap<&'a [char], usize>) -> usize {

    if design.is_empty() { return 1; }

    if let Some(&result) = cache.get(design) { return result; }

    let counts = towels.iter().map(|t| {

        if !design.starts_with(t) { return 0; }

        count_arrangements(&design[t.len() ..], towels, cache)
    });

    let count = counts.sum();

    cache.insert(design, count);

    count
}

fn get_result(input: &str, map_fn: impl Fn(usize) -> usize) -> usize {

    let mut sections = input.split("\n\n");

    let towels = sections.next()
                            .unwrap()
                            .split(", ")
                            .map(|f| f.chars().collect::<Vec<_>>())
                            .collect::<Vec<_>>();
    sections.next()
            .unwrap()
            .split('\n')
            .map(|l| l.chars().collect::<Vec<_>>())
            .map(|l| count_arrangements(&l, &towels, &mut BTreeMap::new()))
            .map(map_fn)
            .sum()
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {
        
        super::get_result(input, |c| match c { 0 => 0, _ => 1 })
    }
   
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 6); }

   
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 344); }
}

mod part_2 {

    use super::*;

    fn get_result(input: &str) -> usize { super::get_result(input, |c| c) }
   
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 16); }

   
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 996172272010026); }
}