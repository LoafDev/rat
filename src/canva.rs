use std::time::{Duration, Instant};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, style::Color, symbols::border, widgets::{canvas::{Canvas, Circle, Rectangle}, Block, Widget}};
use crate::Rat;

const OFFSET_ANIM: Duration = Duration::from_millis(100);

// viewport constants
const X_BOUNDS: [f64; 2] = [0., 150.];
const Y_BOUNDS: [f64; 2] = [0., 120.];

// 4 circles' constants
const CIRCLE_RADIUS: [f64; 2] = [20., 15.];
const CIRCLE_COLOUR: [Color; 2] = [Color::LightRed, Color::LightYellow];
const CIRCLE_COORD: [[f64; 2]; 4] = [
    [X_BOUNDS[0], Y_BOUNDS[0]],
    [X_BOUNDS[1], Y_BOUNDS[0]],
    [X_BOUNDS[0], Y_BOUNDS[1]],
    [X_BOUNDS[1], Y_BOUNDS[1]],
];

// middle rectangle constants
const MRECTANGLE_SIZE: [f64; 2] = [20., 30.];
const MRECTANGLE_COLOUR: Color = Color::Yellow;

// 4 rectangles' constants
const OFFSET: f64 = 0.8;
const OFFSET_LIMIT: f64 = OFFSET * ((Y_BOUNDS[1] - MRECTANGLE_SIZE[1]) / 2. - RECTANGLE_SIZE[1]);
const OFFSET_ARRAY: [[f64; 2]; 4] = [
    [0., OFFSET],
    [0., -OFFSET],
    [OFFSET, 0.],
    [-OFFSET, 0.]
];
const RECTANGLE_SIZE: [f64; 2] = [15., 20.];
const RECTANGLE_COLOUR: Color = Color::LightMagenta;
const RECTANGLE_COORD: [[f64; 4]; 4] = [
    [(X_BOUNDS[1] - RECTANGLE_SIZE[0]) / 2., (Y_BOUNDS[1] + MRECTANGLE_SIZE[1]) / 2., RECTANGLE_SIZE[0], RECTANGLE_SIZE[1]],
    [(X_BOUNDS[1] - RECTANGLE_SIZE[0]) / 2., (Y_BOUNDS[1] - MRECTANGLE_SIZE[1]) / 2. - RECTANGLE_SIZE[1], RECTANGLE_SIZE[0], RECTANGLE_SIZE[1]],
    [(X_BOUNDS[1] + MRECTANGLE_SIZE[0]) / 2., (Y_BOUNDS[1] - RECTANGLE_SIZE[0]) / 2., RECTANGLE_SIZE[1], RECTANGLE_SIZE[0]],
    [(X_BOUNDS[1] - MRECTANGLE_SIZE[0]) / 2. - RECTANGLE_SIZE[1], (Y_BOUNDS[1] - RECTANGLE_SIZE[0]) / 2., RECTANGLE_SIZE[1], RECTANGLE_SIZE[0]],
];

impl Rat {
    fn change_offset(&mut self, mul: f64) {
        for i in 0..4 {
            for j in 0..2 {
                self.canva_offset[i][j] += OFFSET_ARRAY[i][j] * mul;
            }
        }
    }

    pub fn handle_canva(&mut self, kv: KeyEvent) {
        match kv.code {
            KeyCode::Up if !self.toggle_offset => self.change_offset(1.),
            KeyCode::Down if !self.toggle_offset => self.change_offset(-1.),
            KeyCode::Tab => self.toggle_offset = !self.toggle_offset,
            _ => {}
        }
    }

    pub fn render_canva(&mut self, area: Rect, buf: &mut Buffer) {
        Canvas::default()
            .block(Block::bordered().border_set(border::ROUNDED).title("A simple canvas demonstration"))
            .x_bounds(X_BOUNDS)
            .y_bounds(Y_BOUNDS)
            .paint(|ctx| {

                // Draw 4 at 4 corners
                for i in CIRCLE_COORD {
                    ctx.draw(&Circle {
                        x: i[0],
                        y: i[1],
                        radius: CIRCLE_RADIUS[0],
                        color: CIRCLE_COLOUR[0]
                    });

                    ctx.draw(&Circle {
                        x: i[0],
                        y: i[1],
                        radius: CIRCLE_RADIUS[1],
                        color: CIRCLE_COLOUR[1]
                    });
                }

                // draw a square at middle
                ctx.draw(&Rectangle {
                    x: (X_BOUNDS[1] - MRECTANGLE_SIZE[0]) / 2.,
                    y: (Y_BOUNDS[1] - MRECTANGLE_SIZE[1]) / 2.,
                    width: MRECTANGLE_SIZE[0],
                    height: MRECTANGLE_SIZE[1],
                    color: MRECTANGLE_COLOUR
                });

                // draw 4 rectangles around middle
                for (j,i) in RECTANGLE_COORD.iter().enumerate() {
                    ctx.draw(&Rectangle {
                        x: i[0] + self.canva_offset[j][0],
                        y: i[1] + self.canva_offset[j][1],
                        width: i[2],
                        height: i[3],
                        color: RECTANGLE_COLOUR
                    });
                }

            })
            .render(area, buf)
        ;

        if self.last_tick.elapsed() >= OFFSET_ANIM && self.toggle_offset {
            if self.canva_offset[0][1] >= OFFSET_LIMIT { self.offset_direction = true; }
            else if self.canva_offset[0][1] <= 0. { self.offset_direction = false; }

            if !self.offset_direction { self.change_offset(1.); }
            else { self.change_offset(-1.); }

            self.last_tick = Instant::now();
        }
    }
}
