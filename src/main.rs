// src/main.rs

#[macro_use]
extern crate cocoa;
#[macro_use]
extern crate objc;

mod menu;
mod window;

use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular};
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // Llama a la función de configuración del menú desde el módulo 'menu'
        menu::setup_menu();

        // Crea y muestra la ventana principal.
        let _window = window::create_window();

        // Aquí puedes continuar con la creación de ventanas u otros elementos
        app.run();
    }
}
