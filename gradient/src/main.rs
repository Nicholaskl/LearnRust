extern crate colorful;

use colorful::Colorful;
use colorful::Color;

fn main() {
    let s = "Hello, world! This is a lovely gradient :o\n How does this looks";
    println!("{}", s.gradient(Color::Yellow).bold());
}
