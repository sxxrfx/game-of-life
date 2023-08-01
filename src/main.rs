use std::ops::{Deref, DerefMut, Index, IndexMut};

use macroquad::{prelude::*, rand};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const GRID_WIDTH: u32 = 40;
const GRID_HEIGHT: u32 = 40;
const CELL_WIDTH: u32 = WIDTH / GRID_WIDTH;
const CELL_HEIGHT: u32 = HEIGHT / GRID_HEIGHT;

const NO_OF_CELLS: usize = (GRID_HEIGHT * GRID_WIDTH) as usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Dead,
    Alive,
}

impl Default for CellState {
    fn default() -> Self {
        Self::Dead
    }
}

#[derive(Debug, Clone)]
struct CellGrid {
    cells: [CellState; NO_OF_CELLS],
}

impl CellGrid {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn random_state(mut self) -> Self {
        for cell in self.iter_mut() {
            if rand::gen_range(0, 10) == 0 {
                *cell = CellState::Alive;
            }
        }

        self
    }
}

impl Default for CellGrid {
    fn default() -> Self {
        Self {
            cells: [Default::default(); NO_OF_CELLS],
        }
    }
}

impl Deref for CellGrid {
    type Target = [CellState; NO_OF_CELLS];

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl DerefMut for CellGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cells
    }
}

impl Index<usize> for CellGrid {
    type Output = CellState;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl IndexMut<usize> for CellGrid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

#[macroquad::main("conway's game of life")]
async fn main() {
    let mut cells = CellGrid::new().random_state();

    let mut prev_cells = CellGrid::new();

    let mut buffer = CellGrid::new();

    loop {
        request_new_screen_size(WIDTH as f32, HEIGHT as f32);
        clear_background(WHITE);
        draw_lines();

        color_alive_cells(&cells, &prev_cells);

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

fn color_cell(x: u32, y: u32, color: Color) {
    assert!(x < GRID_WIDTH);
    assert!(y < GRID_HEIGHT);

    draw_rectangle(
        (CELL_WIDTH * x) as f32,
        (CELL_HEIGHT * y) as f32,
        CELL_WIDTH as f32,
        CELL_HEIGHT as f32,
        color,
    );
}

fn color_alive_cells(cells: &CellGrid, prev_cells: &CellGrid) {
    for (i, cell) in cells.iter().enumerate() {
        if prev_cells[i] == CellState::Dead {
            if *cell == CellState::Alive {
                let x = i as u32 % GRID_WIDTH;
                let y = i as u32 / GRID_WIDTH;

                color_cell(x, y, PINK);
            }
        } else {
            if *cell == CellState::Dead {
                let x = i as u32 % GRID_WIDTH;
                let y = i as u32 / GRID_WIDTH;

                color_cell(x, y, LIGHTGRAY);
            }
        }
    }
}
