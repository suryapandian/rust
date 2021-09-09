use std::fmt;

fn get_displayable() -> impl fmt::Display {
        13
}

fn main() {
    println!("output is {}", get_displayable());
}