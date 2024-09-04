
// TODO: This is very slow. 10 mins to solve part 1
//       There must be a better way!
//       Do I have too many states per cell?
//       Immutable linked list to avoid re-visiting blocks?

const INPUT: &str = include_str!("../input/day_17.txt");

const EXAMPLE: &str = "2413432311323\n\
                       3215453535623\n\
                       3255245654254\n\
                       3446585845452\n\
                       4546657867536\n\
                       1438598798454\n\
                       4457876987766\n\
                       3637877979653\n\
                       4654967986887\n\
                       4564679986453\n\
                       1224686865563\n\
                       2546548887735\n\
                       4322674655533";

use super::grid::{ *, Direction:: * };

struct Block { loss: usize, min_total_losses: [usize; 4 * 3] }

struct City { grid: Grid<Block> }

impl City {

    fn parse(text: &str) -> Self {

        let parse_char = |c: char|
            Some(Block { loss: c.to_string().parse().unwrap(),
                         min_total_losses: [usize::MAX; 4 * 3] });

        City { grid: Grid::parse(text, parse_char).unwrap() }
    }

    fn min_loss(&mut self) -> usize {

        let start_coord = Coord::new(0, 0);

        let mut stack = vec![(start_coord, 0usize, Right),
                             (start_coord, 0usize, Down)];

        let to_index = |i, d: Direction| d.to_index() * 3 + i;

        while let Some((mut coord, mut total_loss, direction)) = stack.pop() {

            for forward_index in 0 .. 3 {

                coord = if let Some(c) = coord + direction { c }
                        else { break; };

                let Some(block) = self.grid.get_at_mut(Some(coord))
                                  else { break; };

                total_loss += block.loss;

                let index = to_index(forward_index, direction);

                if block.min_total_losses[index] <= total_loss { break; }

                for i in forward_index .. 3 {

                    block.min_total_losses[to_index(i, direction)] = total_loss;
                }

                // TODO: Prefer direction with lower value
                //       to trim later searches earlier?

                if coord.x != self.grid.width() - 1 && coord.y != 0 {

                    stack.push((coord, total_loss, direction.turned(Turn::Left)));
                }

                if coord.y != self.grid.height() - 1 && coord.x != 0 {

                    stack.push((coord, total_loss, direction.turned(Turn::Right)));
                }
            }
        }

        let end_coord = Coord::new(self.grid.width() - 1,
                                   self.grid.height() - 1);

        self.grid.get_at(Some(end_coord))
                 .unwrap()
                 .min_total_losses
                 .iter()
                 .copied()
                 .min()
                 .unwrap()
    }
}

mod part_1 {

    use super::*;    

    fn get_result(input: &str) -> usize { City::parse(input).min_loss() }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 102); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 851); }
}