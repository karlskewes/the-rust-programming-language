// bring Asparagus into root scope.
use crate::garden::vegetables::Asparagus;

// include code from src/garden.rs
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
