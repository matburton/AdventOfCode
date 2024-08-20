
const EXAMPLE: &str = include_str!("../examples/day_5.txt");

const INPUT: &str = include_str!("../input/day_5.txt");

use std::ops::Range;    

struct MapItem { source: Range<usize>, target_start: usize }

fn parse_to_map_item(line: &str) -> MapItem {

    let numbers = line.split(' ')
                      .map(|t| t.parse().unwrap())
                      .collect::<Vec<_>>();

    MapItem { source: (numbers[1] .. numbers[1] + numbers[2]),
              target_start: numbers[0] }
}

fn parse_to_map(line: &str) -> Vec<MapItem> {

    line.split('\n').skip(1).map(parse_to_map_item).collect()
}

mod part_1 {

    use super::*;

    fn apply_map_item(source: usize, map_item: &MapItem) -> Option<usize> {

        if !map_item.source.contains(&source) { return None }
    
        Some(source - map_item.source.start + map_item.target_start)
    }
    
    fn apply_map(source: usize, map: &Vec<MapItem>) -> usize {
    
        map.iter()
           .find_map(|i| apply_map_item(source, i))
           .unwrap_or(source)
    }

    fn get_result(input: &str) -> usize {
    
        let sections = input.split("\n\n").collect::<Vec<_>>();

        let maps: Vec<_> =
            sections.iter().skip(1).map(|t| parse_to_map(t)).collect();

        sections[0].split(' ')
                   .skip(1)
                   .map(|t| t.parse().unwrap())
                   .map(|s| maps.iter().fold(s, apply_map))
                   .min()
                   .unwrap()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 35); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 218513636); }
}

mod part_2 {
    
    use std::cmp::{ max, min };

    use super::*;

    struct Sliced<T> { matching: Range<T>, remainder: [Range<T>; 2] }

    fn slice_range(source: &Range<usize>, target: &Range<usize>) ->
        Sliced<usize> {

        if source.end <= target.start || source.start >= target.end {

            return Sliced { matching: 0 .. 0,
                            remainder: [source.clone(), 0 .. 0] };
        }

        Sliced { matching: max(source.start, target.start)
                           .. min(source.end, target.end),

                 remainder: [ source.start .. target.start,
                              target.end   .. source.end] }
    }

    #[derive(Default)]
    struct Applied<T> { output: Vec<Range<T>>, remainder: Vec<Range<T>> }

    fn apply_map_item(ranges: &Vec<Range<usize>>, map_item: &MapItem) ->
        Applied<usize> {

        let mut applied: Applied<_> = Default::default();

        for range in ranges {

            let sliced = slice_range(range, &map_item.source);

            if !sliced.matching.is_empty() {

                let start_offset = sliced.matching.start
                                 - map_item.source.start;

                let target_start = map_item.target_start + start_offset;

                let target_end = target_start + sliced.matching.len();

                applied.output.push(target_start .. target_end);
            }

            sliced.remainder.into_iter()
                            .filter(|r| !r.is_empty())
                            .for_each(|r| applied.remainder.push(r));
        }

        applied
    }

    fn apply_map(range: Range<usize>, map: &Vec<MapItem>) ->
        Vec<Range<usize>> {
    
        let mut output = Vec::new();

        let mut remainders = Vec::from([range]);

        for item in map {

            let mut applied = apply_map_item(&remainders, item);

            output.append(&mut applied.output);

            remainders = applied.remainder;
        }

        output.append(&mut remainders);

        output
    }

    fn get_result(input: &str) -> usize {
    
        let sections = input.split("\n\n").collect::<Vec<_>>();

        let maps: Vec<_> =
            sections.iter().skip(1).map(|t| parse_to_map(t)).collect();

        let mut ranges = sections[0].split(' ')
                                    .skip(1)
                                    .map(|t| t.parse().unwrap())
                                    .collect::<Vec<_>>()
                                    .chunks(2)
                                    .map(|p| p[0] .. p[0] + p[1])
                                    .collect::<Vec<_>>();
        for map in maps {
            
            ranges = ranges.into_iter()
                           .flat_map(|r| apply_map(r, &map))
                           .collect();
        }

        ranges.into_iter().map(|r| r.start).min().unwrap()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 46); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 81956384); }
}