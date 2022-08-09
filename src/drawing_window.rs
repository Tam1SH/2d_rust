extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crate::draw_surface::DrawSurface;
use crate::rect::Rect;
use crate::shape::Shape;
use crate::update_window_observer::UpdateWindowObserver;

pub struct DrawingWindow {
	window : Window,
	surface: DrawSurface,
	objects : Vec<Rc<RefCell<dyn Shape>>>,
	observers : Vec<Weak<RefCell<dyn UpdateWindowObserver>>>,
}


pub trait ObservableWindow {
	fn add_observer(&mut self, o : Weak<RefCell<dyn UpdateWindowObserver>>);
	fn remove_observer(&mut self, o : Weak<RefCell<dyn UpdateWindowObserver>>);
	fn notify(&self);
}

impl DrawingWindow {

	pub fn add_object(&mut self, object : Rc<RefCell<dyn Shape>>) {
		self.objects.push(object.clone());

	}

	pub fn new(title : String, rect: Rect, options : WindowOptions) -> Self {
		let surface = DrawSurface::new(rect);

		let mut window = Window::new(
			title.as_str(),
			rect.x,
			rect.y,
			options,
			)
			.unwrap_or_else(|e| {
				panic!("{}", e);
		});
		
		window.limit_update_rate(Some(std::time::Duration::from_millis(10)));
		
		DrawingWindow { window, surface, objects : vec![], observers : vec![] }
	}
	
	pub fn run(&mut self) {

		while self.window.is_open() && !self.window.is_key_down(Key::Escape) {

			for object in &self.objects {
				object.borrow_mut().draw(&mut self.surface);
			}

			self.notify();
			let mut buffer = self.surface.get_buffer();


			self.window
				.update_with_buffer(&buffer, 
					self.window.get_size().0,
					self.window.get_size().1)
				.unwrap();

			for i in buffer.iter_mut() {
				*i = 0;
			}
		}

	}

}

impl ObservableWindow for DrawingWindow {

	fn add_observer(&mut self, o : Weak<RefCell<dyn UpdateWindowObserver>>) {
		self.observers.push(o);
	}

	fn remove_observer(&mut self, o : Weak<RefCell<dyn UpdateWindowObserver>>) {
		self.observers.iter()
			.position(|x| x.ptr_eq(&o))
			.and_then(|index| Some({
				self.observers.remove(index);
			})).unwrap();
		
	}

	fn notify(&self) {
		for o in &self.observers {
			let o = o.upgrade();
			o.and_then(|o| Some({
				o.borrow_mut().on_update();
			})).unwrap();
		}
	}
}

