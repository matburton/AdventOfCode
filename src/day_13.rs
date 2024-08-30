
const INPUT: &str = include_str!("../input/day_13.txt");

const EXAMPLE: &str = include_str!("../examples/day_13.txt");

fn rotated(slices: &[&[char]]) -> Vec<Vec<char>> {

    let mut slices = slices.to_owned();

    let mut lines = Vec::new();

    while !slices.iter().any(|s| s.is_empty()) {

        let mut line = Vec::new();

        for slice in slices.as_mut_slice() {

            line.push(slice[0]);

            *slice = &slice[1 ..];
        }

        lines.push(line);
    }

    lines
}

fn reflection_index(slices: &[&[char]]) -> Option<usize> {

    (1 .. slices.len()).find(|&i| slices[.. i].iter()
                                              .rev()
                                              .zip(slices[i ..].iter())
                                              .all(|(a, b)| a == b))
}

fn score_pattern(text: &[char]) -> Option<usize> {

    let slices = text.chunk_by(|_, &c| c != '\n')
                     .map(|s| if s[0] == '\n' { &s[1 ..] } else { s })
                     .collect::<Vec<_>>();

    if let Some(index) = reflection_index(&slices) { return Some(index * 100); }
    
    let rotated = rotated(&slices);

    reflection_index(&rotated.iter().map(|v| v.as_slice()).collect::<Vec<_>>())
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        input.split("\n\n")
             .map(|t| t.chars().collect::<Vec<_>>())
             .filter_map(|v| score_pattern(&v))
             .sum()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 405); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 30158); }
}

mod part_2 {

    use super::*;

    fn find_unsmudge_score(text: &str) -> usize {

        let mut chars = text.chars().collect::<Vec<_>>();

        let original_score = score_pattern(&chars).unwrap();

        let invert = |chars: &mut [char], index|
            chars[index] = match chars[index] { '#' => '.', _ => '#' };

        for index in 0 .. chars.len() {

            if chars[index] == '\n' { continue; }

            invert(&mut chars, index);

            if let Some(score) = score_pattern(&chars) {

                if score != original_score { return score; }
            }

            invert(&mut chars, index);
        };

        original_score
    }

    fn get_result(input: &str) -> usize {

        input.split("\n\n").map(find_unsmudge_score).sum()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 400); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 44333); } // TODO: Too high
}