use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use piston::RenderArgs;
use piston::RenderEvent;
use piston::{AfterRenderEvent, GenericEvent};

use crate::game::Game;
use crate::ui_state_controller::UiBoardStateController;

pub use self::board_scene::board::BoardState;
pub use self::board_scene::board::Mark;
use self::board_scene::BoardScene;

mod board_scene;

pub trait Render {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) -> ();
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

pub fn ui(game: &mut Game) {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("tic tac toe", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());

    loop {
        if game.has_anyone_won() != None {
            println!("done");
        } else {
            let board_state_controller = Box::new(UiBoardStateController::new(game));
            let mut scene = BoardScene::new(board_state_controller);

            while let Some(e) = events.next(&mut window) {
                scene.handle_event(e.clone(), &mut gl);

                if let Some(true) = e.after_render(|_| scene.ended()) {
                    break;
                }
            }
        }
    }
}
