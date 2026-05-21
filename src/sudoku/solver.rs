use super::{Grid, BOX_SIZE};

/// Returns true if placing `num` at `grid[row][col]` would not violate
/// sudoku row, column, or 3x3 box constraints.
pub fn is_safe(grid: &Grid, row: usize, col: usize, num: u8) -> bool {
    if grid[row].contains(&num) {
        return false;
    }
    if grid.iter().any(|r| r[col] == num) {
        return false;
    }
    let start_row = row - row % BOX_SIZE;
    let start_col = col - col % BOX_SIZE;
    for r in &grid[start_row..start_row + BOX_SIZE] {
        if r[start_col..start_col + BOX_SIZE].contains(&num) {
            return false;
        }
    }
    true
}

/// Returns the coordinates of the first empty cell scanning row-by-row,
/// or `None` if every cell is filled.
pub(crate) fn find_empty_cell(grid: &Grid) -> Option<(usize, usize)> {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == 0 {
                return Some((row_idx, col_idx));
            }
        }
    }
    None
}

/// Solve `grid` in place using backtracking. Returns true if a solution was found.
#[allow(dead_code)] // exercised by tests; reserved as part of the solver API
pub fn solve(grid: &mut Grid) -> bool {
    let (row, col) = match find_empty_cell(grid) {
        Some(pos) => pos,
        None => return true,
    };
    for num in 1..=9 {
        if is_safe(grid, row, col, num) {
            grid[row][col] = num;
            if solve(grid) {
                return true;
            }
            grid[row][col] = 0;
        }
    }
    false
}

/// Count solutions reachable from the current grid state, short-circuiting
/// once `limit` solutions have been observed. Useful for verifying uniqueness
/// without exploring the full search space.
pub fn count_solutions(grid: &mut Grid, limit: usize) -> usize {
    let mut count = 0;
    count_solutions_helper(grid, &mut count, limit);
    count
}

fn count_solutions_helper(grid: &mut Grid, count: &mut usize, limit: usize) {
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
            grid[row][col] = 0;
            if *count >= limit {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sudoku::{print_grid, GRID_SIZE};

    #[test]
    fn test_is_safe() {
        let mut grid: Grid = [[0; GRID_SIZE]; GRID_SIZE];
        assert!(is_safe(&grid, 0, 0, 5));
        grid[0][0] = 5;
        assert!(!is_safe(&grid, 0, 8, 5), "same row");
        assert!(!is_safe(&grid, 8, 0, 5), "same col");
        assert!(!is_safe(&grid, 2, 2, 5), "same box");
        assert!(is_safe(&grid, 2, 2, 6), "different number");
    }

    #[test]
    fn test_solve_empty_board() {
        let mut grid: Grid = [[0; GRID_SIZE]; GRID_SIZE];
        assert!(solve(&mut grid));
        assert!(grid[0][0] != 0, "solver should have filled the board");
        println!("test_solve_empty_board generated grid:");
        print_grid(&grid);
    }

    #[test]
    fn test_count_solutions_empty_board_short_circuits() {
        let mut grid: Grid = [[0; GRID_SIZE]; GRID_SIZE];
        assert_eq!(count_solutions(&mut grid, 2), 2);
    }
}
