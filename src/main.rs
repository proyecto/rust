extern crate cocoa;
extern crate objc;

use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular};
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;
use objc::{class, msg_send, sel, sel_impl};

mod constants;
mod main_menu;
mod main_window;
pub mod views;
pub mod traits;
pub mod actions;
pub mod models;
pub mod libs;

use crate::libs::database;


fn main() {

    unsafe {
        let _: () = msg_send![class!(NSApplication), sharedApplication];
        println!("🔍 Cocoa runtime loaded, tracking what happens next...");
    }

    unsafe {
        // Crea un pool de autorelease para gestionar la memoria de los objetos Objective-C.
        let _pool = NSAutoreleasePool::new(nil);

        // Inicializa la aplicación Cocoa.
        let app = NSApp();

        // Crea una conexión a la base de datos SQLite.
        database::init(SQLITE_DB_PATH);

        // Establece la política de activación de la aplicación.
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // Llama a la función de configuración del menú desde el módulo 'menu'
        main_menu::MainMenu::setup();

        // Crea y muestra la ventana principal.
        let _window = main_window::MainWindow::create();

        // Aquí puedes continuar con la creación de ventanas u otros elementos
        app.run();
    }
}
