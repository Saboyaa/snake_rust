extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, ButtonArgs, EventLoop};
use crate::piston::ButtonEvent;
use piston::ButtonState;
use piston::Key;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend. 
    snake: Snake,
    x: f64,
    y: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
        });
        self.snake.render(&mut self.gl, args);
    }

    fn update(&mut self) {
        self.snake.update();
    }
    fn button(&mut self,btn : &Button) {
        let last_direction = self.snake.direction.clone();
        self.snake.direction = match btn {
            &Button::Keyboard(Key::Up) => Direction::Up,
            &Button::Keyboard(Key::Down) => Direction::Down,
            &Button::Keyboard(Key::Right) => Direction::Right,
            &Button::Keyboard(Key::Left) => Direction::Left,
            _ => last_direction,
        }
    }
}

#[derive(Clone)]
struct Snake {
    pos_x: f64,
    pos_y: f64,
    direction: Direction,
}
impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square((self.pos_x * 50.0),(self.pos_y * 50.0), 50.0);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self) {
        match self.direction {
            Direction::Left => self.pos_x += -1.0,
            Direction::Right => self.pos_x += 1.0,
            Direction::Up => self.pos_y += -1.0,
            Direction::Down => self.pos_y += 1.0,
        }

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
    let mut window: Window = WindowSettings::new("snake game", [1500, 1500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    // Create a new game and run it.

    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake{pos_x: 0.0, pos_y:0.0,direction:Direction::Right},
        x: 0.0,
        y: 0.0,
    };
    let mut events = Events::new(EventSettings::new()).ups(4);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update();
        }
        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                app.button(&args.button);
            };

        }
    }
}
