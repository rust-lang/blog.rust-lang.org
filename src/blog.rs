use std::error::Error;

#[path = "lib.rs"]
mod lib;

pub fn main() -> Result<(), Box<dyn Error>> {
    lib::main()
}
