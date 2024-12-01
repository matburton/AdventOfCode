
const INPUT: &str = include_str!("../input/day_1.txt");

const EXAMPLE: &str = "";

mod part_1 {

    use super::*;
    
    fn get_result(input: &str) -> u32 {
    
        0
    }
    
    #[test]
    fn example() {
    
        assert_eq!(get_result(EXAMPLE), 0);
    }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 0); }
}
