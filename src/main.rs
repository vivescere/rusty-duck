use std::fs;
use std::io::prelude::*;

mod encoder;
mod parser;

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("examples/test.duck")?;
    let program = parser::parse(&content).ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to parse program.")
    })?;
    dbg!(&program);
    let encoded = encoder::encode(&program);
    println!("{}", encoded);
    Ok(())
}
