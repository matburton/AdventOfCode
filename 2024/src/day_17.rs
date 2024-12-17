
const INPUT: &str = include_str!("../input/day_17.txt");

const EXAMPLE: &str = "Register A: 729\n\
                       Register B: 0\n\
                       Register C: 0\n\
                       \n\
                       Program: 0,1,5,4,3,0";

struct Computer { program_counter: usize,
                  registers: [isize; 3],
                  program: Vec<usize>,
                  output: Vec<isize> }

impl Computer {

    fn parse(text: &str) -> Self {

        let sections = text.split("\n\n");

        let registers = sections.next()
                                .unwrap()
                                .split('\n')
                                .map(|l| l.split(' ').last().unwrap())
                                .map(|f| f.parse::<isize>().unwrap());
    }

    fn next(&mut self) -> bool {

        false
    }
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> Vec<usize> {

        Vec::new()
    }
   
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), [4,6,3,5,6,3,5,2,1,0]); }

   
    #[test]
    fn real() { assert_eq!(get_result(INPUT), []); }
}