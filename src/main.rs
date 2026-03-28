mod sudoku;

fn main()
{
    println!("Sudoku Printer Algorithm");
    let difficulty = sudoku::Difficulty::Hard;
    let grid = sudoku::generator::generate_sudoku(difficulty);
    println!("Generated Sudoku ({:?}):", difficulty);
    sudoku::print_grid(&grid);
}
