use std::io;

pub mod csv_parser;
use csv_parser::csv_open;

fn main() -> io::Result<()> {
    csv_open("./GLB.Ts+dSST.csv")?;
    Ok(())
}
