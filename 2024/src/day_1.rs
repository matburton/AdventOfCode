
const INPUT: &str = include_str!("../input/day_1.txt");

const EXAMPLE: &str = "3   4\n\
                       4   3\n\
                       2   5\n\
                       1   3\n\
                       3   9\n\
                       3   3";

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {

    let mut sets = (Vec::new(), Vec::new());
    
    for line in input.split('\n')
                     .map(|l| l.split("   ")
                               .collect::<Vec<_>>()) {

        sets.0.push(line[0].parse::<usize>().unwrap());
        sets.1.push(line[1].parse::<usize>().unwrap());
    }

    (sets.0, sets.1)
}

mod part_1 {

    use super::*;
    
    fn get_result(input: &str) -> usize {

        let mut sets = parse(input);

        sets.0.sort();
        sets.1.sort();

        sets.0.iter()
              .zip(sets.1)
              .map(|(a, b)| a.abs_diff(b))
              .sum()
    }
    
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 11); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 1938424); }
}

mod part_2 {

    use std::collections::BTreeMap;

    use super::*;
    
    fn get_result(input: &str) -> usize {

        let sets = parse(input);

        let mut counts = BTreeMap::new();

        for number in sets.1 {

            if let Some(value) = counts.get_mut(&number) { *value += 1; }
            else { counts.insert(number, 1); }
        }

        sets.0.iter()
              .map(|a| a * counts.get(a).unwrap_or(&0))
              .sum()
    }
    
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 31); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 22014209); }
}