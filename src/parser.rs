use std::{fs, io};

pub type Table = Vec<Vec<String>>; // simplest representation of csv possible

// convert csv file to a table in memory you can just operate on
pub fn csv_to_table(file_path: &str) -> io::Result<Table> {
    // init 
    let dquote = false;
    let start_index = 0;
    let str_data = fs::read_to_string(file_path)?;

    // sanitize
    if !str_data.is_ascii() { 
        return Err(io::ErrorKind::InvalidData.into()); 
    }

    // while looping through the data
    for (i, ch) in str_data.chars().enumerate() {
    }
    Ok(vec![])
}
