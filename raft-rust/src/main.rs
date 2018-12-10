use crate::colors::Color;

mod colors;
mod log;
mod membership;
mod state;

fn main() {
    println!("Hello, world!");
    let color: Color = Default::default();
    println!("{:?}", color.unwrap());
}
