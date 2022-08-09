use crate::{drawing_area::DrawingArea, rect::Rect, position::Position, shape::Shape, draw_surface::DrawSurface};


pub struct Rectangle {
	area : DrawingArea,
	position : Position,
	size : Rect,
	color : u32,
}

impl Rectangle { 

	pub fn set_size(&mut self, size : Rect) { 
		self.size = size;
	}


	pub fn new(pos: Position, size : Rect, color : Option<u32>) -> Self {
		Rectangle {
			position : pos,
			size,
			color : color.unwrap_or(0xFFFFFF),
			area : DrawingArea { position: pos, area: size }
		}
	}
}


impl Shape for Rectangle {

	fn get_area(&self) -> &DrawingArea {
		&self.area
	}
	fn get_area_mut(&mut self) -> &mut DrawingArea {
		&mut self.area
	}
	fn set_color(&mut self, color : u32) {
		self.color = color;
	}

	fn set_position(&mut self, pos: Position) {
		self.position = pos;
	}

	fn draw(&self, surface: &mut DrawSurface) {

		let size = surface.get_size();
		let buffer = surface.get_buffer();
		

		let mut bounds = (0, 0);

		let top = self.position.top;
		let left = self.position.left;

		bounds.0 = if self.size.x + top >= size.x { 
			size.x + top
		} 
		else  { 
			self.size.x + top 
		};

		bounds.1 = if self.size.y + left >= size.y {
			size.y + left
		}
		else { 
			self.size.y + left
		};
		
		for i in top..bounds.0  {
			for j in left..bounds.1 {
				buffer[size.x * i + j] = self.color;
			}
		}
		
	}
}