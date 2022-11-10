use std::ops;
use super::point::Point;

#[derive(Clone, Copy)]
pub struct Position {
	pub left : isize,
	pub top : isize,
	pub z : usize,
}

impl ops::Sub<Position> for Position { 
	type Output = Position;
	fn sub(self, rhs: Position) -> Self::Output {
		Position {
			left : self.left - rhs.left,
			top : self.top - rhs.top,
			z  : self.z - rhs.z,
		}
	}
}


pub trait PositionFrom<T> {
	fn new(t : T) -> Position;
}
pub trait PositionFrom2<T, U> {
	fn new(t : T, u : U) -> Position;
}

impl PositionFrom<Point> for Position {
	fn new(pos : Point) -> Self {
		Position {
			left : pos.x,
			top: pos.y,
			z : 1,
		}
	}
}

impl PositionFrom2<Point, usize> for Position {
	fn new(pos : Point, z : usize) -> Self {
		Position {
			left : pos.x,
			top: pos.y,
			z : z,
		}
	}
}


impl PositionFrom<(isize, isize)> for Position {
	fn new(pos : (isize, isize)) -> Self {
		Position {
			left : pos.0,
			top: pos.1,
			z : 1,
		}
	}
}

impl PositionFrom<(isize, isize, usize)> for Position {
	fn new(pos : (isize, isize, usize)) -> Self {
		Position {
			left : pos.0,
			top: pos.1,
			z : pos.2,
		}
	}
}
