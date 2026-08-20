use std::io;

pub mod spiralviz;
use spiralviz::CsvIterator;

fn main() -> io::Result<()> {
    let mut csv_iterator = CsvIterator::new("./GLB.Ts+dSST.csv")?;
    for elem in csv_iterator {
    }
    Ok(())
}
