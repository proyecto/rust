use crate::traits::Action;

#[derive(Debug)]
pub struct PrintHello;

impl Action for PrintHello {
    fn run(&self) {
        println!("¡Hola desde PrintHello!");
    }
}