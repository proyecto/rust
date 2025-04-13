use crate::traits::Action;
use std::error::Error;

#[derive(Debug)]
pub struct PrintHello;

impl Action for PrintHello {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("¡Hola desde PrintHello!");
        Ok(())
    }
}