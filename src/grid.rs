
#[derive(Debug, Clone, Copy)]
pub enum Direction { Up, Down, Left, Right }

use Direction::*;

impl std::fmt::Display for Direction {

    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        std::fmt::Debug::fmt(self, formatter)
    }
}

impl std::ops::Not for Direction {

    type Output = Direction;

    fn not(self) -> Direction {
        
        match self { Up    => Down,
                     Down  => Up,
                     Left  => Right,
                     Right => Left }
    }
}

#[derive(Clone, Copy)]
pub struct Coord { pub x: usize, pub y: usize }

impl Coord { pub fn new(x: usize, y: usize) -> Self { Self { x, y } } }

impl std::fmt::Display for Coord {

    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Debug for Coord {

    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        std::fmt::Display::fmt(self, formatter)
    }
}

impl From<(usize, usize)> for Coord {

    fn from(tuple: (usize, usize)) -> Self { Self { x: tuple.0, y: tuple.1 } }   
}

impl std::ops::Add<Direction> for Coord {

    type Output = Option<Self>;

    fn add(self, direction: Direction) -> Option<Self> {
        
        let (x, y) = match direction { Up    => ( 0, -1),
                                       Down  => ( 0,  1),
                                       Left  => (-1,  0),
                                       Right => ( 1,  0)};
        self.x.checked_add_signed(x)
              .and_then(|x| self.y.checked_add_signed(y)
                                  .map(|y| Coord { x, y }))
    }
}

impl std::ops::Add<Direction> for Option<Coord> {

    type Output = Self;

    fn add(self, direction: Direction) -> Self { self? + direction }
}

impl std::ops::AddAssign<Direction> for Option<Coord> {

    fn add_assign(&mut self, direction: Direction) {
        
        if let Some(c) = *self { *self = c + direction; }
    }
}

#[derive(Hash)]
pub struct Grid<T> { cells: Vec<Vec<T>> }

impl<T> Grid<T> {

    pub fn parse(text: &str, parse_char: impl Fn(char) -> Option<T>)
        -> Result<Self, String> {

        let parse_char = |char|
            parse_char(char).ok_or_else(|| format!("Can't parse '{}'", char));

        let cells = text.split('\n')
                        .map(|l| l.chars().map(parse_char).collect())
                        .collect::<Result<Vec<_>, _>>()?;

        if cells.len() > 1
        && cells[1 ..].iter().any(|v: &Vec<_>| v.len() != cells[0].len()) {

            return Err("Grid width was insonsistent".to_string());
        }

        Ok(Self { cells })
    }

    pub fn width(&self) -> usize {
        
        match &self.cells[..] { [v, ..] => v.len(), [] => 0 }
    }

    pub fn height(&self) -> usize { self.cells.len() }

    pub fn in_bounds(&self, coord: Coord) -> bool {

        coord.x < self.width() && coord.y < self.height()
    }

    pub fn get_at(&mut self, coord: Coord) -> Option<&T> {

        self.cells.get(coord.y).and_then(|v| v.get(coord.x))
    }

    pub fn get_at_mut(&mut self, coord: Coord) -> Option<&mut T> {

        self.cells.get_mut(coord.y).and_then(|v| v.get_mut(coord.x))
    }

    pub fn get_two_at_mut(&mut self, a: Option<Coord>, b: Option<Coord>)
        -> Option<(&mut T, &mut T)> {

        let (a, b) = a.zip(b)?;

        use std::cmp::{ Ordering::*, max, min };

        if !self.in_bounds(a) || !self.in_bounds(b) { return None; }

        Some(if a.y == b.y {

            let slices = self.cells[a.y].split_at_mut(max(a.x, b.x));

            let tuple = (&mut slices.0[min(a.x, b.x)], &mut slices.1[0]);

            match a.x.cmp(&b.x) {
                Equal   => panic!("get_two_at_mut with same coord {}", a),
                Greater => (tuple.1, tuple.0),
                Less    => tuple
            }
        }
        else {

            let slices = self.cells.split_at_mut(max(a.y, b.y));

            let tuple = (&mut slices.0[min(a.y, b.y)], &mut slices.1[0]);

            if a.y > b.y { (&mut tuple.1[a.x], &mut tuple.0[b.x]) }
                    else { (&mut tuple.0[a.x], &mut tuple.1[b.x]) }
        })
    }

    pub fn iter(&self) -> GridIterator<T> {

        GridIterator { grid: self, x: 0, y: 0 }
    }
}

impl<T: std::hash::Hash> Grid<T> {

    pub fn get_hash(&self) -> u64 {

        use std::hash::Hasher;

        let mut hasher = std::hash::DefaultHasher::new();

        std::hash::Hash::hash(self, &mut hasher);

        hasher.finish()
    }
}

pub struct GridIterator<'a, T> { grid: &'a Grid<T>, x: usize, y: usize }

impl<'a, T> Iterator for GridIterator<'a, T> {

    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        
        if self.x >= self.grid.width() { self.y += 1; self.x = 0; }

        if self.y >= self.grid.height() { return None; }

        let x = self.x;

        self.x += 1;

        Some(((x, self.y).into(), &self.grid.cells[self.y][x]))
    }
}