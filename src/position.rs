use crate::point::Point;




#[derive(Clone, Copy)]
pub struct Position {
	pub left : isize,
	pub top : isize,
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

impl PositionFrom<(isize, isize)> for Position {
	fn new(pos : (isize, isize)) -> Self {
		Position {
			left : pos.0,
			top: pos.1
		}
	}
}
