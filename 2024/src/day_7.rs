
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

fn get_result(input: &str,
              operators: &[impl Fn(usize, usize) -> usize]) -> usize {

    input.split('\n')
         .map(|l| l.split(' ')
                   .map(|f| f.trim_end_matches(':').parse::<usize>().unwrap())
                   .collect::<Vec<_>>())
         .filter(|v| is_possible(v[0], v[1], &v[2 ..], operators))
         .map(|v| v[0])
         .sum()
}

fn is_possible(target: usize,
               total: usize,
               series: &[usize],
               operators: &[impl Fn(usize, usize) -> usize]) -> bool {

    if total > target { return false; }

    if series.is_empty() { return target == total; }

    operators.iter().any(|o| {
        
        is_possible(target, o(total, series[0]), &series[1 ..], operators)
    })
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

    fn base_10_digits(n: usize) -> usize {

        match n { 0 ..= 9 => 1, _ => base_10_digits(n / 10) + 1 }
    }

    fn get_result(input: &str) -> usize {

        let operators = [
            |a, b| a + b,
            |a, b| a * b,
            |a, b| (0 .. base_10_digits(b)).fold(a, |a, _| a * 10) + b
        ];

        super::get_result(input, &operators)
    }
  
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 11387); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 254136560217241); }
}