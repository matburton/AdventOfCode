
const INPUT: &str = include_str!("../input/day_9.txt");

const EXAMPLE: &str = "2333133121414131402";

use std::collections::VecDeque;

struct Block { id: usize, file_len: usize, free_len: usize }

fn parse(input: &str) -> VecDeque<Block> {

    let to_block = |(id, (file_len, free_len))|
        Block { id, file_len, free_len };

    input.chars()
         .map(|c| c.to_digit(10).unwrap() as usize)
         .collect::<Vec<_>>()
         .chunks(2)
         .map(|c| (c[0], *c.get(1).unwrap_or(&0)))
         .enumerate()
         .map(to_block)
         .collect()
}

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize {

        let mut disk_map = parse(input);

        let (mut checksum, mut position) = (0, 0);

        let mut checksum_add = |id, file_len| {

            checksum += id * (position .. position + file_len).sum::<usize>();

            position += file_len;
        };
 
        while let Some(mut front_block) = disk_map.pop_front() {

            checksum_add(front_block.id, front_block.file_len);

            while front_block.free_len > 0 {

                if let Some(mut back_block) = disk_map.pop_back() {

                    let length = std::cmp::min(front_block.free_len,
                                               back_block.file_len);

                    checksum_add(back_block.id, length);

                    front_block.free_len -= length;

                    back_block.file_len -= length;

                    if back_block.file_len > 0 {
                        
                        disk_map.push_back(back_block);
                    }
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

mod part_2 {

    use super::*;

    fn checksum(blocks: &[Block]) -> usize {

        let (mut checksum, mut position) = (0, 0);

        for block in blocks {

            let position_range = position .. position + block.file_len;

            checksum += block.id * position_range.sum::<usize>();

            position += block.file_len + block.free_len;
        }

        checksum
    }

    fn get_result(input: &str) -> usize {

        let mut disk_map = parse(input);

        let mut back_index = disk_map.len() - 1;

        while back_index > 0 {

            let back_file_len = disk_map[back_index].file_len;

            let front_find = disk_map.make_contiguous()[0 .. back_index]
                                     .iter()
                                     .position(|b| b.free_len >= back_file_len);

            if let Some(front_index) = front_find {

                let mut back_block = disk_map.remove(back_index).unwrap();

                disk_map[back_index - 1].free_len += back_block.file_len
                                                   + back_block.free_len;

                let front_block = &mut disk_map[front_index];

                back_block.free_len = front_block.free_len
                                    - back_block.file_len;

                front_block.free_len = 0;

                disk_map.insert(front_index + 1, back_block);
            }
            else { back_index -= 1; }
        }

        checksum(disk_map.make_contiguous())
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 2858); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 6323761685944); }
}