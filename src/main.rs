use macroquad::{prelude::*, rand};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const GRID_WIDTH: u32 = 40;
const GRID_HEIGHT: u32 = 40;
const CELL_WIDTH: u32 = WIDTH / GRID_WIDTH;
const CELL_HEIGHT: u32 = HEIGHT / GRID_HEIGHT;

#[macroquad::main("conway's game of life")]
async fn main() {
    // let w = screen_width() as usize;
    // let h = screen_height() as usize;

    let mut cells = vec![
        CellState::Dead;
        (GRID_WIDTH * GRID_HEIGHT) as usize
    ];
    let mut buffer = vec![
        CellState::Dead;
        (GRID_WIDTH * GRID_HEIGHT) as usize
    ];

    let mut image = Image::gen_image_color(
        WIDTH as u16,
        HEIGHT as u16,
        GRAY,
    );

    for cell in cells.iter_mut() {
        if rand::gen_range(0, 10) == 0 {
            *cell = CellState::Alive;
        }
    }

    let texture = Texture2D::from_image(&image);

    loop {
        request_new_screen_size(WIDTH as f32, HEIGHT as f32);
        clear_background(WHITE);
        draw_lines();

        for (i, cell) in cells.iter().enumerate() {
            if *cell == CellState::Alive {
                let x = i as u32 % GRID_WIDTH;
                let y = i as u32 / GRID_WIDTH;

                color_cell(x, y);
            }
        }

        next_frame().await;
    }
}

fn draw_lines() {
    for i in 1..40 {
        draw_line(
            20. * i as f32,
            0.,
            20. * i as f32,
            800.,
            1.,
            GRAY,
        );
        draw_line(
            0.0,
            20.0 * i as f32,
            800.0,
            20.0 * i as f32,
            1.0,
            GRAY,
        );
    }
}

fn color_cell(x: u32, y: u32) {
    assert!(x < 40);
    assert!(y < 40);

    draw_rectangle(
        (CELL_WIDTH * x) as f32,
        (CELL_HEIGHT * y) as f32,
        CELL_WIDTH as f32,
        CELL_HEIGHT as f32,
        PINK,
    );
}
