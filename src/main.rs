mod parser;

fn main() {
    println!("Hello, world!");
    dbg!(parser::parse("DELAY 500"));
}
