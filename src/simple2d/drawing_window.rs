extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::rc::Rc;
use std::cell::RefCell;

use super::draw_surface::DrawSurface;
use super::rect::Rect;
use super::shape::Shape;


pub struct DrawingWindow {
	window : Window,
	surface: DrawSurface,
	objects : Vec<Rc<RefCell<dyn Shape>>>,
	observers : Vec<Rc<Box<dyn Fn(&mut DrawSurface)>>>,
}


impl DrawingWindow {

	pub fn get_screen_size(&self) -> Rect {
		let size = self.window.get_size();
		Rect { x : size.0, y : size.1 }
	}

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
		
		window.limit_update_rate(Some(std::time::Duration::from_millis(16)));
		
		DrawingWindow { window, surface, objects : vec![], observers : vec![] }
	}
	
	pub fn run(&mut self) {

		while self.window.is_open() && !self.window.is_key_down(Key::Escape) {

			for object in &self.objects {
				object.borrow_mut().draw(&mut self.surface);
			}

			self.update();
			
			let buffer = self.surface.get_buffer_mut();

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

	pub fn subscribe_on_draw<'a>(&mut self, o : Rc<Box<dyn Fn(&mut DrawSurface)>>) {
		self.observers.push(o.clone());
	}
	pub fn unsubscribe_on_draw(&mut self, o : Rc<Box<dyn Fn(&mut DrawSurface)>>) {
		let index = self.observers.iter().position(|x| Rc::ptr_eq(x, &o)).unwrap();
		self.observers.remove(index);
	}

	pub fn update(&mut self) {
		for o in &self.observers {
			o(&mut self.surface);
		}
	}

}
