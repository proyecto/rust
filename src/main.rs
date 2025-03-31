extern crate cocoa;
extern crate objc;

mod constants;
mod main_menu;
mod main_window;
mod main_sideview;

use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular};
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;
//use rusqlite::Connection;

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();
        //let conn = Connection::open("test.db");

        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // Llama a la función de configuración del menú desde el módulo 'menu'
        main_menu::MainMenu::setup();

        // Crea y muestra la ventana principal.
        let _window = main_window::MainWindow::create();

        // Aquí puedes continuar con la creación de ventanas u otros elementos
        app.run();
    }
}
