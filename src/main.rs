use std::io;

pub mod parser;
use parser::csv_to_table;

fn main() -> io::Result<()> {
    csv_to_table("./GLB.Ts+dSST.csv")?;
    Ok(())
}
