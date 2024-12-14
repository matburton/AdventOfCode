
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

pub trait PerCore<T> {

    fn map_per_core<F, U>(&mut self, f: F) -> Vec<U>
        where F: 'static + Fn(&T) -> U + Clone + Send,
              U: 'static + Send;
}

impl<T, I> PerCore<T> for I
    where I: Iterator<Item = T>,
          T: 'static + Clone + Send {

    fn map_per_core<F, U>(&mut self, f: F) -> Vec<U>
        where F: 'static + Fn(&T) -> U + Clone + Send,
              U: 'static + Send {

        let collected = self.collect::<Vec<_>>();

        let core_count = std::thread::available_parallelism().unwrap().get();

        let threads = collected.chunks(core_count).map(|c| {

            let (c, f) = (c.to_vec(), f.clone());

            std::thread::spawn(move || c.iter().map(f).collect::<Vec<_>>())
        });

        threads.collect::<Vec<_>>()
               .into_iter()
               .flat_map(|t| t.join().unwrap())
               .collect()
    }
}

fn get_result(input: &str,
              operators: &[fn(usize, usize) -> usize]) -> usize {

    let operators = operators.to_vec();

    input.split('\n')
         .map(|l| l.split(' ')
                   .map(|f| f.trim_end_matches(':').parse::<usize>().unwrap())
                   .collect::<Vec<_>>())
         .map_per_core(move |v| { if is_possible(v[0], v[1], &v[2 ..], &operators) { Some(v[0]) } else { None } })
         .iter()
         .flatten()
         .sum()
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

        let operators = [|a, b| a + b, |a, b| a * b];

        super::get_result(input, &operators)
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

        let operators = [|a, b| a + b,
                         |a, b| a * b,
                         |a, b| prefix(a, b) + b];

        super::get_result(input, &operators)
    }
  
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 11387); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 254136560217241); }
}