use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
// use piston::Button;
use piston::GenericEvent;
use piston::RenderArgs;

use self::board_scene::BoardScene;

mod board_scene;

pub trait Render {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) -> ();
}

pub trait HandleEvent {
    fn handle_event<E: GenericEvent>(&mut self, event: E, gl: &mut GlGraphics) -> ();
}

#[derive(PartialEq)]
pub struct RenderCoords {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}


pub fn ui2() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("tic tac toe", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut board = BoardScene::new();

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        board.handle_event(e, &mut gl);
    }
}

