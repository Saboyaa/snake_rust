extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, EventLoop};
use crate::piston::ButtonEvent;
use piston::ButtonState;
use piston::Key;
use std::collections::LinkedList;
use std::iter::FromIterator;
use rand::prelude::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend. 
    snake: Snake,
    food: Food,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
        });
        self.snake.render(&mut self.gl, args);
        self.food.render(&mut self.gl, args);
    }

    fn update(&mut self) {
        self.snake.update();
    }
    fn button(&mut self,btn : &Button) {
        let last_direction = self.snake.direction.clone();
        self.snake.direction = match btn {
            &Button::Keyboard(Key::Up) => if last_direction!=Direction::Down {Direction::Up}else{last_direction},
            &Button::Keyboard(Key::Down) => if last_direction!=Direction::Up {Direction::Down}else{last_direction},
            &Button::Keyboard(Key::Right) => if last_direction!=Direction::Left {Direction::Right}else{last_direction},
            &Button::Keyboard(Key::Left) => if last_direction!=Direction::Right {Direction::Left}else{last_direction},
            _ => last_direction,
        }
    }
    fn colision_check(&mut self) {
        if self.snake.body.front() == Some(&(self.food.pos_x,self.food.pos_y)){
            self.snake.food_check = true;
            self.food.pos_x = rand::thread_rng().gen_range(0..30) as f64;
            self.food.pos_y = rand::thread_rng().gen_range(0..20) as f64;
        }
    }
}

struct Food {
    pos_x: f64,
    pos_y: f64,
}
impl Food {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(self.pos_x * 50.0,self.pos_y * 50.0, 50.0);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform;

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

}

#[derive(Clone)]
struct Snake {
    body: LinkedList<(f64,f64)>,
    direction: Direction,
    food_check: bool,
}
impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x,y)|{
                graphics::rectangle::square(x * 50.0,y * 50.0, 50.0)
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square|
                    rectangle(RED, square, transform, gl))
        });
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("No head")).clone();
        match self.direction {
            Direction::Left => new_head.0 += -1.0,
            Direction::Right => new_head.0 += 1.0,
            Direction::Up => new_head.1 += -1.0,
            Direction::Down => new_head.1 += 1.0,
        }
        self.body.push_front(new_head);
        if self.food_check == true {self.food_check = false} else {self.body.pop_back();};
    }

}
#[derive(Clone,PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("snake game", [1500, 1000])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    // Create a new game and run it.

    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake{body: LinkedList::from_iter((vec![(0.0,0.0)]).into_iter()),direction:Direction::Right,food_check:false},
        food: Food{pos_x:rand::thread_rng().gen_range(0..30) as f64,pos_y:rand::thread_rng().gen_range(0..20) as f64},
    };
    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(_args) = e.update_args() {
            app.update();
            app.colision_check();
        }
        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                app.button(&args.button);
            };

        }
    }
}
