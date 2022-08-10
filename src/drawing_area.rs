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
		let x = if self.left_top.x < 0 {
			0
		}
		else {
			self.left_top.x as usize
		};
		let y = if self.left_top.y < 0 {
			0
		}
		else {
			self.left_top.y as usize
		};

		let left_top = Rect {x, y};
		//println!("bounds({}, {})", bounds.x, bounds.y);
		//println!("left_top({}, {})", self.left_top.x, self.left_top.y);

		for i in left_top.y..bounds.y  {
			for j in left_top.x..bounds.x {
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