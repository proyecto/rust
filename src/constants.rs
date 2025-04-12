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
pub const RIGHT_VIEW_COLOR: (f64, f64, f64) = (1.0, 1.0, 1.0);
pub const SELECTED_BUTTON_COLOR: (f64, f64, f64) = (0.85, 0.85, 0.85); // Gris claro
pub const LEFT_VIEW_COLOR: (f64, f64, f64) = (0.95, 0.95, 0.95); // Gris medio
pub const BUTTON_TEXT_COLOR: (f64, f64, f64) = (0.25, 0.25, 0.25); // Gris oscuro

// Constantes botones sidebar
pub const BUTTON_WIDTH_MARGIN: f64 = 20.0;
pub const BUTTON_HEIGHT: f64 = 28.0;
pub const BUTTON_SPACING: f64 = 10.0;
pub const BUTTON_PADDING_TOP: f64 = 10.0;
pub const BUTTON_MARGIN_LEFT: f64 = 10.0;
pub const BUTTON_MARGIN_TOP: f64 = 10.0;

// Constantes label de los botones sidebar
pub const LABEL_MARGIN_LEFT: f64 = 8.0;
pub const LABEL_MARGIN_BOTTOM: f64 = 8.0;




