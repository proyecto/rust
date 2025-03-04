#[macro_use]
extern crate objc;
extern crate cocoa;

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivationPolicyRegular,
    NSWindow, NSWindowStyleMask, NSBackingStoreType,
};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString};
use objc::runtime::{Class, Object, Sel};
use objc::declare::ClassDecl;

fn main() {
    unsafe {
        // Crea un pool de autorelease para gestionar la memoria
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // Creamos y asignamos nuestro delegate personalizado
        let delegate_class = create_delegate_class();
        let delegate: id = msg_send![delegate_class, new];

        // Combinamos las máscaras necesarias para la ventana
        let style = NSWindowStyleMask::NSTitledWindowMask
            | NSWindowStyleMask::NSClosableWindowMask
            | NSWindowStyleMask::NSMiniaturizableWindowMask
            | NSWindowStyleMask::NSResizableWindowMask;

        let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
            NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(640.0, 480.0)),
            style,
            NSBackingStoreType::NSBackingStoreBuffered,
            0,
        );

        window.center();
        window.setTitle_(NSString::alloc(nil).init_str("Mínima App en Rust"));
        // Asignamos el delegate a la ventana
        window.setDelegate_(delegate);
        window.makeKeyAndOrderFront_(nil);

        app.run();
    }
}

/// Crea una clase delegate que intercepta el evento de cierre de la ventana
unsafe fn create_delegate_class() -> *const Class {
    // La clase base es NSObject
    let superclass = Class::get("NSObject").unwrap();
    let mut decl = ClassDecl::new("AppDelegate", superclass).unwrap();

    // Esta función se ejecuta cuando la ventana recibe el mensaje windowShouldClose:
    extern "C" fn window_should_close(_this: &Object, _sel: Sel, _sender: id) -> bool {
        // Sale inmediatamente de la aplicación
        std::process::exit(0);
    }

    // Se añade el método al delegate con el selector windowShouldClose:
    decl.add_method(sel!(windowShouldClose:), window_should_close as extern "C" fn(&Object, Sel, id) -> bool);
    decl.register()
}
