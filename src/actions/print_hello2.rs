use crate::traits::Action;
use std::error::Error;
use cocoa::base::id;


#[derive(Debug)]
pub struct PrintHello2;

impl Action for PrintHello2 {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("¡Hola desde PrintHell2 !");
        Ok(())
    }
    fn render_view(&self, content_view: id) {
        // Aquí puedes implementar la lógica para renderizar la vista
        // utilizando el objeto `content_view`.
        // Por ejemplo, podrías agregar un botón o una etiqueta.
        println!("Renderizando vista en PrintHello2");
    }
}
