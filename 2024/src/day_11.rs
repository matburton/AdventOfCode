
const INPUT: &str = "28591 78 0 3159881 4254 524155 598 1";

const EXAMPLE: &str = "125 17";

type Cache = std::collections::BTreeMap<(usize, usize), usize>;

fn digit_count(num: usize) -> u32 {

    match num { ..= 9 => 1, _ => digit_count(num / 10) + 1 }
}

fn split_digits(num: usize, digit_count: u32) -> (usize, usize) {

    let split_factor = usize::pow(10, digit_count / 2);

    let front_digits = num / split_factor;

    (front_digits, num - front_digits * split_factor)
}

fn split_stone(stone: usize, blinks: usize, cache: &mut Cache) -> usize {

    if blinks == 0 { return 1; }

    if let Some(&count) = cache.get(&(stone, blinks)) { return count; }

    let count = {

        if stone == 0 { return split_stone(1, blinks - 1, cache); }

        let digit_count = digit_count(stone);

        if digit_count % 2 == 0 {

            let (stone_a, stone_b) = split_digits(stone, digit_count);

              split_stone(stone_a, blinks - 1, cache)
            + split_stone(stone_b, blinks - 1, cache)
        }
        else { split_stone(stone * 2024, blinks - 1, cache) }
    };

    cache.insert((stone, blinks), count);

    count
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let mut cache = Cache::new();

        input.split(' ')
             .map(|f| f.parse::<usize>().unwrap())
             .map(|s| split_stone(s, 25, &mut cache))
             .sum()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 55312); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 220722); }
}

mod part_2 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let mut cache = Cache::new();

        input.split(' ')
             .map(|f| f.parse::<usize>().unwrap())
             .map(|s| split_stone(s, 75, &mut cache))
             .sum()
    }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 261952051690787); }
}