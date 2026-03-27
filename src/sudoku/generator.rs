use rand::seq::SliceRandom;
use rand::thread_rng;

use super::solver::{count_solutions, find_empty_cell, is_safe};
use super::{Grid, BOX_SIZE, GRID_SIZE, TOTAL_CELLS};

const DEFAULT_CLUES: usize = 40;

/// Generates a valid Sudoku grid with exactly one unique solution.
pub fn generate_sudoku() -> Grid
{
    let mut grid = [[0; GRID_SIZE]; GRID_SIZE];
    fill_diagonal(&mut grid);
    random_fill(&mut grid);
    remove_numbers(&mut grid, DEFAULT_CLUES);
    grid
}

/// Fills the diagonal 3x3 boxes (which are independent of each other).
fn fill_diagonal(grid: &mut Grid)
{
    for i in (0..GRID_SIZE).step_by(BOX_SIZE) {
        fill_box(grid, i, i);
    }
}

/// Helper to fill a single 3x3 box with random digits 1-9.
fn fill_box(grid: &mut Grid, row_start: usize, col_start: usize)
{
    let mut rng = thread_rng();
    let mut nums: Vec<u8> = (1..=9).collect();
    nums.shuffle(&mut rng);
    let mut idx = 0;
    for i in 0..BOX_SIZE {
        for j in 0..BOX_SIZE {
            grid[row_start + i][col_start + j] = nums[idx];
            idx += 1;
        }
    }
}

/// Recursively fills the rest of the board with random cell choices
/// to create a complete valid grid. Similar to a solver, but randomizes digit attempts.
fn random_fill(grid: &mut Grid) -> bool
{
    let (row, col) = match find_empty_cell(grid) {
        Some(pos) => pos,
        None => return true, // Board is completely filled
    };
    let mut rng = thread_rng();
    let mut nums: Vec<u8> = (1..=9).collect();
    nums.shuffle(&mut rng);
    for num in nums {
        if is_safe(grid, row, col, num) {
            grid[row][col] = num;
            if random_fill(grid) {
                return true;
            }
            grid[row][col] = 0; // Backtrack
        }
    }
    false
}

/// Removes numbers from a completed grid to reach the target number of clues,
/// guaranteeing the puzzle still has exactly one solution.
fn remove_numbers(grid: &mut Grid, clues: usize)
{
    let target_removals = TOTAL_CELLS - clues;
    if target_removals == 0 {
        return;
    }
    let mut rng = thread_rng();
    // Generate a list of all coordinates and shuffle them
    let mut cells: Vec<(usize, usize)> = Vec::with_capacity(TOTAL_CELLS);
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            cells.push((row, col));
        }
    }
    cells.shuffle(&mut rng);
    let mut removed = 0;
    for (row, col) in cells {
        if removed >= target_removals {
            break;
        }
        // Temporarily remove the number
        let backup = grid[row][col];
        grid[row][col] = 0;
        // Using *grid natively clones it because `[[u8; GRID_SIZE]; GRID_SIZE]` implements Copy
        let mut test_grid = *grid;
        // Check if removing this breaks uniqueness
        if count_solutions(&mut test_grid, 2) != 1 {
            grid[row][col] = backup; // Put it back, non-unique
        } else {
            removed += 1; // Successfully removed
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_generator_validity_and_uniqueness()
    {
        let clues = DEFAULT_CLUES;
        let grid = generate_sudoku();
        let mut actual_clues = 0;
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                if grid[r][c] != 0 {
                    actual_clues += 1;
                }
            }
        }
        // Checks every cell once, should get exactly the target clues
        assert!(
            actual_clues <= clues + 5,
            "Should have around {} clues, found {}",
            clues,
            actual_clues
        );
        let mut test_grid = grid; // Copy semantic
        assert_eq!(
            count_solutions(&mut test_grid, 2),
            1,
            "Generated puzzle doesn't have exactly 1 solution!"
        );
    }
}
