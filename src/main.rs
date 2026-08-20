use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let target_file = File::open("./GLB.Ts+dSST.csv")?;
    let target_file = BufReader::new(target_file);

    // for each line in file
    for line in target_file.lines() {
        let line = line?;

        // print the index-value pairs
        for (i, value) in line.split(',').enumerate() {
            print!("({i},{value}), ");
        }
        println!();
    }

    // exit
    return Ok(());
}
