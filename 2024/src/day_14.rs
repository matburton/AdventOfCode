
const INPUT: &str = include_str!("../input/day_14.txt");

const EXAMPLE: &str = "p=0,4 v=3,-3\n\
                       p=6,3 v=-1,-3\n\
                       p=10,3 v=-1,2\n\
                       p=2,0 v=2,-1\n\
                       p=0,0 v=1,3\n\
                       p=3,0 v=-2,-2\n\
                       p=7,6 v=-1,-3\n\
                       p=3,0 v=-1,-2\n\
                       p=9,3 v=2,3\n\
                       p=7,3 v=-1,2\n\
                       p=2,4 v=2,-3\n\
                       p=9,5 v=-3,-3";

struct Robot { offset: Offset, velocity: Offset }

#[derive(Clone, Copy)]
struct Offset { x: isize, y: isize }

impl From<(isize, isize)> for Offset {

    fn from((x, y): (isize, isize)) -> Self { Self { x, y } }
}

struct Zone { from: Offset, to: Offset } // Inclusive to exclusive

impl From<((isize, isize), (isize, isize))> for Zone {

    fn from((from, to): ((isize, isize), (isize, isize))) -> Self {
        
        Self { from: from.into(), to: to.into() }
    }
}

impl Zone {

    fn contains(&self, offset: Offset) -> bool {

           offset.x >= self.from.x && offset.y >= self.from.y
        && offset.x < self.to.x && offset.y < self.to.y
    }
}

impl Robot {

    fn parse(line: &str) -> Robot {

        let values = line.split(['=', ',', ' '])
                         .enumerate()
                         .filter(|&(i, _)| i % 3 != 0)
                         .map(|(_, f)| f.parse().unwrap())
                         .collect::<Vec<_>>();

        Robot { offset:   (values[0], values[1]).into(),
                velocity: (values[2], values[3]).into() } }
}

fn safety_factor(robots: &[Robot], room: Offset) -> usize {

    let quadrants: [Zone; 4] = [
        ((0, 0), (room.x / 2, room.y / 2)).into(),
        ((room.x / 2 + 1, 0), (room.x, room.y / 2)).into(),
        ((0, room.y / 2 + 1), (room.x / 2, room.y)).into(),
        ((room.x / 2 + 1, room.y / 2 + 1), (room.x, room.y)).into()
    ];

    let count_robots = |z: &Zone|
        robots.iter().filter(|r| z.contains(r.offset)).count();

    quadrants.iter().map(count_robots).product()
}

mod part_1 {

    use super::*;

    fn get_result(input: &str, room: Offset) -> usize {

        let mut robots =
            input.split('\n').map(Robot::parse).collect::<Vec<_>>();

        let modulo = |lhs, rhs|
            match lhs % rhs { v @ .. 0 => v + rhs, v => v };

        for robot in robots.iter_mut() {

            robot.offset = Offset {
                x: modulo(robot.offset.x + robot.velocity.x * 100, room.x),
                y: modulo(robot.offset.y + robot.velocity.y * 100, room.y)
            }
        }

        safety_factor(&robots, room)
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE, (11, 7).into()), 12); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT, (101, 103).into()), 211773366); }
}