mod queue;
mod sudoku;

use queue::PrintQueue;
use sudoku::Difficulty;

fn main()
{
    println!("Sudoku Printer");
    let mut print_queue = PrintQueue::new();
    let batch_size = 3;
    let difficulty = Difficulty::Hard;
    println!(
        "Generating a batch of {} {:?} Sudokus...",
        batch_size, difficulty
    );
    print_queue.generate_batch(difficulty, batch_size);
    println!("Current queue size: {}", print_queue.queue_size());
    let dbg_file = "batch_test.json";
    if let Err(e) = print_queue.save_to_file(dbg_file) {
        eprintln!("Failed to save batch: {}", e);
    } else {
        println!("Successfully saved generated batch to {}", dbg_file);
    }
    if let Some(puzzle) = print_queue.get_next() {
        println!("\nPopped next puzzle from queue:");
        println!("ID: {}", puzzle.id);
        println!("Timestamp: {}", puzzle.timestamp);
        println!("Difficulty: {:?}", puzzle.difficulty);
        sudoku::print_grid(&puzzle.grid);
        println!("\nRemaining queue size: {}", print_queue.queue_size());
    }
}
