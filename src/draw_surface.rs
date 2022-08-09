use crate::rect::Rect;

pub struct DrawSurface {
	buffer: Vec<u32>,
	size: Rect,
	
}

impl DrawSurface {

	pub fn new(rect: Rect) -> Self {

		let buffer = vec![0; rect.x * rect.y];
		DrawSurface {buffer, size: rect }
	}

	pub fn get_buffer(&mut self) -> &mut Vec<u32> {
		&mut self.buffer
	}

	pub fn get_size(&self) -> Rect {
		self.size
	}
}
