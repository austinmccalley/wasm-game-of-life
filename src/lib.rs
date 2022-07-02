mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn new(w: u32, h: u32) -> Universe {
        let width = w;
        let height = h;

        if width < 2 || height < 2 {
            panic!("Universe must be at least 2x2");
        }

        if width > 1024 || height > 1024 {
            panic!("Universe must be at most 1024x1024");
        }

           let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();


        // let mut cells: Vec<Cell> = (0..width * height).map(|_| Cell::Dead).collect();

        // // Get the center of the universe.
        // let middle = width / 2;
        // let mi = middle as u32;

        // let middle_height = height / 2;
        // let hi = middle_height as u32;

        // let mut m_idx = (mi * width + hi) as usize;
        // cells[m_idx] = Cell::Alive;

        // // Index lookup is
        // // (row * width + col)

        // // Positives
        // m_idx = (mi * width + (hi + 1)) as usize; // 0, 1
        // cells[m_idx] = Cell::Alive;
        // m_idx = ((mi + 1) * width + (hi + 1)) as usize; // 1, 1
        // cells[m_idx] = Cell::Alive;
        // m_idx = ((mi + 1) * width + (hi + 2)) as usize; // 1, 2
        // cells[m_idx] = Cell::Alive;
        // m_idx = ((mi + 2) * width + (hi + 2)) as usize; // 2, 2
        // cells[m_idx] = Cell::Alive;

        // // Negatives
        // m_idx = ((mi - 1) * width + (hi + 1)) as usize; // -1, 1
        // cells[m_idx] = Cell::Alive;
        // m_idx = ((mi - 1) * width + (hi + 2)) as usize; // -1, 2
        // cells[m_idx] = Cell::Alive;
        // m_idx = ((mi - 2) * width + (hi + 2)) as usize; // -2, 2
        // cells[m_idx] = Cell::Alive;

        // // Wall
        // m_idx = (mi * width + (hi - 13)) as usize; // 0, -13
        // cells[m_idx] = Cell::Alive;
        // m_idx = ((mi - 1) * width + (hi - 13)) as usize; // -1, -13
        // cells[m_idx] = Cell::Alive;
        // m_idx = ((mi + 1) * width + (hi - 13)) as usize; // 1, -13
        // cells[m_idx] = Cell::Alive;

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_| Cell::Dead).collect();
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}
