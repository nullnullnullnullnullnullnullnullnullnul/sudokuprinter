use super::{Grid, BOX_SIZE, GRID_SIZE};

/// Checks if it's safe to place `num` at `grid[row][col]`.
pub fn is_safe(grid: &Grid, row: usize, col: usize, num: u8) -> bool
{
    // Check row and column
    for x in 0..GRID_SIZE {
        if grid[row][x] == num {
            return false;
        }
        if grid[x][col] == num {
            return false;
        }
    }
    // Check 3x3 box
    let start_row = row - row % BOX_SIZE;
    let start_col = col - col % BOX_SIZE;
    for i in 0..BOX_SIZE {
        for j in 0..BOX_SIZE {
            if grid[start_row + i][start_col + j] == num {
                return false;
            }
        }
    }
    true
}

/// Helper to find the next empty cell on the board.
pub(crate) fn find_empty_cell(grid: &Grid) -> Option<(usize, usize)>
{
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if grid[row][col] == 0 {
                return Some((row, col));
            }
        }
    }
    None
}

/// Solves a Sudoku grid using backtracking. Returns true if a solution is found.
/// The `grid` parameter is modified in place to hold the solution.
#[allow(dead_code)]
pub fn solve(grid: &mut Grid) -> bool
{
    let (row, col) = match find_empty_cell(grid) {
        Some(pos) => pos,
        None => return true, // Solved
    };
    for num in 1..=9 {
        if is_safe(grid, row, col, num) {
            grid[row][col] = num;
            if solve(grid) {
                return true;
            }
            grid[row][col] = 0; // Backtrack
        }
    }
    false
}

/// Counts the number of solutions from the current grid state.
/// Limits counting to `limit` to avoid long execution times for empty or highly open boards.
pub fn count_solutions(grid: &mut Grid, limit: usize) -> usize
{
    let mut count = 0;
    count_solutions_helper(grid, &mut count, limit);
    count
}

fn count_solutions_helper(grid: &mut Grid, count: &mut usize, limit: usize)
{
    if *count >= limit {
        return;
    }
    let (row, col) = match find_empty_cell(grid) {
        Some(pos) => pos,
        None => {
            *count += 1;
            return;
        }
    };
    for num in 1..=9 {
        if is_safe(grid, row, col, num) {
            grid[row][col] = num;
            count_solutions_helper(grid, count, limit);
            grid[row][col] = 0; // Backtrack
            if *count >= limit {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_is_safe()
    {
        let mut grid: Grid = [[0; GRID_SIZE]; GRID_SIZE];
        assert!(is_safe(&grid, 0, 0, 5));
        grid[0][0] = 5;
        assert!(!is_safe(&grid, 0, 8, 5)); // Same row
        assert!(!is_safe(&grid, 8, 0, 5)); // Same col
        assert!(!is_safe(&grid, 2, 2, 5)); // Same box
        assert!(is_safe(&grid, 2, 2, 6)); // Diff num
    }

    #[test]
    fn test_solve_empty_board()
    {
        let mut grid: Grid = [[0; GRID_SIZE]; GRID_SIZE];
        assert!(solve(&mut grid));
        assert!(grid[0][0] != 0); // Sanity check that it filled something
        println!("test_solve_empty_board generated grid:");
        super::super::print_grid(&grid);
    }

    #[test]
    fn test_count_solutions_empty()
    {
        let mut grid: Grid = [[0; GRID_SIZE]; GRID_SIZE];
        assert_eq!(count_solutions(&mut grid, 2), 2);
    }
}
