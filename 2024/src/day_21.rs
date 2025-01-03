
const INPUT: &str = "935A\n\
                     319A\n\
                     480A\n\
                     789A\n\
                     176A";

const EXAMPLE: &str = "029A\n\
                       980A\n\
                       179A\n\
                       456A\n\
                       379A";

use std::{ collections::BTreeMap, sync::LazyLock };

static LOOKUP: LazyLock<BTreeMap<(char, char), Vec<char>>> = LazyLock::new(|| {

    let mut lookup = BTreeMap::new();

    let mut add_keypad = |keypad: &[[char; 3]]| {

        let coords = (0 .. keypad.len())
                    .flat_map(|y| (0 .. 3usize).map(move |x| (x, y)))
                    .filter(|&(x, y)| keypad[y][x] != ' ')
                    .collect::<Vec<_>>();

        for &(from_x, from_y) in coords.iter() {

            for &(to_x, to_y) in coords.iter() {

                let mut buttons = Vec::new();

                let vertical_button = if from_y < to_y { 'v' } else { '^' };

                for _ in 0 .. from_y.abs_diff(to_y) {
                    
                    buttons.push(vertical_button);
                }

                let horizontal_button = if from_x < to_x { '>' } else { '<' };

                for _ in 0 .. from_x.abs_diff(to_x) {
                    
                    buttons.push(horizontal_button);
                }

                if from_x == 0 { buttons.reverse(); }

                let key = (keypad[from_y][from_x], keypad[to_y][to_x]);

                lookup.insert(key, buttons);
            }
        }
    };

    add_keypad(&[['7', '8', '9'],
                 ['4', '5', '6'],
                 ['1', '2', '3'],
                 [' ', '0', 'A']]);

    add_keypad(&[[' ', '^', 'A'],
                 ['<', 'v', '>']]);
    lookup
});

fn to_presses(mut sequence: &[char]) -> Vec<char> {

    let mut presses = Vec::new();

    let mut on = 'A';

    while !sequence.is_empty() {

        for &button in LOOKUP[&(on, sequence[0])].iter() {
            
            presses.push(button);
        };

        presses.push('A');

        on = sequence[0];

        sequence = &sequence[1 ..];
    }

    presses
}

fn min_presses(text: &str) -> usize {

    let mut sequence = text.chars().collect::<Vec<_>>();

    for _ in 0 .. 3 { sequence = to_presses(&sequence); }

    println!("Text: {}\nSequence length: {}\nSequence: {}",
             text,
             sequence.len(),
             sequence.iter().collect::<String>());
    
    sequence.len()
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let numeric_part = |text: &str|
            text.trim_end_matches('A').parse::<usize>().unwrap();

        input.split('\n')
             .map(|l| min_presses(l) * numeric_part(l))
             .sum()
    }
  
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 126384); }

   
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 0); }
}