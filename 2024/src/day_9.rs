
const INPUT: &str = include_str!("../input/day_9.txt");

const EXAMPLE: &str = "2333133121414131402";

mod part_1 {

    use std::collections::VecDeque;

    use super::*;

    fn get_result(input: &str) -> usize {

        let mut disk_map = input.chars()
                                .map(|c| c.to_digit(10).unwrap() as usize)
                                .collect::<Vec<_>>()
                                .chunks(2)
                                .map(|c| [c[0], *c.get(1).unwrap_or(&0)])
                                .enumerate()
                                .collect::<VecDeque<_>>();

        let (mut checksum, mut position) = (0, 0);

        let mut checksum_add = |id, file_len| {

            checksum += id * (position .. position + file_len).sum::<usize>();

            position += file_len;
        };
 
        while let Some((id, [file_len, mut free_len])) = disk_map.pop_front() {

            checksum_add(id, file_len);

            while free_len > 0 {

                if let Some((id, [mut file_len, _])) = disk_map.pop_back() {

                    let take = std::cmp::min(free_len, file_len);

                    checksum_add(id, take);

                    free_len -= take;
                    file_len -= take;

                    if file_len > 0 { disk_map.push_back((id, [file_len, 0])); }
                }
                else { break; }
            }
        }

        checksum
    }
  
    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 1928); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 6301895872542); }
}