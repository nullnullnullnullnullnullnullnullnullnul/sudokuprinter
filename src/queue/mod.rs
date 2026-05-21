use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::sudoku::{generator::generate_sudoku, Difficulty, Grid};

/// A puzzle waiting in the print queue. The id is generated on enqueue and
/// stays stable across save/load cycles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedPuzzle {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub difficulty: Difficulty,
    pub grid: Grid,
}

/// FIFO queue of puzzles awaiting print. Persistable to and from JSON.
#[derive(Debug, Default)]
pub struct PrintQueue {
    queue: VecDeque<QueuedPuzzle>,
}

impl PrintQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Append `count` newly generated puzzles to the back of the queue.
    pub fn generate_batch(&mut self, difficulty: Difficulty, count: usize) {
        for _ in 0..count {
            self.queue.push_back(QueuedPuzzle {
                id: Uuid::new_v4(),
                timestamp: Utc::now(),
                difficulty,
                grid: generate_sudoku(difficulty),
            });
        }
    }

    /// Pop the next puzzle from the front of the queue.
    pub fn get_next(&mut self) -> Option<QueuedPuzzle> {
        self.queue.pop_front()
    }

    pub fn queue_size(&self) -> usize {
        self.queue.len()
    }

    /// Serialize the queue to a pretty-printed JSON file.
    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        let writer = BufWriter::new(File::create(path)?);
        serde_json::to_writer_pretty(writer, &self.queue).map_err(io::Error::other)?;
        Ok(())
    }

    /// Load a JSON batch and append its puzzles to the back of the current queue.
    pub fn load_from_file(&mut self, path: &str) -> io::Result<()> {
        let reader = BufReader::new(File::open(path)?);
        let batch: VecDeque<QueuedPuzzle> =
            serde_json::from_reader(reader).map_err(io::Error::other)?;
        self.queue.extend(batch);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sudoku::print_grid;
    use std::fs;

    #[test]
    fn test_print_queue_basic_operations() {
        let mut queue = PrintQueue::new();
        assert_eq!(queue.queue_size(), 0);

        queue.generate_batch(Difficulty::Easy, 3);
        assert_eq!(queue.queue_size(), 3);

        let first = queue.get_next().unwrap();
        assert_eq!(first.difficulty, Difficulty::Easy);
        print_grid(&first.grid);
        assert_eq!(queue.queue_size(), 2);
    }

    #[test]
    fn test_queue_round_trips_through_disk() {
        let mut queue = PrintQueue::new();
        queue.generate_batch(Difficulty::Medium, 2);

        let path = "test_batch.json";
        queue.save_to_file(path).expect("save should succeed");

        let mut loaded = PrintQueue::new();
        loaded.load_from_file(path).expect("load should succeed");
        assert_eq!(loaded.queue_size(), 2);
        assert_eq!(loaded.get_next().unwrap().difficulty, Difficulty::Medium);

        let _ = fs::remove_file(path);
    }
}
