use std::io;

pub mod csv_parser;
use csv_parser::csv_open;

//TODO: make csv_open work with double quoted values instead of crashing 
// also test if CRLF_REQUIRED works when turned true
// also add a pedantic rfc4180 mode out of spite
fn main() -> io::Result<()> {
    let table = csv_open("./GLB.Ts+dSST.csv")?;
    for line in &table[0..=2] {
        println!("line -> {:?}", line);
    }
    Ok(())
}
