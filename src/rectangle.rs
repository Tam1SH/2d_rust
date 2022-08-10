use crate::{drawing_area::DrawingArea, rect::Rect, position::Position, shape::Shape, draw_surface::DrawSurface, point::Point};


pub struct Rectangle {
	area : DrawingArea,
	color : u32,
}

impl Rectangle { 

	pub fn set_size(&mut self, size : Rect) { 
		self.area.resize(size);

	}


	pub fn new(pos: Position, size : Rect, color : Option<u32>) -> Self {
		Rectangle {
			color : color.unwrap_or(0xFFFFFF),
			area : DrawingArea::new(pos, size)
		}
	}
}


impl Shape for Rectangle {

	fn get_color(&self) -> u32 { self.color	}

	fn get_area(&self) -> &DrawingArea { &self.area }

	fn set_color(&mut self, color : u32) { self.color = color; }

	fn set_position(&mut self, pos: Position) { self.area.set_position(pos); }


	fn draw(&self, surface: &mut DrawSurface) {

		let drawing_area = surface.get_size();
		let buffer = surface.get_buffer();
		
		self.area.draw(drawing_area, buffer, &|pixel : &mut u32, x: usize, y: usize | {
			*pixel = self.color;
		});
		
	}
}