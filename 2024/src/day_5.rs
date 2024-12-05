
const INPUT: &str = include_str!("../input/day_5.txt");

const EXAMPLE: &str = include_str!("../examples/day_5.txt");

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {

    let sections = input.split("\n\n").collect::<Vec<_>>();

    let rules = sections[0].split('\n')
                           .map(|l| l.split('|')
                                     .map(|f| f.parse::<usize>().unwrap())
                                     .collect::<Vec<_>>())
                           .map(|v| (v[0], v[1]))
                           .collect::<Vec<_>>();

    let updates = sections[1].split('\n')
                             .map(|l| l.split(',')
                                       .map(|f| f.parse::<usize>().unwrap())
                                       .collect::<Vec<_>>())
                             .collect::<Vec<_>>();
    (rules, updates)
}

fn meets_rule(rule: (usize, usize), update: &[usize]) -> bool {

    update.iter()
          .copied()
          .find(|&p| p == rule.0 || p == rule.1)
          .filter(|&p| p == rule.1 && update.contains(&rule.0))
          .is_none()
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let (rules, updates) = parse(input);

        updates.iter()
               .filter(|u| rules.iter().all(|&r| meets_rule(r, u)))
               .map(|u| u[u.len() / 2])
               .sum()
    }
  
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 143); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 5747); }
}

mod part_2 {

    use std::collections::{BTreeMap, BTreeSet};

    use super::*;

    fn reordered(pages: &[usize], rules: &[(usize, usize)]) -> Vec<usize> {

        let mut dependencies: BTreeMap<usize, BTreeSet<usize>> =
            BTreeMap::from_iter(pages.iter().map(|&p| (p, BTreeSet::new())));

        for (page_a, page_b) in rules {

            if dependencies.contains_key(page_a) {

                if let Some(set) = dependencies.get_mut(page_b) {

                    set.insert(*page_a);
                }
            }
        }

        let mut reordered = Vec::new();

        while !dependencies.is_empty() {
            
            let page = *dependencies.iter()
                                    .find(|(_, s)| s.is_empty())
                                    .unwrap()
                                    .0;
            reordered.push(page);

            dependencies.remove(&page);

            for (_, set) in dependencies.iter_mut() { set.remove(&page); }
        }

        reordered
    }

    fn get_result(input: &str) -> usize {

        let (rules, updates) = parse(input);

        updates.iter()
               .filter(|u| !rules.iter().all(|&r| meets_rule(r, u)))
               .map(|u| reordered(u, &rules))
               .map(|u| u[u.len() / 2])
               .sum()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 123); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 5502); }
}