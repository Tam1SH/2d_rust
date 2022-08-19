extern crate minifb;
mod draw_surface;
mod drawing_area;
mod position;
mod rectangle;
mod rect;
mod shape;
mod update_window_observer;
mod point;
mod ellipse;
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

use crate::ellipse::Ellipse;




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

	let mut vec = vec![1, 1, 1, 1, 1, 1];
	let slice = vec![2, 2, 2];
	
	vec.splice(1..3, slice.iter().cloned());
	println!("{:?}", vec);

	let mut window = DrawingWindow::new(
		"BETA EPTA".to_string(),
		Rect::new(640,360),
		WindowOptions::default(),
	);
	
	let size = window.get_screen_size();
	let mut vec: Vec<Rc<RefCell<dyn Shape>>> = vec![];

	for i in 0..1 {

		let ell = Rc::new(RefCell::new(
			Ellipse::new(
				Position::new(Point { x : 100, y : 100}), 
				Rect::new(50, 50),
				Option::Some(0xFF)
			)
		));
		window.add_object(ell.clone());
		vec.push(ell);
	}

	window.on_draw(Box::new(move |surface| {
		let vec = vec.clone();
		for obj in vec.iter() {
			let mut obj = obj.borrow_mut();
			//let mut pos = obj.get_position();
			//pos.left += 1;
			//obj.set_position(pos);
		}

	}));
	window.run();

}