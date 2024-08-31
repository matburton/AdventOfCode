
use std::fmt::{ Debug, Display, Formatter };

#[derive(Debug, Clone, Copy)]
pub enum Direction { Up, Down, Left, Right }

use Direction::*;

impl Display for Direction {

    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        
        Debug::fmt(self, formatter)
    }
}

#[derive(Clone, Copy)]
pub struct Coord { pub x: usize, pub y: usize }

#[derive(Hash)]
struct Grid<T> { cells: Vec<Vec<T>> }

impl Direction {

    pub fn reversed(self) -> Direction {

        match self { Up    => Down,
                     Down  => Up,
                     Left  => Right,
                     Right => Left }
    }
}

impl Display for Coord {

    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

impl Debug for Coord {

    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        
        Display::fmt(self, formatter)
    }
}

impl Coord {
    
    pub fn stepped(self, direction: Direction) -> Result<Coord, String> {

        let (x, y) = match direction { Up    => ( 0, -1),
                                       Down  => ( 0,  1),
                                       Left  => (-1,  0),
                                       Right => ( 1,  0)};
        self.x.checked_add_signed(x)
              .and_then(|x| self.y.checked_add_signed(y)
                                  .map(|y| Coord { x, y }))
              .ok_or_else(|| format!("Can't go {} from {}", direction, self))
    }
}

impl<T> Grid<T> {

    fn parse(text: &str, parse_char: &impl Fn(char) -> Option<T>)
        -> Result<Self, String> {

        let parse_char = |char|
            parse_char(char).ok_or_else(|| format!("Can't parse '{}'", char));

        let cells = text.split('\n')
                        .map(|l| l.chars().map(parse_char).collect())
                        .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { cells })
    }

    fn width(&self) -> usize {
        
        match &self.cells[..] { [v, ..] => v.len(), [] => 0 }
    }

    fn height(&self) -> usize { self.cells.len() }

    fn in_bounds(&self, coord: Coord) -> bool {

        coord.x < self.width() && coord.y < self.height()
    }

    fn stepped(self, coord: Coord, direction: Direction)
        -> Result<Coord, String> {

        let coord = coord.stepped(direction)?;

        if !self.in_bounds(coord) {
            
            return Err(format!("{} from {} is out of bounds", direction, coord))
        }

        Ok(coord)
    }

    fn get_at(&mut self, coord: Coord) -> Option<&T> {

        self.cells.get(coord.y).and_then(|v| v.get(coord.x))
    }

    fn get_at_mut(&mut self, coord: Coord) -> Option<&mut T> {

        self.cells.get_mut(coord.y).and_then(|v| v.get_mut(coord.x))
    }

    fn get_two_at_mut(&mut self, (a, b): (Coord, Coord))
        -> Option<(&mut T, &mut T)> {

        None // TODO
    }
}

// TODO: Grid pretty print