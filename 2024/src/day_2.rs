
const INPUT: &str = include_str!("../input/day_2.txt");

const EXAMPLE: &str = "7 6 4 2 1\n\
                       1 2 7 8 9\n\
                       9 7 6 2 1\n\
                       1 3 2 4 5\n\
                       8 6 4 4 1\n\
                       1 3 6 7 9";

fn is_safe_step(level_prior: isize, level: isize, direction: bool) -> bool {

    let diff = level - level_prior;

    (diff > 0) == direction && diff.abs() <= 3 && diff != 0
}

fn is_safe(levels: &[isize], direction: bool, dampen: bool) -> bool {

    let mut all_steps_safe = true;

    let mut step_safe = true;

    for index in 1 .. levels.len() {

        let is_safe_step = |i_a: usize, i_b: usize|
               index < i_a
            || is_safe_step(levels[index - i_a], levels[index - i_b], direction);

        step_safe = match step_safe {

            true  => is_safe_step(1, 0),

            false => is_safe_step(2, 0)
                  || (is_safe_step(1, 0) && is_safe_step(3, 1))
        };

        if !step_safe {

            if !all_steps_safe || !dampen { return false; }

            all_steps_safe = false;
        }
    }

    true
}

fn get_result(input: &str, dampen: bool) -> usize {

    input.split('\n')
         .map(|l| l.split(' ')
                   .map(|f| f.parse::<isize>().unwrap())
                   .collect::<Vec<_>>())
         .filter(|v| is_safe(v, true, dampen) || is_safe(v, false, dampen))
         .count()
}

mod part_1 {

    use super::*;
   
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE, false), 2); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT, false), 257); }
}

mod part_2 {

    use super::*;
    
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE, true), 4); }
   
    #[test]
    fn real() { assert_eq!(get_result(INPUT, true), 328); }
}