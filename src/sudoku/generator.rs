use rand::seq::SliceRandom;
use rand::thread_rng;

use super::solver::{count_solutions, find_empty_cell, is_safe};
use super::{Difficulty, Grid, BOX_SIZE, GRID_SIZE, TOTAL_CELLS};

/// Generate a sudoku puzzle with exactly one solution and approximately
/// `difficulty.target_clues()` clues remaining.
pub fn generate_sudoku(difficulty: Difficulty) -> Grid {
    let mut grid = [[0; GRID_SIZE]; GRID_SIZE];
    fill_diagonal(&mut grid);
    random_fill(&mut grid);
    remove_numbers(&mut grid, difficulty.target_clues());
    grid
}

/// Fill the three 3x3 diagonal boxes with random digits 1-9.
/// These boxes share no row or column with each other, so they can be
/// filled independently without backtracking.
fn fill_diagonal(grid: &mut Grid) {
    for i in (0..GRID_SIZE).step_by(BOX_SIZE) {
        fill_box(grid, i, i);
    }
}

#[allow(clippy::needless_range_loop)]
fn fill_box(grid: &mut Grid, row_start: usize, col_start: usize) {
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

/// Backtracking fill of every empty cell with a random valid digit.
/// Returns true once the grid is fully filled, false if no valid completion exists.
fn random_fill(grid: &mut Grid) -> bool {
    let (row, col) = match find_empty_cell(grid) {
        Some(pos) => pos,
        None => return true,
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
            grid[row][col] = 0;
        }
    }
    false
}

/// Remove cells from a complete grid until roughly `clues` cells remain,
/// rejecting any removal that would make the puzzle ambiguous (i.e. produce
/// more than one solution).
fn remove_numbers(grid: &mut Grid, clues: usize) {
    let target_removals = TOTAL_CELLS.saturating_sub(clues);
    if target_removals == 0 {
        return;
    }
    let mut rng = thread_rng();
    let mut cells: Vec<(usize, usize)> = (0..GRID_SIZE)
        .flat_map(|r| (0..GRID_SIZE).map(move |c| (r, c)))
        .collect();
    cells.shuffle(&mut rng);

    let mut removed = 0;
    for (row, col) in cells {
        if removed >= target_removals {
            break;
        }
        let backup = grid[row][col];
        grid[row][col] = 0;
        // Grid is `Copy`, so this clones it cheaply for the uniqueness check.
        let mut test_grid = *grid;
        if count_solutions(&mut test_grid, 2) != 1 {
            grid[row][col] = backup;
        } else {
            removed += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_all_difficulties() {
        let difficulties = [
            Difficulty::Easy,
            Difficulty::Medium,
            Difficulty::Hard,
            Difficulty::Expert,
        ];
        for &difficulty in &difficulties {
            println!("Testing difficulty: {difficulty:?}");
            let clues = difficulty.target_clues();
            let grid = generate_sudoku(difficulty);
            let actual_clues = grid.iter().flatten().filter(|&&v| v != 0).count();
            // Removal can stop short of the target if no further cell removal
            // preserves the unique-solution invariant; tolerate a small overrun.
            assert!(
                actual_clues <= clues + 5,
                "expected around {clues} clues, found {actual_clues}"
            );
            let mut test_grid = grid;
            assert_eq!(
                count_solutions(&mut test_grid, 2),
                1,
                "generated puzzle for {difficulty:?} must have exactly one solution"
            );
            super::super::print_grid(&grid);
            println!();
        }
    }
}
