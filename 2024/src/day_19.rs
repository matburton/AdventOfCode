
const INPUT: &str = include_str!("../input/day_19.txt");

const EXAMPLE: &str = include_str!("../examples/day_19.txt");

mod part_1 {

    use std::collections::BTreeMap;

    use super::*;

    fn is_possible<'a>(design: &'a [char],
                       towels: &[&[char]],
                       cache: &mut BTreeMap<&'a [char], bool>) -> bool {

        if design.is_empty() { return true; }

        if let Some(&result) = cache.get(design) { return result; }

        let result = towels.iter().any(|t| {

            if !design.starts_with(t) { return false; }

            is_possible(&design[t.len() ..], towels, cache)
        });

        cache.insert(design, result);

        result
    }

    fn get_result(input: &str) -> usize {

        let mut sections = input.split("\n\n");

        let towels = sections.next()
                             .unwrap()
                             .split(", ")
                             .map(|f| f.chars().collect::<Vec<_>>())
                             .collect::<Vec<_>>();

        let towel_slices = towels.iter()
                                 .map(|v| v.as_slice())
                                 .collect::<Vec<_>>();
        sections.next()
                .unwrap()
                .split('\n')
                .map(|l| l.chars().collect::<Vec<_>>())
                .filter(|l| is_possible(l, &towel_slices, &mut BTreeMap::new()))
                .count()
    }
   
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 6); }

   
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 344); }
}