use std::cell::{RefCell, Cell};

use crate::draw_surface::DrawSurface;
use crate::point::Point;
use crate::position::Position;
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
	
	pub fn draw(&self, drawing_area : Rect, draw_surface : &mut DrawSurface, f: &dyn Fn(usize,usize,usize) -> u32) {
		let bounds = self.calc_bounds(drawing_area);

		let x = if self.left_top.x < 0 {0}
		else { self.left_top.x as usize };
		
		let y = if self.left_top.y < 0 {0}
		else { self.left_top.y as usize };

		let left_top = Rect {x, y};


		let (buffer, _) = draw_surface.get_buffers_mut();

		for i in left_top.y..bounds.y  {
			for j in left_top.x..bounds.x {
				//indexes[drawing_area.x * i + j];
				// if(indexes[drawing_area.x * i + j] > self.position.z as u32) {

				// }
				buffer[drawing_area.x * i + j] = f(0, j, i);
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
			left_top : Point { x: 0, y: 0 },
			right_bottom : Point { x: 0, y: 0 }
		};
		this.calc_points();
		this
	}

	pub fn calc_bounds(&self, drawing_area_size : Rect) -> Rect {

		let mut bounds = Rect::new(0, 0);

		
		let left = self.left_top.x;
		let top = self.left_top.y;
		let zero_check = if (self.area.x as isize) + left > 0 {
			((self.area.x as isize) + left) as usize
		}
		else {
			0
		};

		bounds.x = if zero_check >= drawing_area_size.x { 
			drawing_area_size.x
		} 
		else  { 
			zero_check
		};

		let zero_check = if (self.area.y as isize) + left > 0 {
			((self.area.y as isize) + top) as usize
		}
		else {
			0
		};

		bounds.y = if zero_check >= drawing_area_size.y {
			drawing_area_size.y
		}
		else { 
			zero_check
		};
		bounds
	}

	fn calc_points(&mut self) {
		let half_x = self.area.x / 2;
		let half_y = self.area.y / 2;
		self.left_top.x = self.position.left - half_x as isize;
		self.left_top.y = self.position.top - half_y as isize;
		self.right_bottom.x = self.position.left + half_x as isize;
		self.right_bottom.y = self.position.top + half_y as isize;
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