use crate::traits::Action;

#[derive(Debug)]
pub struct PrintHello2;

impl Action for PrintHello2 {
    fn run(&self) {
        println!("¡Hola desde PrintHell2 !");
    }
}