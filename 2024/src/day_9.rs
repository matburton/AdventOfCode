
const INPUT: &str = include_str!("../input/day_9.txt");

const EXAMPLE: &str = "2333133121414131402";

mod part_1 {

    use super::*;

    struct Block { id: u16, file_len: u16, free_len: u16 }

    fn parse(input: &str) -> Vec<Block> {

        let to_block = |(id, (file_len, free_len))|
            Block { id, file_len, free_len };

        input.chars()
             .map(|c| c.to_digit(10).unwrap() as u16)
             .collect::<Vec<_>>()
             .chunks(2)
             .map(|c| (c[0], *c.get(1).unwrap_or(&0)))
             .enumerate()
             .map(|(i, b)| to_block((i as u16, b)))
             .collect()
    }

    fn get_result(input: &str) -> usize {

        let mut disk_map = std::collections::VecDeque::from(parse(input));

        let (mut checksum, mut position) = (0, 0);

        let mut checksum_add = |id, file_len| {

            let position_range = position .. position + file_len as usize;

            checksum += id as usize * position_range.sum::<usize>();

            position += file_len as usize;
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

    type Run = Option<std::ops::Range<usize>>;

    fn file_from_back(disk_map: &[Option<u16>]) -> Run {

        if let Some(i) = disk_map.iter().rev().position(|id| id.is_some()) {

            let end_index = disk_map.len() - i;

            let id = disk_map[end_index - 1];

            let len = disk_map[0 .. end_index].iter()
                                              .rev()
                                              .take_while(|&&i| i == id)
                                              .count();

            return Some(end_index - len .. end_index);
        }

        None                  
    }

    fn free_from_front(disk_map: &[Option<u16>], len: usize) -> Run {

        let mut index = 0;

        while let Some(i) = disk_map[index ..].iter()
                                              .position(|id| id.is_none()) {

            if index + i + len > disk_map.len() { return None; }

            let range = index + i .. index + i + len;

            if disk_map[range.clone()].iter().all(|id| id.is_none()) {

                return Some(range);
            }

            index = range.end;
        }

        None
    }

    fn get_result(input: &str) -> usize {

        let expand = |(id, (file_len, free_len))| {

            let free_iter = std::iter::repeat_n(None, free_len);

            std::iter::repeat_n(Some(id as u16), file_len).chain(free_iter)
        };

        let mut disk_map = input.chars()
                                .map(|c| c.to_digit(10).unwrap() as usize)
                                .collect::<Vec<_>>()
                                .chunks(2)
                                .map(|c| (c[0], *c.get(1).unwrap_or(&0)))
                                .enumerate()
                                .flat_map(expand)
                                .collect::<Vec<_>>();

        let mut prior_index = disk_map.len();

        while let Some(file_run) = file_from_back(&disk_map[0 .. prior_index]) {

            prior_index = file_run.start;

            if let Some(free_run) = free_from_front(&disk_map[0 .. file_run.start],
                                                    file_run.len()) {

                disk_map.copy_within(file_run.clone(), free_run.start);

                disk_map[file_run].fill(None);
            }
        }

        disk_map.iter()
                .enumerate()
                .fold(0, |a, (index, o)| o.map_or(a, |id| a + id as usize * index))
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 2858); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 6323761685944); }
}