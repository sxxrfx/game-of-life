use std::{
    mem,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use rand::Rng;
use raylib::prelude::*;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;
const GRID_WIDTH: u32 = 80;
const GRID_HEIGHT: u32 = 80;
const CELL_WIDTH: u32 = SCREEN_WIDTH / GRID_WIDTH;
const CELL_HEIGHT: u32 = SCREEN_HEIGHT / GRID_HEIGHT;
const FPS: u32 = 30;

const NO_OF_CELLS: usize = (GRID_HEIGHT * GRID_WIDTH) as usize;
const GRID_LINE_THICKNESS: f32 = 1.0;

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
            if rand::thread_rng().gen_range(0..3) == 0 {
                *cell = CellState::Alive;
            }
        }

        self
    }

    pub fn swap(&mut self, other: &mut Self) {
        mem::swap(&mut self.cells, &mut other.cells);
    }

    pub fn copy_from(&mut self, other: &Self) {
        self.copy_from_slice(&other.cells);
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

fn main() {
    let alive_cell_color = Color::GOLD;
    let grid_color = Color::LIGHTGRAY;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("conway's game of life")
        .vsync()
        .msaa_4x()
        .build();

    let mut cells = CellGrid::new().random_state();

    let mut prev_cells = CellGrid::new();

    let mut buffer = CellGrid::new();

    let neighbor_idx = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    rl.set_target_fps(FPS);

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            cells = cells.random_state();
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        draw_grid(&mut d, grid_color);

        for idx in 0..NO_OF_CELLS {
            let mut neighbors_count = 0;
            let (x, y) = idx_to_xy(idx);

            for (i, j) in neighbor_idx {
                // out of bounds
                if y as i32 + j < 0
                    || y as i32 + j >= GRID_HEIGHT as i32
                    || x as i32 + i < 0
                    || x as i32 + i >= GRID_WIDTH as i32
                {
                    continue;
                }
                // self is not present (0, 0)

                let neighbor = cells[xy_to_idx(
                    (x as i32 + i) as usize,
                    (y as i32 + j) as usize,
                )];
                if neighbor == CellState::Alive {
                    neighbors_count += 1;
                }
            }

            let current_cell = cells[idx];
            buffer[idx] = match (current_cell, neighbors_count) {
                (CellState::Alive, x) if x < 2 => {
                    CellState::Dead
                }
                (CellState::Alive, 2)
                | (CellState::Alive, 3) => CellState::Alive,
                (CellState::Alive, x) if x > 3 => {
                    CellState::Dead
                }
                (CellState::Dead, 3) => CellState::Alive,
                (otherwise, _) => otherwise,
            }
        }

        color_alive_cells(
            &mut d,
            &buffer,
            &prev_cells,
            alive_cell_color,
        );

        prev_cells.copy_from(&cells);
        cells.copy_from(&buffer);
    }
}

fn draw_grid(d: &mut RaylibDrawHandle, color: Color) {
    for i in 1..GRID_WIDTH {
        d.draw_line(
            (CELL_WIDTH * i) as i32,
            0,
            (CELL_WIDTH * i) as i32,
            SCREEN_HEIGHT as i32,
            color,
        );
    }
    for i in 1..GRID_HEIGHT {
        d.draw_line(
            0,
            (CELL_HEIGHT * i) as i32,
            SCREEN_WIDTH as i32,
            (CELL_HEIGHT * i) as i32,
            color,
        );
    }
}

fn color_cell(
    d: &mut RaylibDrawHandle,
    x: u32,
    y: u32,
    color: Color,
) {
    assert!(x < GRID_WIDTH);
    assert!(y < GRID_HEIGHT);

    d.draw_rectangle(
        (CELL_WIDTH * x) as i32,
        (CELL_HEIGHT * y) as i32,
        CELL_WIDTH as i32,
        CELL_HEIGHT as i32,
        color,
    );
}

fn xy_to_idx(x: usize, y: usize) -> usize {
    y * GRID_WIDTH as usize + x
}

fn idx_to_xy(idx: usize) -> (usize, usize) {
    (idx % GRID_WIDTH as usize, idx / GRID_WIDTH as usize)
}

fn color_alive_cells(
    d: &mut RaylibDrawHandle,
    cells: &CellGrid,
    prev_cells: &CellGrid,
    color: Color,
) {
    for (i, cell) in cells.iter().enumerate() {
        if prev_cells[i] == CellState::Dead {
            if *cell == CellState::Alive {
                let x = i as u32 % GRID_WIDTH;
                let y = i as u32 / GRID_WIDTH;

                color_cell(d, x, y, color);
            }
        } else {
            // if *cell == CellState::Dead {
            //     let x = i as u32 % GRID_WIDTH;
            //     let y = i as u32 / GRID_WIDTH;
            //
            //     color_cell(d, x, y, Color::WHITE);
            // }
        }
    }
}
