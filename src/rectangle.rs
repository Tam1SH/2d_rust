use crate::{drawing_area::DrawingArea, rect::Rect, position::{Position, PositionFrom}, shape::Shape, draw_surface::DrawSurface, point::Point};


pub struct Rectangle {
	area : DrawingArea,
	color : u8,
}

impl Rectangle { 

	pub fn set_size(&mut self, size : Rect) { 
		self.area.resize(size);

	}

	
	pub fn new(pos: Position, size : Rect, color : Option<u8>) -> Self {
		
		Rectangle {
			color : color.unwrap_or(0xFF as u8),
			area : DrawingArea::new(pos, size)
		}
	}
}


impl Shape for Rectangle {

	fn get_color(&self) -> u8 { self.color	}

	fn get_area(&self) -> &DrawingArea { &self.area }

	fn set_color(&mut self, color : u8) { self.color = color; }

	fn set_position(&mut self, pos: Position) { self.area.set_position(pos); }

	fn get_position(&self) -> Position {
		self.area.get_position()
	}
	fn move_to(&mut self, point : Point) {
		let p = Position::new(point);
		self.set_position(p);
	}


	fn draw(&self, surface: &mut DrawSurface) {

		let drawing_area = surface.get_size();

		self.area.draw(drawing_area, surface, &|z_index, x, y | {
			0x11FFFFF
		});
		
	}
}

fn g(d : &mut Vec<u32>, a : &mut Vec<u32>) {

}