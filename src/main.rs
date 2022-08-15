extern crate minifb;
mod draw_surface;
mod drawing_area;
mod position;
mod rectangle;
mod rect;
mod shape;
mod update_window_observer;
mod point;
mod drawing_window;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

use drawing_window::{ObservableWindow, DrawingWindow};
use point::Point;
use position::{PositionFrom, Position};
use minifb::WindowOptions;
use rect::Rect;
use rectangle::Rectangle;
use shape::Shape;
use update_window_observer::UpdateWindowObserver;




fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
	let (r, g, b) = (r as u32, g as u32, b as u32);
	(r << 16) | (g << 8) | b
}

struct TestObserver;

impl UpdateWindowObserver for TestObserver {
	fn on_update(&mut self) {
		
	}
}


fn main() {

	let mut window = DrawingWindow::new(
		"BETA EPTA".to_string(),
		Rect::new(640,360),
		WindowOptions::default(),
	);
	
	let size = window.get_screen_size();
	let mut vec = vec![];

	for i in 0..35 {
			let rect = Rc::new(RefCell::new(
				Rectangle::new(
					Position::new(Point { x : 20 * i, y : i * 10}), 
					Rect::new(10, 10),
					Option::Some(0xFF)
			)));
			
		window.add_object(rect.clone());
		vec.push(rect);
		let rect = Rc::new(RefCell::new(
			Rectangle::new(
				Position::new(Point { x : size.x as isize - 20 * i, y : i * 10}), 
				Rect::new(10, 10),
				Option::Some(0x11)
			)));
		
		window.add_object(rect.clone());
		vec.push(rect);
	}

	window.on_draw(Box::new(move |surface| {
		let vec = vec.clone();
		for obj in vec.iter() {
			let mut obj = obj.borrow_mut();
			let mut pos = obj.get_position();
			pos.left += 1;
			obj.set_position(pos);
		}
		let vec = vec![
			Point { 
				x : 10,
				y : 10,
			},
			Point { 
				x : 11,
				y : 10,
			},
			Point { 
				x : 12,
				y : 10,
			},
			Point { 
				x : 13,
				y : 10,
			},
			Point { 
				x : 14,
				y : 10,
			},
			Point { 
				x : 15,
				y : 10,
			},
			Point { 
				x : 16,
				y : 10,
			},
		];
		for p in &vec {
			surface.set_pixel(0xFFFFFF, p.x, p.y);
		}
	}));
	window.run();

}