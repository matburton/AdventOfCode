
const INPUT: &str = include_str!("../input/day_17.txt");

const EXAMPLE: &str = "Register A: 729\n\
                       Register B: 0\n\
                       Register C: 0\n\
                       \n\
                       Program: 0,1,5,4,3,0";

struct Computer { counter: usize,
                  registers: [usize; 3],
                  program: Vec<usize>,
                  output: Vec<usize> }
impl Computer {

    fn parse(text: &str) -> Self {

        let mut sections = text.split("\n\n");

        let registers: [_; 3] = sections.next()
                                        .unwrap()
                                        .split('\n')
                                        .map(|l| l.split(' ').last().unwrap())
                                        .map(|f| f.parse().unwrap())
                                        .collect::<Vec<_>>()
                                        .try_into()
                                        .unwrap();
        let program = sections.next()
                              .unwrap()
                              .split(' ')
                              .nth(1)
                              .unwrap()
                              .split(',')
                              .map(|f| f.parse().unwrap())
                              .collect();

        Self { counter: 0, registers, program, output: Vec::new() }
    }

    fn run(&mut self) {

        while self.counter < self.program.len() - 1 {

            let [code, op] = [self.program[self.counter],
                              self.program[self.counter + 1]];

            self.counter += 2;

            let combo = match op { 4 .. 7 => self.registers[op - 4], _ => op };

            let mut dv = |r|
                self.registers[r] = self.registers[0] / (1 << combo);

            match code {

                0 /* adv */ => dv(0),
            
                1 /* bxl */ => self.registers[1] ^= op,

                2 /* bst */ => self.registers[1] = combo % 8,

                3 /* jnz */ => if self.registers[0] != 0 { self.counter = op; },
            
                4 /* bxc */ => self.registers[1] ^= self.registers[2],

                5 /* out */ => self.output.push(combo % 8),

                c /* bdv, cdv */ => dv(c - 5)
            };
        }
    }
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> String {

        let mut computer = Computer::parse(input);

        computer.run();

        format!("{:?}", computer.output).replace(['[', ']', ' '], "")
    }
   
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), "4,6,3,5,6,3,5,2,1,0"); }

   
    #[test]
    fn real() { assert_eq!(get_result(INPUT), "4,3,7,1,5,3,0,5,4"); }
}