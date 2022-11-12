use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

pub const BOARD_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const GRID_SPACE_COLOR_EMPTY: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; // RED
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0]; // BLUE

use crate::ui::{Render, RenderCoords};

#[derive(PartialEq, Clone, Debug)]
pub enum Mark {
    RED,
    BLUE,
}

#[derive(PartialEq)]
pub struct BoardGridSpace {
    pos_x_y: [i8; 2],
    render_coords: RenderCoords,
    pub mark: Option<Mark>,
}

impl Render for BoardGridSpace {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) -> () {
        use graphics::*;

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(0.0, 0.0).rot_rad(0.0).trans(0.0, 0.0);

            let mut grid_space_color = GRID_SPACE_COLOR_EMPTY;
            match self.mark {
                Some(Mark::RED) => grid_space_color = RED,
                Some(Mark::BLUE) => grid_space_color = BLUE,
                _ => (),
            }

            rectangle(
                grid_space_color,
                rectangle::rectangle_by_corners(
                    self.render_coords.x0,
                    self.render_coords.y0,
                    self.render_coords.x1,
                    self.render_coords.y1,
                ),
                transform,
                &mut *gl,
            );
        });
    }
}

pub struct Board<'a> {
    pub state: Box<dyn BoardState + 'a>,// TODO: remove pub
    grid_spaces: Vec<BoardGridSpace>, 
    view_coords: [f64; 2],
    view_size: f64,
}

pub trait BoardState {
    fn get_grid_spaces(&self) -> [[Option<Mark>; 3]; 3];
    fn on_grid_space_click(&mut self, x: usize, y: usize) -> ();
}

impl<'a> Board<'a> {
    pub fn new(board_state: Box<dyn BoardState + 'a>) -> Board {
        let view_size = 400.00; //TODO: hardcoded value
        let view_coords = [50.0, 50.0]; //TODO: hardcoded value

        let board_grid_spaces: Vec<BoardGridSpace> = (0..9)
            .map(|i| {
                let x = i % 3;
                let y = i / 3;

                let render_coords = Board::calc_render_coords(view_size, view_coords, x, y);

                BoardGridSpace {
                    pos_x_y: [x, y],
                    render_coords,
                    mark: None,
                }
            })
            .collect();

        Board {
            state: board_state,
            grid_spaces: board_grid_spaces,
            view_size,
            view_coords,
        }
    }

    fn calc_render_coords(
        board_view_size: f64,
        view_coords: [f64; 2],
        grid_x_pos: i8,
        grid_y_pos: i8,
    ) -> RenderCoords {
        let board_size = board_view_size;

        let line_width = board_size as f64 * 0.01;

        let grid_space_width = board_size as f64 / 3.0 - line_width / 2.0;

        let x0 = view_coords[0] + line_width + grid_x_pos as f64 * grid_space_width;
        let y0 = view_coords[1] + line_width + grid_y_pos as f64 * grid_space_width;
        let x1 = x0 + grid_space_width - line_width;
        let y1 = y0 + grid_space_width - line_width;

        RenderCoords { x0, x1, y0, y1 }
    }

    // Returns true if the the click has added a mark
    pub fn handle_left_mouse_click(&mut self, coords: [f64; 2]) -> bool {
        let grid_space_being_hovered_option = self.grid_spaces.iter_mut().find(|grid_space| {
            let inside_x_axis = coords[0] >= grid_space.render_coords.x0
                && coords[0] <= grid_space.render_coords.x1;

            let inside_y_axis = coords[1] >= grid_space.render_coords.y0
                && coords[1] <= grid_space.render_coords.y1;

            return inside_x_axis && inside_y_axis;
        });

        if let Some(grid_space) = grid_space_being_hovered_option {
            //TODO: inverted?
            self.state.on_grid_space_click(grid_space.pos_x_y[1] as usize, grid_space.pos_x_y[0] as usize);
            true
        } else {
            false
        }
    }
}

impl<'a> Render for Board<'a> {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) -> () {
        use graphics::*;

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(0.0, 0.0).rot_rad(0.0).trans(0.0, 0.0);

            rectangle(
                BOARD_COLOR,
                rectangle::rectangle_by_corners(
                    self.view_coords[0],
                    self.view_coords[1],
                    self.view_coords[0] + self.view_size,
                    self.view_coords[1] + self.view_size,
                ),
                transform,
                &mut *gl,
            );

            self.grid_spaces
                .iter_mut()
                .enumerate()
                .for_each(|(i, grid_space)| {
                    let y = i % 3;
                    let x = i / 3;

                    grid_space.mark = self.state.get_grid_spaces()[x][y].clone();
                    grid_space.render(gl, args);
                });
        });
    }
}
