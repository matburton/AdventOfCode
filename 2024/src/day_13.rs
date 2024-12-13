
const INPUT: &str = include_str!("../input/day_13.txt");

const EXAMPLE: &str = "Button A: X+94, Y+34\n\
                       Button B: X+22, Y+67\n\
                       Prize: X=8400, Y=5400\n\
                       \n\
                       Button A: X+26, Y+66\n\
                       Button B: X+67, Y+21\n\
                       Prize: X=12748, Y=12176\n\
                       \n\
                       Button A: X+17, Y+86\n\
                       Button B: X+84, Y+37\n\
                       Prize: X=7870, Y=6450\n\
                       \n\
                       Button A: X+69, Y+23\n\
                       Button B: X+27, Y+71\n\
                       Prize: X=18641, Y=10279";

#[derive(Clone, Copy)]
struct Offset { x: isize, y: isize }

struct Button { offset: Offset, tokens: isize }

struct Machine { buttons: [Button; 2], prize: Offset }

impl Machine {

    fn parse(text: &str) -> Machine {

        let parse_offset = |line: &str| {
            
            let values = line.split([':', ','])
                             .collect::<Vec<_>>()[1 ..]
                             .iter()
                             .map(|f| f.split(['+', '=']).last().unwrap())
                             .map(|f| f.parse().unwrap())
                             .collect::<Vec<_>>();
    
            Offset { x: values[0], y: values[1] }
        };
    
        let offsets = text.split('\n').map(parse_offset).collect::<Vec<_>>();
    
        Machine { buttons: [ Button { offset: offsets[0], tokens: 3 },
                             Button { offset: offsets[1], tokens: 1 } ],
                  prize: offsets[2] }
    }
}

fn button_line_intersection(m: &Machine) -> Offset {

    let numerator_factor = m.prize.x * m.buttons[1].offset.y
                         - m.prize.y * m.buttons[1].offset.x;

    let denominator = m.buttons[0].offset.x * m.buttons[1].offset.y
                    - m.buttons[0].offset.y * m.buttons[1].offset.x;

    Offset { x: m.buttons[0].offset.x * numerator_factor / denominator,
             y: m.buttons[0].offset.y * numerator_factor / denominator }
}

fn min_tokens_to_prize(m: &Machine) -> Option<isize> {

    let intersection = button_line_intersection(m);

    if intersection.x % m.buttons[0].offset.x != 0
    || intersection.y % m.buttons[0].offset.y != 0
    || (m.prize.x - intersection.x) % m.buttons[1].offset.x != 0
    || (m.prize.y - intersection.y) % m.buttons[1].offset.y != 0 {

        return None;
    }

    let tokens = [

        intersection.x
        * m.buttons[0].tokens
        / m.buttons[0].offset.x,

        (m.prize.x - intersection.x)
        * m.buttons[1].tokens
        / m.buttons[1].offset.x
    ];

    Some(tokens.iter().sum())
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> isize {

        input.split("\n\n")
             .map(Machine::parse)
             .filter_map(|m| min_tokens_to_prize(&m))
             .sum()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 480); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 29187); }
}

mod part_2 {

    use super::*;

    fn get_result(input: &str) -> isize {

        let rig = |m: &mut Machine|
            m.prize = Offset { x: m.prize.x + 10000000000000,
                               y: m.prize.y + 10000000000000 };

        input.split("\n\n")
             .map(Machine::parse)
             .map(|mut m| { rig(&mut m); m })
             .filter_map(|m| min_tokens_to_prize(&m))
             .sum()
    }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 99968222587852); }
}