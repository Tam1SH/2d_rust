use std::ops;
use super::point::Point;
#[derive(Clone, Copy)]
pub struct Rect {
	pub x : usize,
	pub y : usize,
}

impl Rect {
	pub fn new(x : usize, y : usize) -> Rect { Rect { x, y } }
	pub fn from_point(point : Point) -> Rect {
		let x = if point.x < 0 {0}
		else { point.x as usize };
		
		let y = if point.y < 0 {0}
		else { point.y as usize };

		Rect {x, y}
	}
}

impl ops::Add<Rect> for Rect {

	type Output = Rect;

	fn add(self, rhs: Rect) -> Self::Output {
		Rect { 
			x: self.x + rhs.x,
			y: self.y + rhs.y,		
		}
	}

}
impl ops::Sub<Rect> for Rect { 
	type Output = Rect;
	fn sub(self, rhs: Rect) -> Self::Output {
		Rect {
			x : self.x - rhs.x,
			y : self.y - rhs.y,
		}
	}
}