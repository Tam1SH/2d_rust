mod simple2d;
extern crate minifb;
use crate::simple2d::drawing_window::{DrawingWindow};
use crate::simple2d::ellipse::Ellipse;
use crate::simple2d::position::PositionFrom;
use crate::simple2d::point::Point;
use crate::simple2d::position::Position;
use crate::simple2d::shape::Shape;
use crate::simple2d::rect::Rect;

use core::cell::RefCell;
use std::rc::Rc;
use minifb::WindowOptions;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
	let (r, g, b) = (r as u32, g as u32, b as u32);
	(r << 16) | (g << 8) | b
}



fn main() {

	let mut vec = vec![1, 1, 1, 1, 1, 1];
	let slice = vec![2, 2, 2];
	
	vec.splice(1..3, slice.iter().cloned());
	println!("{:?}", vec);
	let a = 10;
	println!("{a}");
	
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

	window.subscribe_on_draw(Rc::new(Box::new(move |surface| {
		let vec = vec.clone();
		for obj in vec.iter() {
			let mut obj = obj.borrow_mut();
			let mut pos = obj.get_position();
			pos.left += 1;
			obj.set_position(pos);
		}

	})));
	window.run();

}