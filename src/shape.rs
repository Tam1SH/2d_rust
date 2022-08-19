use crate::{draw_surface::DrawSurface, position::Position, drawing_area::DrawingArea, point::Point};


pub trait Shape {
	fn draw(&self, surface: &mut DrawSurface);
	fn move_to(&mut self, point : Point);
	fn set_position(&mut self, pos: Position);
	fn get_position(&self) -> Position;
	fn set_color(&mut self, color : u32);
	fn get_color(&self) -> u32;
	fn get_area(&self) -> &DrawingArea;
}
	