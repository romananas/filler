use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::{Duration, Instant};

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;
const CELL_SIZE: u32 = 50;

// Grid
struct Grid {
    values: Vec<Vec<u32>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            values: vec![vec![0; width]; height], // default color green, 0 is green
        }
    }

    // Change cell color by value
    fn set_cell(&mut self, x: usize, y: usize, value: u32) {
        self.values[y][x] = value; // 1 for red, 2 for blue
    }

    // draw the grid with the associate color
    fn draw(&self, canvas: &mut Canvas<Window>) {
        for (y, row) in self.values.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                let rect = Rect::new(
                    (x as u32 * CELL_SIZE) as i32,
                    (y as u32 * CELL_SIZE) as i32,
                    CELL_SIZE,
                    CELL_SIZE,
                );

                // Choose the color
                let color = match value {
                    1 => Color::RGB(255, 0, 0), // red
                    2 => Color::RGB(0, 0, 255), // blue
                    _ => Color::RGB(0, 255, 0), // green
                };

                // draw cell
                canvas.set_draw_color(color);
                canvas.fill_rect(rect).unwrap();

                // draw border
                canvas.set_draw_color(Color::RGB(0, 0, 0)); // Noir pour les bordures
                canvas.draw_rect(rect).unwrap();
            }
        }
    }
}

// player struct
#[derive(PartialEq)]
struct Player {
    id: u32,
}

impl Player {
    fn make_move(&self, grid: &mut Grid, x: usize, y: usize) {
        grid.set_cell(x, y, self.id);
    }
}

// Fonction pour gérer les mouvements des joueurs
fn handle_player_move(player: &Player, grid: &mut Grid, x: usize, y: usize) {
    player.make_move(grid, x, y); // Mettre à jour la grille avec le mouvement du joueur
}


fn test(grid: &mut Grid, player1: &Player) {
    grid.set_cell(5, 5, player1.id);
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

    // grid init
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_update = Instant::now();

    // player init
    let player1 = Player { id: 1 }; // player 1 red
    let player2 = Player { id: 2 }; // player 2 bue

    // red start
    let mut current_player = &player1;

    test(&mut grid, &player2);


    'running: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                break 'running;
            }

            //// we can test with click if player priority, player 1 and after player 2 work
            //if let sdl2::event::Event::MouseButtonDown { x, y, .. } = event {
            //    // Calculer les coordonnées de la cellule cliquée
            //    let grid_x = (x as usize) / CELL_SIZE as usize;
            //    let grid_y = (y as usize) / CELL_SIZE as usize;
            //
            //    handle_player_move(current_player, &mut grid, grid_x, grid_y);
            //
            //    current_player = if current_player == &player1 { &player2 } else { &player1 };
            //}
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        grid.draw(&mut canvas);

        canvas.present();
    }
}
