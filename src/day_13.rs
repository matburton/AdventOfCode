
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

fn reflection_indexes(slices: &[&[char]]) -> Vec<usize> {

    let reflects = |i|
        slices[.. i].iter().rev().zip(slices[i ..].iter()).all(|(a, b)| a == b);

    (1 .. slices.len()).filter(|&i| reflects(i)).collect()
}

fn score_pattern(text: &[char]) -> Vec<usize> {

    let slices = text.chunk_by(|_, &c| c != '\n')
                     .map(|s| if s[0] == '\n' { &s[1 ..] } else { s })
                     .collect::<Vec<_>>();

    let rotated = rotated(&slices);

    let rotated_slices =
        rotated.iter().map(|v| v.as_slice()).collect::<Vec<_>>();

    reflection_indexes(&slices).iter()
                               .map(|s| s * 100)
                               .chain(reflection_indexes(&rotated_slices))
                               .collect()
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        input.split("\n\n")
             .map(|t| score_pattern(&t.chars().collect::<Vec<_>>())[0])
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

        let original_score = score_pattern(&chars)[0];

        let invert = |chars: &mut [char], index|
            chars[index] = match chars[index] { '#' => '.', _ => '#' };

        for index in 0 .. chars.len() {

            if chars[index] == '\n' { continue; }

            invert(&mut chars, index);

            let score = score_pattern(&chars).iter()
                                             .copied()
                                             .find(|&s| s != original_score);

            if let Some(s) = score { return s; }

            invert(&mut chars, index);
        };

        panic!("No new score:\n{}\nOriginal Score {}", text, original_score);
    }

    fn get_result(input: &str) -> usize {

        input.split("\n\n").map(find_unsmudge_score).sum()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 400); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 36474); }
}