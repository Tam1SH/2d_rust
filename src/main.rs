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
use update_window_observer::UpdateWindowObserver;




fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
	let (r, g, b) = (r as u32, g as u32, b as u32);
	(r << 16) | (g << 8) | b
}


struct TestObserver;

impl UpdateWindowObserver for TestObserver {
	fn on_update(&mut self) { }
}

fn main() {

	let mut window = DrawingWindow::new(
		"BETA EPTA".to_string(),
		Rect::new(640,360),
		WindowOptions::default(),
	);


	for i in 0..10 {
			let rect = Rc::new(RefCell::new(
				Rectangle::new(
					Position::new(Point { x : 20 * i, y : i * 10}), 
					Rect::new(10, 10),
					Option::Some(0x11FF00)
				)));
			
		window.add_object(rect);
	}

	let mut obs = vec![];

	for _ in 0..5 {
		let ob: Rc<RefCell<dyn UpdateWindowObserver>> = Rc::new(RefCell::new(TestObserver{}));
		window.add_observer(Rc::downgrade(&ob));
		obs.push(ob);
	}

	window.run();

	for i in 0..5 {
		window.remove_observer(Rc::downgrade(&obs[i]));
	}

}