use std::cell::Cell;

use crate::rect::Rect;

pub struct DrawSurface {
	buffers : (Vec<u32>, Vec<u32>),
	size: Rect,
}

impl DrawSurface {

	pub fn set_pixel(&mut self, color : u32, x : isize, y : isize) {

		let size = self.get_size();
		let buffer = self.get_buffer_mut();
		
		let zero_check_x = if x > 0 {x as usize}
		else {0};


		let zero_check_y = if y > 0 {y as usize}
		else {0};
		
		if size.x * size.y > zero_check_x * zero_check_y {
			buffer[size.x * zero_check_y + zero_check_x] = color;
		} 

	}

	pub fn new(rect: Rect) -> Self {

		let buffer = vec![0; rect.x * rect.y];
		let buffer_depth_indexes = vec![0; rect.x * rect.y];
		DrawSurface { 
			size: rect, 
			buffers : (buffer, buffer_depth_indexes)
		}
	}


	pub fn get_buffer(&self) -> &Vec<u32>  {
		&self.buffers.0
	}

	pub fn get_buffer_mut(&mut self) -> &mut Vec<u32>  {
		&mut self.buffers.0
	}

	pub fn get_buffer_index(&self) -> &Vec<u32> {
		&self.buffers.1
	}
	
	pub fn get_buffer_index_mut(&mut self) -> &mut Vec<u32> {
		&mut self.buffers.1
	}
	pub fn get_buffers_mut(&mut self) -> &mut (Vec<u32>, Vec<u32>) {
		&mut self.buffers
	}



	pub fn get_size(&self) -> Rect {
		self.size
	}
}
