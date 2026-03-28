use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use uuid::Uuid;

use crate::sudoku::{generator::generate_sudoku, print_grid, Difficulty, Grid};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedPuzzle
{
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub difficulty: Difficulty,
    pub grid: Grid,
}

#[derive(Debug, Default)]
pub struct PrintQueue
{
    queue: VecDeque<QueuedPuzzle>,
}

impl PrintQueue
{
    pub fn new() -> Self
    {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn generate_batch(&mut self, difficulty: Difficulty, count: usize)
    {
        for _ in 0..count {
            let grid = generate_sudoku(difficulty);
            let puzzle = QueuedPuzzle {
                id: Uuid::new_v4(),
                timestamp: Utc::now(),
                difficulty,
                grid,
            };
            self.queue.push_back(puzzle);
        }
    }

    pub fn clear_queue(&mut self)
    {
        self.queue.clear();
    }

    pub fn get_next(&mut self) -> Option<QueuedPuzzle>
    {
        self.queue.pop_front()
    }

    pub fn queue_size(&self) -> usize
    {
        self.queue.len()
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()>
    {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.queue)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(())
    }

    pub fn load_from_file(&mut self, path: &str) -> std::io::Result<()>
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let batch: VecDeque<QueuedPuzzle> = serde_json::from_reader(reader)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        self.queue.extend(batch);
        Ok(())
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use std::fs;

    #[test]
    fn test_print_queue_operations()
    {
        println!("\nTesting PrintQueue basic operations...");
        let mut queue = PrintQueue::new();
        assert_eq!(queue.queue_size(), 0);
        println!("Queue size is initially 0.");
        queue.generate_batch(Difficulty::Easy, 3);
        assert_eq!(queue.queue_size(), 3);
        println!(
            "Queue size after generating 3 Easy puzzles: {}",
            queue.queue_size()
        );
        let first = queue.get_next().unwrap();
        assert_eq!(first.difficulty, Difficulty::Easy);
        println!("Popped next puzzle. ID: {}", first.id);
        println!("Difficulty: {:?}", first.difficulty);
        print_grid(&first.grid);
        assert_eq!(queue.queue_size(), 2);
        println!("Queue size after pop: {}", queue.queue_size());
        queue.clear_queue();
        assert_eq!(queue.queue_size(), 0);
        println!("Queue size after clear: {}", queue.queue_size());
    }

    #[test]
    fn test_queue_file_operations()
    {
        println!("\nTesting PrintQueue file I/O operations...");
        let mut queue = PrintQueue::new();
        queue.generate_batch(Difficulty::Medium, 2);
        let file_path = "test_batch.json";
        println!("Saving 2 puzzles to {}...", file_path);
        let save_result = queue.save_to_file(file_path);
        assert!(save_result.is_ok());
        let mut loaded_queue = PrintQueue::new();
        println!("Loading puzzles from {}...", file_path);
        let load_result = loaded_queue.load_from_file(file_path);
        assert!(load_result.is_ok());
        assert_eq!(loaded_queue.queue_size(), 2);
        let loaded_puzzle = loaded_queue.get_next().unwrap();
        assert_eq!(loaded_puzzle.difficulty, Difficulty::Medium);
        println!("Successfully loaded puzzle. ID: {}", loaded_puzzle.id);
        let _ = fs::remove_file(file_path);
        println!("Cleaned up file {}.", file_path);
    }
}
