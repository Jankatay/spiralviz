use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::collections::VecDeque;

// commandline options
pub mod option {
}

// for iterating csv files, it opens the file in an internal buffer
// and reads each comma-seperated value into a vector as needed one by one
pub struct CsvIterator {
    file_buffer: BufReader<File>,   // reads values into the memory
    current_line: VecDeque<String>, // stores current memory
}

// just like the normal file type
impl CsvIterator {
    pub fn new(path: &str) -> io::Result<CsvIterator> {
        Ok(CsvIterator {
            file_buffer: BufReader::new(File::open(path)?),
            current_line: VecDeque::new(),
        })
    }
}

// main idea
impl Iterator for CsvIterator {
    // may change to String later
    // TODO: Change this so it also gives row and column too, like in a tuple
    //          may also change name from spiralviz.rs to CsvIterator.rs while at it
    type Item = String;
    
    // main purpose of this whole class
    fn next(&mut self) -> Option<Self::Item> {
        // read a line if not already
        if self.current_line.is_empty() {
            // read_line returns Ok(0) at EOF - https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line
            let mut temp_line = String::new();
            if let Ok(0) = self.file_buffer.read_line(&mut temp_line) {
                return None;
            }
            // one-liner convert String -> Iterator<&str> -> vec<String>
            self.current_line = temp_line.split(',').map(|s| s.to_string()).collect();
            println!("current_line: {:?}", self.current_line);
        }
        // get an elem from the line if can
        self.current_line.pop_front()
    }
}

pub type Output = Vec<(i32, i32, i32)>;
pub fn run(file: &str) -> io::Result<Output> {
    Ok(vec![])
}

