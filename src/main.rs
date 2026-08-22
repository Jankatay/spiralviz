use std::io;

pub mod csv_parser;
use csv_parser::csv_open;

//TODO: test if CRLF_REQUIRED works when turned true
fn main() -> io::Result<()> {
    let table = csv_open("./GLB.Ts+dSST.csv")?;
    for line in &table[0..=2] {
        println!("line -> {:?}", line);
    }
    Ok(())
}
