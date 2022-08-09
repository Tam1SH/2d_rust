#[derive(Clone, Copy)]
pub struct Rect {
	pub x : usize,
	pub y : usize,
}

impl Rect {
	pub fn new(x : usize, y : usize) -> Rect { Rect { x, y } }
}
