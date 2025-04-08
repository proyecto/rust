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
pub const LEFT_VIEW_COLOR: (f64, f64, f64) = (0.9, 0.9, 0.9); // Gris medio
pub const RIGHT_VIEW_COLOR: (f32, f32, f32) = (1.0, 1.0, 1.0);

// Constantes del sidebar
pub const BUTTON_HEIGHT: f64 = 40.0;
pub const BUTTON_MARGIN_TOP: f64 = 60.0;
pub const BUTTON_SPACING: f64 = 10.0;

