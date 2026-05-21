mod queue;
mod sudoku;

use std::process::ExitCode;

use clap::{Parser, Subcommand};

use queue::PrintQueue;
use sudoku::{print_grid, Difficulty};

/// Generate sudoku puzzles, persist them as JSON, and (eventually) send them
/// to a FutureLogic Gen2 thermal ticket printer over RS232.
///
/// The printer integration is not yet implemented; the `print` subcommand
/// is a stub. See the project README for the roadmap.
#[derive(Parser)]
#[command(name = "sudokuprinter", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate a batch of puzzles and write them to a JSON file.
    Generate {
        #[arg(short, long, default_value_t = 1)]
        count: usize,
        #[arg(short, long, default_value = "medium")]
        difficulty: String,
        #[arg(short, long, default_value = "puzzles.json")]
        output: String,
    },
    /// Load a JSON batch and pretty-print every puzzle to stdout.
    Show {
        #[arg(default_value = "puzzles.json")]
        input: String,
    },
    /// Send queued puzzles to the thermal printer (not yet implemented).
    Print {
        #[arg(default_value = "puzzles.json")]
        input: String,
    },
}

fn parse_difficulty(s: &str) -> Result<Difficulty, String> {
    match s.to_lowercase().as_str() {
        "easy" => Ok(Difficulty::Easy),
        "medium" => Ok(Difficulty::Medium),
        "hard" => Ok(Difficulty::Hard),
        "expert" => Ok(Difficulty::Expert),
        other => Err(format!(
            "unknown difficulty {other:?} (expected easy|medium|hard|expert)"
        )),
    }
}

fn cmd_generate(
    count: usize,
    difficulty: &str,
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let diff = parse_difficulty(difficulty)?;
    let mut queue = PrintQueue::new();
    queue.generate_batch(diff, count);
    queue.save_to_file(output)?;
    eprintln!("Generated {count} {diff:?} puzzle(s) -> {output}");
    Ok(())
}

fn cmd_show(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut queue = PrintQueue::new();
    queue.load_from_file(input)?;
    eprintln!("Loaded {} puzzle(s) from {}", queue.queue_size(), input);
    while let Some(p) = queue.get_next() {
        println!("ID: {}", p.id);
        println!("Timestamp: {}", p.timestamp);
        println!("Difficulty: {:?}", p.difficulty);
        print_grid(&p.grid);
        println!();
    }
    Ok(())
}

fn cmd_print(_input: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err("printer integration not yet implemented (RS232 + ESC/P2 backend pending)".into())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Generate {
            count,
            difficulty,
            output,
        } => cmd_generate(count, &difficulty, &output),
        Command::Show { input } => cmd_show(&input),
        Command::Print { input } => cmd_print(&input),
    };
    if let Err(e) = result {
        eprintln!("error: {e}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}
