
const INPUT: &str = include_str!("../input/day_12.txt");

const EXAMPLE: &str = "???.### 1,1,3\n\
                       .??..??...?##. 1,1,3\n\
                       ?#?#?#?#?#?#?#? 1,3,1,6\n\
                       ????.#...#... 4,1,1\n\
                       ????.######..#####. 1,6,5\n\
                       ?###???????? 3,2,1";
mod part_1 {

    use super::*;

    use std::iter::repeat;

    #[derive(PartialEq, Eq, Clone, Copy)]
    enum Spring { Operational, Damaged, Unknown }

    use Spring::*;

    struct Config<'a> { springs: &'a [Spring],
                        spec:    &'a [usize],
                        spacing: &'a mut [usize] }

    fn possible(config: &Config) -> bool {

        let base_space = |i|
            if i == 0 || i == config.spacing.len() { 0 } else { 1 };

        config.spacing
              .iter()
              .cloned()
              .enumerate()
              .map(|(i, v)| v + base_space(i))
              .zip(config.spec.iter().cloned().chain(repeat(0).take(1)))
              .map(|(s, d)| (repeat(Operational).take(s),
                             repeat(Damaged).take(d)))
              .flat_map(|(o, d)| o.chain(d))
              .zip(config.springs.iter().cloned())
              .all(|(a, b)| a == b || b == Unknown)
    }

    fn recurse(config: &mut Config, index: usize, left: usize) -> usize {

        if index == config.spacing.len() - 1 {

            config.spacing[index] = left;

            return match possible(config) { true => 1, _ => 0 };
        }

        let mut count = 0;

        for value in 0 ..=left {

            config.spacing[index] = value;

            count += recurse(config, index + 1, left - value);
        }

        count
    }

    fn arrangements(springs: &[Spring], spec: &[usize]) -> usize {
        
        let mut spacing =
            Vec::from_iter(repeat::<usize>(0).take(spec.len() + 1));

        let left = springs.len()
                 - spec.iter().sum::<usize>()
                 + 1
                 - spec.len();

        let mut config = Config { springs, spec, spacing: &mut spacing };

        recurse(&mut config, 0, left)
    }

    fn get_result(input: &str) -> usize {

        let parse_spring = |c| match c { '.' => Operational,
                                         '#' => Damaged,
                                         '?' => Unknown,
                                         _   => panic!() };

        let parse_springs = |text: &str| -> Vec<_> {
            text.chars().map(parse_spring).collect()
        };

        let parse_spec = |text: &str| -> Vec<_> {
            text.split(',').map(|f| f.parse().unwrap()).collect()
        };

        input.split('\n')
             .map(|l| l.split(' ').collect::<Vec<_>>())
             .map(|v| arrangements(&parse_springs(v[0]), &parse_spec(v[1])))
             .sum()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 21); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 7007); }
}