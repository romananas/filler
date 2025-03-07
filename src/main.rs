use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::{Duration, Instant};

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;
const CELL_SIZE: u32 = 50;

struct Grid {
    values: Vec<Vec<u32>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            values: vec![vec![0; width]; height], // default value green and 0
        }
    }


    // make it red and the value 1
    fn set_red_cell(&mut self, x: usize, y: usize) {
        self.values[y][x] = 1; // 1 pour rouge
    }

    // draw the grid
    fn draw(&self, canvas: &mut Canvas<Window>) {
        for (y, row) in self.values.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                let rect = Rect::new(
                    (x as u32 * CELL_SIZE) as i32,
                    (y as u32 * CELL_SIZE) as i32,
                    CELL_SIZE,
                    CELL_SIZE,
                );


                // color by value
                let color = if value == 1 {
                    Color::RGB(255, 0, 0) // red
                } else {
                    Color::RGB(0, 255, 0) // green
                };

                // draw cell
                canvas.set_draw_color(color);
                canvas.fill_rect(rect).unwrap();

                // draw border
                canvas.set_draw_color(Color::RGB(0, 0, 0)); // black
                canvas.draw_rect(rect).unwrap();
            }
        }
    }
}

// update cell color every 1s
fn update_red_cell(grid: &mut Grid, last_update: &mut Instant) {
    if last_update.elapsed() > Duration::from_millis(1000) {
        grid.set_red_cell(4, 4); // position of red cell
        *last_update = Instant::now();
    }
}

fn main() {
    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Grille SDL2", GRID_WIDTH as u32 * CELL_SIZE, GRID_HEIGHT as u32 * CELL_SIZE)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    // grid is init
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_update = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                break 'running;
            }
        }

        update_red_cell(&mut grid, &mut last_update);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        grid.draw(&mut canvas);

        canvas.present();
    }
}
