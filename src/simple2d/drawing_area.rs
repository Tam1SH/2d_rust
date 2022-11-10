use super::{position::Position, rect::Rect, point::Point, draw_surface::DrawSurface};

pub struct DrawingArea {
	position : Position,
	area : Rect,
	left_top : Point,
	right_bottom : Point,
}
pub enum Positioning {
	Relative, Absolute
}
impl DrawingArea {

	pub fn get_left_top(&self, positioning : Positioning) -> Point {
		match positioning {
			Positioning::Absolute => self.left_top,
			Positioning::Relative => {
				Point {
					x : self.left_top.x - self.position.left,
					y : self.left_top.y - self.position.top ,
				}
			},
		}
	}
	
	
	pub fn get_right_bottom(&self, positioning : Positioning) -> Point {
		match positioning {
			Positioning::Absolute => self.right_bottom,
			Positioning::Relative => {
				Point {
					x : self.right_bottom.x - self.position.left,
					y : self.right_bottom.y - self.position.top,
				}
			},
		}
	}


	fn _draw_with_once<T>(&self, drawing_area : Rect, f1 : T) 
	where T : FnOnce(Rect, Rect) {

		let bounds = self.calc_bounds(drawing_area);
		let point = self.get_left_top(Positioning::Absolute);
		let left_top = Rect::from_point(point);

		f1(left_top, bounds);
	}
	
	fn _draw<T>(&self, drawing_area : Rect, f: &mut T) where T : FnMut(usize,usize) {

		let bounds = self.calc_bounds(drawing_area);
		let left_top = Rect::from_point(self.get_left_top(Positioning::Absolute));

		for i in left_top.y..bounds.y  {
			for j in left_top.x..bounds.x {
				f(i, j);
			}
		}
	}
	
	pub fn draw_from_buffer(&self, drawing_area : Rect, draw_surface : &mut DrawSurface, buffer : &Vec<u32>) {

		let (_buffer, _) = draw_surface.get_buffers_mut();

		let self_area = self.get_area();
		let f = |left_top: Rect, right_bottom: Rect| { 

			for i in left_top.y..right_bottom.y  {
				for j in left_top.x..right_bottom.x {
					_buffer[drawing_area.x * i + j] = buffer[self_area.x * (i - left_top.y) + (j - left_top.x)];
				}
			}
		};

		self._draw_with_once(drawing_area, f);

		
	}

	pub fn draw<T>(&self, drawing_area : Rect, draw_surface : &mut DrawSurface, f: &T)
	where T : Fn(Rect, Rect, usize, usize, usize) -> Option<u32> {

		let (buffer, _) = draw_surface.get_buffers_mut();
		let left_top = Rect::from_point(self.get_left_top(Positioning::Absolute));
		let right_bottom = Rect::from_point(self.get_right_bottom(Positioning::Absolute));

		let mut f1 = |i: usize, j: usize| { 
			
			match f(left_top, right_bottom, 0, j, i) {
				Some(value) => buffer[drawing_area.x * i + j] = value,
				None => return,
			}
		};

		self._draw(drawing_area, &mut f1);
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
