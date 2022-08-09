extern crate minifb;


use std::borrow::Borrow;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;


struct DrawingWindow {
    window : Window,
    surface: DrawSurface,
	objects : Vec<Rc<RefCell<dyn Shape>>>,
	observers : Vec<Weak<RefCell<dyn Observer>>>,
}

#[derive(Clone, Copy)]
struct Rect {
	x : usize,
	y : usize,
}

impl Rect {
	fn new(x : usize, y : usize) -> Rect { Rect { x, y } }
}

#[derive(Clone, Copy)]
struct Point {
    x : usize,
    y : usize,
}

#[derive(Clone, Copy)]
struct Position {
    left : usize,
    top : usize,
}


trait PositionFrom<T> {
    fn new(t : T) -> Position;
}

impl PositionFrom<Point> for Position {
    fn new(pos : Point) -> Self {
        Position {
            left : pos.x,
            top: pos.y
        }
    }
}

impl PositionFrom<(usize, usize)> for Position {
    fn new(pos : (usize, usize)) -> Self {
        Position {
            left : pos.0,
            top: pos.1
        }
    }
}


trait Shape {
    fn draw(&self, surface: &mut DrawSurface);
	fn set_position(&mut self, pos: Position);
	fn set_color(&mut self, color : u32);
	fn get_area(&self) -> &DrawingArea;
	fn get_area_mut(&mut self) -> &mut DrawingArea;
}


struct DrawingArea {
    position : Position,
    area : Rect,
}

impl DrawingArea {

	fn resize(new_size : Rect) {

	}
	fn move_to(new_pos : Position) {

	}

}

struct Reactangle {
	area : DrawingArea,
    position : Position,
    size : Rect,
	color : u32,
}

impl Reactangle { 

	fn set_size(&mut self, size : Rect) { 
		self.size = size;
	}

	
	fn new(pos: Position, size : Rect, color : Option<u32>) -> Self {
        Reactangle {
            position : pos,
            size,
			color : color.unwrap_or(0xFFFFFF),
			area : DrawingArea { position: pos, area: size }
        }
    }
}


impl Shape for Reactangle {

	fn get_area(&self) -> &DrawingArea {
		&self.area
	}
	fn get_area_mut(&mut self) -> &mut DrawingArea {
		&mut self.area
	}
	fn set_color(&mut self, color : u32) {
		self.color = color;
	}

	fn set_position(&mut self, pos: Position) {
		self.position = pos;
	}

    fn draw(&self, surface: &mut DrawSurface) {

		let size = surface.get_size();
        let buffer = surface.get_buffer();
		

        let mut bounds = (0, 0);

		let top = self.position.top;
		let left = self.position.left;

		bounds.0 = if self.size.x + top >= size.x { 
			size.x + top
		} 
		else  { 
			self.size.x + top 
		};

        bounds.1 = if self.size.y + left >= size.y {
            size.y + left
        }
        else { 
            self.size.y + left
        };
		
        for i in top..bounds.0  {
            for j in left..bounds.1 {
                buffer[size.x * i + j] = self.color;
            }
        }
		
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
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
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .unwrap();

			for i in buffer.iter_mut() {
				*i = 0;
			}
        }

    }

}

trait Observer {
	fn on_update(&mut self);
}

struct TestObserver;

impl Observer for TestObserver {
	fn on_update(&mut self) { }
}

trait Observable {
    fn add_observer(&mut self, o : Weak<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, o : Weak<RefCell<dyn Observer>>);
    fn notify(&self);
}



struct DrawSurface {
    buffer: Vec<u32>,
    size: Rect,
	
}

impl Observable for DrawingWindow {

	fn add_observer(&mut self, o : Weak<RefCell<dyn Observer>>) {
		self.observers.push(o);
	}

	fn remove_observer(&mut self, o : Weak<RefCell<dyn Observer>>) {
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

impl DrawSurface {

    pub fn new(rect: Rect) -> Self {

        let buffer = vec![0; rect.x * rect.y];
        DrawSurface {buffer, size: rect }
    }

    pub fn get_buffer(&mut self) -> &mut Vec<u32> {
        &mut self.buffer
    }

    pub fn get_size(&self) -> Rect {
        self.size
    }
}


fn main() {

    let mut window = DrawingWindow::new(
        "BETA EPTA".to_string(),
        Rect::new(WIDTH,HEIGHT),
        WindowOptions::default(),
    );


	for i in 0..10 {
			let rect = Rc::new(RefCell::new(
				Reactangle::new(
					Position::new(Point { x : 20, y : i * 10}), 
					Rect::new(10, 10),
					Option::Some(0x11FF00)
				)));
			
		window.add_object(rect);
	}
	let mut obs = vec![];
	for _ in 0..5 {
		let ob: Rc<RefCell<dyn Observer>> = Rc::new(RefCell::new(TestObserver{}));
		window.add_observer(Rc::downgrade(&ob));
		obs.push(ob);
	}

    window.run();


}