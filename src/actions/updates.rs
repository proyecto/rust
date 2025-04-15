use crate::traits::Action;
use std::error::Error;

#[derive(Debug)]
pub struct Updates;

impl Action for Updates {
    fn run(&self) -> Result<(), Box<dyn Error>> {

        println!("¡Hola desde Updates!");

        Ok(())
    }
}
