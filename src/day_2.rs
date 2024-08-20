
const EXAMPLE: &str =
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

const INPUT: &str = include_str!("../input/day_2.txt");

use std::collections::BTreeMap;

type BallCounts<'a> = BTreeMap<&'a str, usize>;

fn to_draws(text: &str) -> Vec<BallCounts> {

    text.split(';').map(to_draw).collect()
}

fn to_draw(text: &str) -> BallCounts {

    text.split(',')
        .map(|p| p.trim().split(' ').collect::<Vec<_>>())
        .map(|p| (p[1], p[0].parse::<usize>().unwrap()))
        .collect()
}

mod part_1 {

    use super::*;
 
    fn get_result(input: &str, contents: &BallCounts) -> usize {
    
        input.split('\n')
             .map(|l| l.split(':').collect::<Vec<_>>())
             .map(|p| (p[0][5 ..].parse::<usize>().unwrap(), to_draws(p[1])))
             .filter(|(_, b)| b.iter().all(|d| draw_possible(d, contents)))
             .map(|(g, _)| g)
             .sum()
    }

    fn draw_possible(draw: &BallCounts, contents: &BallCounts) -> bool {

        draw.iter().all(|t| match contents.get(t.0) {

            Some(c) => t.1 <= c,
            _       => false
        })
    }

    use std::sync::LazyLock;

    static CONTENTS: LazyLock<BallCounts> = LazyLock::new(|| {

        BallCounts::from([("red",   12),
                          ("green", 13),
                          ("blue",  14)])
    });
    
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE, &CONTENTS), 8); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT, &CONTENTS), 2268); }
}

mod part_2 {
    
    use std::cmp::max;

    use super::*;

    fn get_result(input: &str) -> usize {
    
        input.split('\n')
             .map(|l| to_draws(l.split(':').skip(1).next().unwrap()))
             .map(|d| min_contents(&d).values().fold(1, |a, c| a * c))
             .sum()
    }

    fn min_contents<'a>(draws: &[BallCounts<'a>]) -> BallCounts<'a> {

        let mut min_contents = BallCounts::new();

        for draw in draws.iter().flatten() {
            
            match min_contents.get_mut(draw.0) {

                Some(c) => { *c = max(*c, *draw.1) },
                _       => { min_contents.insert(draw.0, *draw.1); }
            }
        }

        min_contents
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 2286); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 63542); }
}