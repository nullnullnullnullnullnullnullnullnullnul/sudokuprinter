pub mod generator;
pub mod solver;

use serde::{Deserialize, Serialize};

/// Difficulty preset selecting how many initial clues remain in a generated puzzle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl Difficulty {
    /// Target number of clue cells (filled cells) for a fresh puzzle.
    pub fn target_clues(self) -> usize {
        match self {
            Difficulty::Easy => 45,
            Difficulty::Medium => 35,
            Difficulty::Hard => 28,
            Difficulty::Expert => 24,
        }
    }
}

pub const GRID_SIZE: usize = 9;
pub const BOX_SIZE: usize = 3;
pub const TOTAL_CELLS: usize = 81;

/// A 9x9 sudoku grid. A cell value of `0` represents an empty cell.
pub type Grid = [[u8; GRID_SIZE]; GRID_SIZE];

/// Render a sudoku grid to stdout using box-drawing characters.
#[allow(clippy::needless_range_loop)]
pub fn print_grid(grid: &Grid) {
    println!("┌───────┬───────┬───────┐");
    for row in 0..GRID_SIZE {
        if row % BOX_SIZE == 0 && row != 0 {
            println!("├───────┼───────┼───────┤");
        }
        print!("│ ");
        for col in 0..GRID_SIZE {
            if col % BOX_SIZE == 0 && col != 0 {
                print!("│ ");
            }
            if grid[row][col] == 0 {
                print!(". ");
            } else {
                print!("{} ", grid[row][col]);
            }
        }
        println!("│");
    }
    println!("└───────┴───────┴───────┘");
}
