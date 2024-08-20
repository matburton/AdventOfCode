
const EXAMPLE: &str = "Time:      7  15   30\n\
                       Distance:  9  40  200";

const INPUT: &str = "Time:        60     94     78     82\n\
                     Distance:   475   2138   1015   1650";

fn get_record_beating_count(race_time: usize, record: usize) -> usize {

    let root = ((race_time * race_time - 4 * record) as f64).sqrt();
    
    let mut high = ((race_time as f64 + root) / 2f64).floor() as usize;
    let mut low =  ((race_time as f64 - root) / 2f64).ceil()  as usize;

    let is_record = |p: usize| (race_time - p) * p > record;

    if !is_record(high) { high -= 1; }
    if !is_record(low)  { low  += 1; }

    high - low + 1
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let numbers = input.split('\n')
                           .map(|l| l.split_whitespace().skip(1))
                           .map(|t| t.map(|s| s.parse().unwrap()))
                           .map(|n| n.collect::<Vec<usize>>())
                           .collect::<Vec<_>>();
        numbers[0].iter()
                  .zip(&numbers[1])
                  .map(|(t, r)| get_record_beating_count(*t, *r))
                  .product()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 288); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 345015); }
}

mod part_2 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let numbers = input.replace(' ', "")
                           .split('\n')
                           .map(|l| l.split(':').skip(1).next())
                           .map(|t| t.unwrap().parse().unwrap())
                           .collect::<Vec<usize>>();
        
        get_record_beating_count(numbers[0], numbers[1])
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 71503); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 42588603); }
}