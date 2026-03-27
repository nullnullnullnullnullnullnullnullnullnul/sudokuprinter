mod sudoku;

fn main()
{
    println!("Sudoku Printer Algorithm");
    // Quick demonstration of the module
    let grid = sudoku::generator::generate_sudoku();
    println!("Generated basic SDK:");
    sudoku::print_grid(&grid);
}
