use crate::point::Point;
use crate::position::{Position, self};
use crate::rect::Rect;

pub struct DrawingArea {
	position : Position,
	area : Rect,
	left_top : Point,
	right_bottom : Point,
}

impl DrawingArea {

	pub fn get_left_top(&self) -> Point {
		self.left_top
		
	}
	pub fn get_right_bottom(&self) -> Point {
		
		self.right_bottom
	}
	
	pub fn draw(&self, drawing_area : Rect, buffer : &mut Vec<u32>, f: &dyn Fn(&mut u32,usize,usize) -> ()) {
		let bounds = self.calc_bounds(drawing_area);
		for i in self.left_top.y..bounds.y  {
			for j in self.left_top.x..bounds.x {
				f(&mut buffer[drawing_area.x * i + j], j, i);
			}
		}
	}
	

	pub fn set_position(&mut self, pos : Position) {
		self.position = pos;
		self.calc_points();	
	}
	

	pub fn get_area(&self) -> Rect {
		self.area
	}

	pub fn get_position(&self) -> Position { 
		self.position
	}

	pub fn new(position : Position, area : Rect) -> DrawingArea {
		let mut this = DrawingArea {
			area : area, 
			position : position,
			left_top : Point { x: position.left, y: position.top },
			right_bottom : Point { x: 0, y: 0 }
		};
		this.calc_points();
		this
	}

	pub fn calc_bounds(&self, drawing_area_size : Rect) -> Rect {

		let mut bounds = Rect::new(0, 0);

		
		let left = self.left_top.x;
		let top = self.left_top.y;

		bounds.x = if self.area.x + left >= drawing_area_size.x { 
			drawing_area_size.x + left
		} 
		else  { 
			self.area.x + left 
		};

		bounds.y = if self.area.y + top >= drawing_area_size.y {
			drawing_area_size.y + top
		}
		else { 
			self.area.y + top
		};
		bounds
	}

	fn calc_points(&mut self) {
		let half_x = self.area.x / 2;
		let half_y = self.area.y / 2;
		if (self.left_top.x as isize) - half_x as isize  >= 0 {
			self.left_top.x - half_x;
		}
		if (self.left_top.y as isize) - half_y as isize >= 0 {
			self.left_top.y -= half_y;
		}
		self.right_bottom.x += half_x;
		self.right_bottom.y += half_y;
	}
	pub fn resize(&mut self, new_size : Rect) {
		self.area = new_size;
		self.calc_points();
	}
	pub fn move_to(&mut self, new_pos : Position) {
		self.position = new_pos;
		self.calc_points();
	}

}