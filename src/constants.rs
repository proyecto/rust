use crate::traits::Action;
use crate::actions::{PrintHello, PrintHello2, ListPlayers};


// src/constants.rs

// Dimensiones de la ventana principal y barra lateral al inicio (en píxeles) si no hay valores en la base de datos
// (en caso de que no haya una base de datos SQLite, se usan estos valores por defecto)
pub const WINDOW_WIDTH: f64 = 800.0;
pub const WINDOW_HEIGHT: f64 = 600.0;
pub const SIDEBAR_WIDTH: f64 = 200.0;

// Dimensiones minimas de la ventana principal y barra lateral al inicio (en píxeles)
pub const MIN_WINDOW_WIDTH: f64 = 512.0;
pub const MIN_WINDOW_HEIGHT: f64 = 360.0;
pub const MIN_SIDEBAR_WIDTH: f64 = 200.0;

// Colores
pub const RIGHT_VIEW_COLOR: (f64, f64, f64) = (0.980, 0.980, 0.983); // Gris medio
pub const SELECTED_BUTTON_COLOR: (f64, f64, f64) = (0.85, 0.85, 0.85); // Gris claro
pub const LEFT_VIEW_COLOR: (f64, f64, f64) = (0.960, 0.966, 0.973); // Gris medio
pub const BUTTON_TEXT_COLOR: (f64, f64, f64) = (0.25, 0.25, 0.25); // Gris oscuroÇ


// Constantes botones sidebar
pub const BUTTON_WIDTH_MARGIN: f64 = 20.0;
pub const BUTTON_HEIGHT: f64 = 28.0;
pub const BUTTON_SPACING: f64 = 10.0;
pub const BUTTON_PADDING_TOP: f64 = 10.0;
pub const BUTTON_MARGIN_LEFT: f64 = 10.0;
pub const BUTTON_MARGIN_TOP: f64 = 10.0;

// Define una función que devuelve las entradas
pub fn get_buttons() -> Vec<(i16,&'static str, Box<dyn Action>)> {
    vec![
        (1,"👨🏻‍🦱​ Plantilla", Box::new(ListPlayers)),
        (2,"⚽​ Partidos", Box::new(PrintHello2)),
        (3,"🏟️ Estadio", Box::new(PrintHello)),
        /*(4,"Button 4", Box::new(PrintHello2)),
        (5,"Button 5", Box::new(PrintHello)),
        (6,"Button 6", Box::new(PrintHello2)),
        (7,"Button 7", Box::new(PrintHello)),
        (8,"Button 8", Box::new(PrintHello2)),
        (9,"Button 9", Box::new(PrintHello))*/
    ]
}


// Constantes label de los botones sidebar
pub const LABEL_MARGIN_LEFT: f64 = 8.0;
pub const LABEL_MARGIN_BOTTOM: f64 = 8.0;




