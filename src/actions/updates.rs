use crate::traits::Action;
use std::error::Error;

#[derive(Debug)]
pub struct Updates;

impl Action for Updates {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("¡Hola desde PrintHello!");
        Ok(())
    }
    fn render_view(&self) {
        // Aquí puedes implementar la lógica para renderizar la vista
        // utilizando el objeto `content_view`.
        // Por ejemplo, podrías agregar un botón o una etiqueta.
        println!("Renderizando vista en PrintHello 3");
    }
}