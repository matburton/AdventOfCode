
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_12.txt");

const EXAMPLE: &str = "???.### 1,1,3\n\
                       .??..??...?##. 1,1,3\n\
                       ?#?#?#?#?#?#?#? 1,3,1,6\n\
                       ????.#...#... 4,1,1\n\
                       ????.######..#####. 1,6,5\n\
                       ?###???????? 3,2,1";

fn parse_spec(text: &str) -> Vec<usize> {
    text.split(',').map(|f| f.parse().unwrap()).collect()
}

#[derive(PartialEq, Eq, Hash)]
struct State<'a> { row: &'a str, spec: &'a[usize] }

fn arrangements<'a>(mut row: &'a str,
                    spec: &'a [usize],
                    cache: &mut HashMap<State<'a>, usize>) -> usize {

    let left = spec.iter().sum::<usize>();

    if row.chars().filter(|&c| c == '#').count() > left { return 0; }

    let run_length = match spec { [l, ..] => *l, [] => { return 1; } };

    let mut count = 0;

    while row.len() >= left + spec.len() - 1
        && row.chars().filter(|&c| c != '.').count() >= left {

        let fits = !row[.. run_length].contains('.')
                 && row.chars().nth(run_length) != Some('#');

        if fits {

            let state = State {
                row: if row.len() > run_length { &row[run_length + 1 ..] }
                                          else { Default::default() },
                spec: &spec[1 ..]
            };

            count += cache.get(&state).copied().unwrap_or_else(|| {

                let count = arrangements(state.row, state.spec, cache);

                cache.insert(state, count);

                count
            });
        }

        row = if &row[.. 1] == "#" { break; } else { &row[1 ..] };
    }

    count
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        input.split('\n')
             .map(|l| l.split(' ').collect::<Vec<_>>())
             .map(|v| arrangements(v[0], &parse_spec(v[1]), &mut HashMap::new()))
             .sum()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 21); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 7007); }
}

mod part_2 {

    use super::*;

    use std::{ iter::repeat, io::stdout, io::Write };

    fn get_result(input: &str) -> usize {

        let unfold = |t, s| repeat(t).take(5).collect::<Vec<_>>().join(s);

        input.split('\n')
             .enumerate()
             .inspect(|(i, l)| { print!("{:<4} {}", i, &l);
                                 let _ = stdout().flush(); })
             .map(|(_, l)| l.split(' ').collect::<Vec<_>>())
             .map(|v| (unfold(v[0], "?"), unfold(v[1], ",")))
             .map(|t| (std::time::Instant::now(),
                       arrangements(&t.0, &parse_spec(&t.1), &mut HashMap::new())))
             .inspect(|(t, c)| println!(" -> {} ({:?})", c, t.elapsed()))
             .map(|(_, c)| c)
             .sum()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 525152); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 3476169006222); }
}