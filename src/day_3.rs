
const EXAMPLE: &str = "467..114..\n\
                       ...*......\n\
                       ..35..633.\n\
                       ......#...\n\
                       617*......\n\
                       .....+.58.\n\
                       ..592.....\n\
                       ......755.\n\
                       ...$.*....\n\
                       .664.598..";

const INPUT: &str = include_str!("../input/day_3.txt");

#[derive(Clone, Copy)]
struct Coord { x: usize, y: usize }

impl Coord {

    fn offset(&self, x: isize, y: isize) -> Option<Coord> {

        match (self.x.checked_add_signed(x), self.y.checked_add_signed(y)) {

            (Some(x), Some(y)) => Some(Coord { x, y }),

            _ => None
        }
    }
}

type Matrix = Vec<Vec<char>>;

fn get_character(coord: Coord, matrix: &Matrix) -> Option<char> {

    matrix.get(coord.y)?.get(coord.x).copied()
}

mod part_1 {

    use super::*;

    impl Coord {

        fn get_surrounding(&self) -> Vec<Coord> {

            let mut surrounding = Vec::new();
    
            for x in -1 .. 2 {
    
                for y in -1 .. 2 {
    
                    if let Some(c) = self.offset(x, y) {
    
                        surrounding.push(c);
                    }
                }
            }
    
            surrounding
        }
    }

    fn get_parts(matrix: &Matrix) -> Vec<usize> {

        let mut parts = Vec::new();

        #[derive(Default)]
        struct Acc { number: usize, seen_symbol: bool }

        let mut accumulator: Option<Acc> = None;

        let flush = |acc: &mut Option<Acc>, parts: &mut Vec<usize>| {

            if let Some(Acc { seen_symbol: true, number: n }) = acc {

                parts.push(*n);
            }

            *acc = None;
        };

        for y in 0 .. matrix.len() {

            for x in 0 .. matrix[y].len() {

                let coord = Coord { x, y };

                let char = get_character(coord, &matrix).unwrap();

                if let Ok(digit) = char.to_string().parse::<usize>() {

                    let mut acc = accumulator.unwrap_or_default();

                    acc.number = acc.number * 10 + digit;

                    if !acc.seen_symbol {

                        acc.seen_symbol =
                            coord.get_surrounding()
                                 .into_iter()
                                 .filter_map(|c| get_character(c, &matrix))
                                 .any(|c| c != '.' && !c.is_ascii_digit());
                    }

                    accumulator = Some(acc);
                }
                else { flush(&mut accumulator, &mut parts); }
            }

            flush(&mut accumulator, &mut parts);
        }

        parts
    }

    fn get_result(input: &str) -> usize {

        let matrix: Matrix = input.split('\n')
                                  .map(|l| l.chars().collect())
                                  .collect();

        get_parts(&matrix).into_iter().sum()
    }
    
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 4361); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 536576); }
}

mod part_2 {

    use super::*;

    fn parse_num(mut coord: Coord, matrix: &Matrix) -> Option<usize> {

        if !get_character(coord, matrix).is_some_and(|c| c.is_ascii_digit()) {

            return None;
        }

        while coord.offset(-1, 0).and_then(|c| get_character(c, matrix))
                                 .is_some_and(|c| c.is_ascii_digit()) {

            coord = coord.offset(-1, 0).unwrap();
        }

        let mut number = 0;

        while let Some(char) = get_character(coord, matrix) {

            if let Ok(digit) = char.to_string().parse::<usize>() {

                number = number * 10 + digit;

                coord = coord.offset(1, 0).unwrap();
            }
            else { break; }
        }

        Some(number)
    }

    fn surrounding_numbers(coord: Coord, matrix: &Matrix) -> Vec<usize> {

        let mut numbers = Vec::new();

        let mut parse_push = |x_offset: isize, y_offset: isize| {

            if let Some(n) = coord.offset(x_offset, y_offset)
                                  .and_then(|c| parse_num(c, matrix)) {

                numbers.push(n);
            }
        };

        (-1 .. 2).for_each(|y_offset| parse_push(-1, y_offset));

        for x_offset in 0 .. 2 {

            for y_offset in -1 .. 2 {

                if coord.offset(x_offset - 1, y_offset)
                        .and_then(|c| get_character(c, matrix))
                        .is_some_and(|c| !c.is_ascii_digit()) {

                    parse_push(x_offset, y_offset);
                }
            }
        }

        numbers
    }

    fn get_result(input: &str) -> usize {

        let matrix: Matrix = input.split('\n')
                                  .map(|l| l.chars().collect())
                                  .collect();

        let mut result = 0;

        for y in 0 .. matrix.len() {

            for x in 0 .. matrix[y].len() {

                let coord = Coord { x, y };

                if get_character(coord, &matrix) != Some('*') {

                    continue;
                }

                if let [a, b] = surrounding_numbers(coord, &matrix)[..] {

                    result += a * b;
                }
            }
        }

        result
    }
    
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 467835); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 75741499); }
}