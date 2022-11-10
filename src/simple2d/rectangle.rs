use super::draw_surface::DrawSurface;
use super::point::Point;
use super::position::{Position, PositionFrom};
use super::rect::Rect;
use super::shape::Shape;
use super::drawing_area::DrawingArea;

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

	fn get_position(&self) -> Position {
		self.area.get_position()
	}

	fn move_to(&mut self, point : Point) {
		let p = Position::new(point);
		self.set_position(p);
	}


	fn draw(&self, surface: &mut DrawSurface) {

		let drawing_area = surface.get_size();

		self.area.draw(drawing_area, surface,  &|_, _, _, _, _| {
			Some(0x11FFFFF)
		});
		
	}
}
