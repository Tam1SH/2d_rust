use crate::{drawing_area::{DrawingArea, Positioning}, rect::Rect, position::{Position, PositionFrom}, shape::Shape, draw_surface::DrawSurface, point::Point};

pub struct Ellipse {
	area : DrawingArea,
	color : u32,
}
impl Ellipse {

	pub fn new(pos: Position, size : Rect, color : Option<u32>) -> Self {
		Ellipse {
			color : color.unwrap_or(0xFFFFFF),
			area : DrawingArea::new(pos, size)
		}
	}
}

impl Shape for Ellipse {
 	fn draw(&self, surface: &mut DrawSurface) {

		let rect = self.area.get_right_bottom(Positioning::Relative);
		let (width, height) = (rect.x, rect.y);

		for y in -height..height {
			for x in -width..width {
				if x*x*height*height+y*y*width*width <= height*height*width*width {
					let pos = self.get_position();
					surface.set_pixel(0xFF11, pos.left + x, pos.top + y);
				}
			}
		}
	}

	fn get_area(&self) -> &DrawingArea {
		&self.area
	}

	fn get_color(&self) -> u32 {
		self.color
	}

	fn get_position(&self) -> Position {
		self.area.get_position()
	}

	fn move_to(&mut self, point : Point) {
		let p = Position::new(point);
		self.set_position(p);
	}

	fn set_color(&mut self, color : u32) {
		self.color = color;
	}

	fn set_position(&mut self, pos: Position) {
		self.area.set_position(pos)
	}

}