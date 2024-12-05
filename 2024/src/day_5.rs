
const INPUT: &str = include_str!("../input/day_5.txt");

const EXAMPLE: &str = include_str!("../examples/day_5.txt");

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let sections = input.split("\n\n").collect::<Vec<_>>();

        let rules = sections[0].split('\n')
                               .map(|l| l.split('|')
                                         .map(|f| f.parse::<usize>().unwrap())
                                         .collect::<Vec<_>>())
                               .map(|v| (v[0], v[1]))
                               .collect::<Vec<_>>();

        let meets_rule = |rule: (usize, usize), update: &[usize]|
            update.iter()
                  .copied()
                  .find(|&p| p == rule.0 || p == rule.1)
                  .filter(|&p| p == rule.1 && update.contains(&rule.0))
                  .is_none();

        sections[1].split('\n')
                   .map(|l| l.split(',')
                             .map(|f| f.parse::<usize>().unwrap())
                             .collect::<Vec<_>>())
                   .filter(|u| rules.iter().all(|&r| meets_rule(r, u)))
                   .map(|u| u[u.len() / 2])
                   .sum()
    }
  
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 143); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 5747); }
}