use crate::point::Point;




#[derive(Clone, Copy)]
pub struct Position {
	pub left : usize,
	pub top : usize,
}


pub trait PositionFrom<T> {
	fn new(t : T) -> Position;
}

impl PositionFrom<Point> for Position {
	fn new(pos : Point) -> Self {
		Position {
			left : pos.x,
			top: pos.y
		}
	}
}

impl PositionFrom<(usize, usize)> for Position {
	fn new(pos : (usize, usize)) -> Self {
		Position {
			left : pos.0,
			top: pos.1
		}
	}
}
