use crate::traits::Action;
use std::error::Error;

#[derive(Debug)]
pub struct PrintHello2;

impl Action for PrintHello2 {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("¡Hola desde PrintHell2 !");
        Ok(())
    }
}
