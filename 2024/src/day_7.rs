
use std::thread;

const INPUT: &str = include_str!("../input/day_7.txt");

const EXAMPLE: &str = "190: 10 19\n\
                       3267: 81 40 27\n\
                       83: 17 5\n\
                       156: 15 6\n\
                       7290: 6 8 6 15\n\
                       161011: 16 10 13\n\
                       192: 17 8 14\n\
                       21037: 9 7 18 13\n\
                       292: 11 6 16 20";

fn per_core<T, U>(items: &[T],
                  reducer: impl Fn(U, U) -> U,
                  start: impl Fn(&[T]) -> thread::JoinHandle<U>) -> U {

    let thread_count = thread::available_parallelism().unwrap();

    items.chunks(usize::max(items.len() / thread_count, 1))
         .map(start)
         .collect::<Vec<_>>()
         .into_iter()
         .map(|t| t.join().unwrap())
         .reduce(reducer)
         .unwrap()
}

type Operator = fn(usize, usize) -> usize;

fn get_result(input: &str, operators: Box<[Operator]>) -> usize {

    let equations = input.split('\n')
                         .map(|l| l.split(' ')
                                   .map(|f| f.trim_end_matches(':')
                                   .parse::<usize>()
                                   .unwrap())
                                   .collect::<Vec<_>>())
                         .collect::<Vec<_>>();

    per_core(&equations, |a, b| a + b, |equations| {

        let (equations, operators) = (Vec::from(equations), operators.clone());

        thread::spawn(move || {

            equations.iter()
                     .filter(|v| is_possible(v[0], v[1], &v[2 ..], &operators))
                     .map(|v| v[0])
                     .sum::<usize>()
        })
    })
}

fn is_possible(target: usize,
               total: usize,
               series: &[usize],
               operators: &[fn(usize, usize) -> usize]) -> bool {

    if total > target { return false; }

    if series.is_empty() { return target == total; }

    operators.iter().any(|o| is_possible(target,
                                         o(total, series[0]),
                                         &series[1 ..],
                                         operators))
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        super::get_result(input, Box::new([|a, b| a + b, |a, b| a * b]))
    }
  
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 3749); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 1038838357795); }
}

mod part_2 {

    use super::*;

    fn prefix(a: usize, b: usize) -> usize {

        match b { 0 ..= 9 => a * 10, _ => prefix(a * 10, b / 10) }
    }

    fn get_result(input: &str) -> usize {

        super::get_result(input,
                          Box::new([|a, b| a + b,
                                    |a, b| a * b,
                                    |a, b| prefix(a, b) + b]))
    }
  
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 11387); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 254136560217241); }
}