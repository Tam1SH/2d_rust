mod simple2d;
extern crate minifb;
use std::cell::RefCell;
use std::ffi::{CStr, c_void};
use std::os::raw::{c_uint, c_char};
use std::rc::Rc;

use minifb::WindowOptions;

use crate::simple2d::drawing_window::{DrawingWindow};
use crate::simple2d::rect::Rect;
use crate::simple2d::draw_surface::DrawSurface;
use crate::simple2d::shape::Shape;
use crate::simple2d::rectangle::Rectangle;
use crate::simple2d::position::Position;
use crate::simple2d::point::Point;
use crate::simple2d::position::PositionFrom;

#[no_mangle]
pub extern "C" fn create_rectangle(window: *mut c_void, a : c_uint, b : c_uint, x : c_uint, y : c_uint) -> *mut c_void {
	let window = window as *mut DrawingWindow;
	let a = a.try_into().unwrap();
	let b = b.try_into().unwrap();
	let x = x.try_into().unwrap();
	let y = y.try_into().unwrap();

	let rectangle = Rc::new(RefCell::new(
		Rectangle::new(
			Position::new(Point { x, y}), 
			Rect::new(a, b),
			Option::Some(0xFF)
		)
	));

	unsafe {
		if let Some(window) = window.as_mut() {
			window.add_object(rectangle.clone());
		}
	}

	Box::into_raw(Box::new(rectangle)) as *mut c_void
}

#[no_mangle]
pub extern "C" fn add_object(window: *mut c_void, object : *mut c_void) {
	let window = window as *mut DrawingWindow;
	let object = object as *mut Rc<RefCell<dyn Shape>>;
	unsafe {
		if let Some(window) = window.as_mut() {
			if let Some(object) = object.as_mut() {
				window.add_object(object.clone());
			}
		}
	}
}
#[no_mangle]
pub extern "C" fn create_window(name : *const c_char, x : c_uint, y : c_uint) -> *mut c_void {
	let name: &CStr = unsafe { CStr::from_ptr(name) };
	let name: &str = name.to_str().unwrap();
	let x = x.try_into().unwrap();
	let y = y.try_into().unwrap();

	

	let window = Box::new(DrawingWindow::new(
		name.to_string(),
		Rect::new(x,y),
		WindowOptions::default(),
	));
	Box::into_raw(window) as *mut c_void
}

#[no_mangle]
pub extern "C" fn unsubscribe_on_draw(window: *mut c_void, func : *mut c_void)  {
	let window = window as *mut DrawingWindow;
	let func = func as *mut Rc<Box<dyn Fn(&mut DrawSurface)>>;
	
	unsafe {
		if let Some(window) = window.as_mut() {
			if let Some(func )= func.as_mut() {
				window.unsubscribe_on_draw(func.to_owned());
			}
		}
	}

}

#[no_mangle]
pub extern "C" fn subscribe_on_draw(window: *mut c_void, func : extern fn() -> ()) -> *mut c_void {
	let window = window as *mut DrawingWindow;
//
	let on_draw: Rc<Box<dyn Fn(&mut DrawSurface)>> = Rc::new(Box::new(move |surface| {
		func();
	}));

	unsafe {
		if let Some(window) = window.as_mut() {
			window.subscribe_on_draw(on_draw.clone());
		}
	}

	Box::into_raw(Box::new(on_draw)) as *mut c_void
}

#[no_mangle]
pub extern "C" fn window_run(window: *mut c_void) {
	let mut window = unsafe { Box::from_raw(window as *mut DrawingWindow) };
	window.run();
	
}