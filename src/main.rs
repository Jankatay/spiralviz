use std::io;

pub mod csv_parser;
use csv_parser::csv_open;

fn main() -> io::Result<()> {
    // init
    let has_header = false;
    let table = csv_open("./GLB.Ts+dSST.csv")?;
    if table.len() <= 0 {
        //TODO:
    }


    Ok(())
}
