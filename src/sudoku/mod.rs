pub mod generator;
pub mod solver;

pub const GRID_SIZE: usize = 9;
pub const BOX_SIZE: usize = 3;
pub const TOTAL_CELLS: usize = 81;

/// Basic Sudoku grid representation.
/// 0 == empty cell.
pub type Grid = [[u8; GRID_SIZE]; GRID_SIZE];

/// Prints a formatted Sudoku grid.
pub fn print_grid(grid: &Grid)
{
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
