// src/window.rs

use cocoa::appkit::{NSWindow, NSBackingStoreType, NSWindowStyleMask};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSString};
use objc::runtime::{Class, Object, Sel};
use objc::declare::ClassDecl;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;

/// Crea y muestra la ventana principal de la aplicación.
/// La ventana incluirá los botones de cerrar, minimizar y redimensionar.
/// Además, al pulsar el botón de cerrar se terminará la aplicación completamente.
pub unsafe fn create_window() -> id {
    // Incluye NSMiniaturizableWindowMask para habilitar el botón de minimizar.
    let style = NSWindowStyleMask::NSTitledWindowMask
        | NSWindowStyleMask::NSClosableWindowMask
        | NSWindowStyleMask::NSResizableWindowMask
        | NSWindowStyleMask::NSMiniaturizableWindowMask;
    
    let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
        NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(640.0, 480.0)),
        style,
        NSBackingStoreType::NSBackingStoreBuffered,
        0,
    );
    
    window.center();
    window.setTitle_(NSString::alloc(nil).init_str("Mi Ventana"));
    
    // Asigna un delegate para interceptar el cierre de la ventana.
    // Cuando se pulse el botón de cerrar se llamará a windowShouldClose:
    // y en ese método terminaremos la aplicación con exit(0).
    let delegate_class = create_delegate_class();
    let delegate: id = msg_send![delegate_class, new];
    let _: () = msg_send![window, setDelegate: delegate];
    
    window.makeKeyAndOrderFront_(nil);
    window
}

/// Crea una clase delegate que intercepta el evento de cierre de la ventana.
/// En el método windowShouldClose: se termina la aplicación llamando a exit(0).
unsafe fn create_delegate_class() -> *const Class {
    // Obtenemos la clase NSObject como base.
    let superclass = Class::get("NSObject").unwrap();
    let mut decl = ClassDecl::new("AppDelegate", superclass).unwrap();
    
    extern "C" fn window_should_close(_this: &Object, _sel: Sel, _sender: id) -> bool {
        // Termina la aplicación inmediatamente.
        std::process::exit(0);
    }
    
    // Se añade el método windowShouldClose: al delegate.
    decl.add_method(sel!(windowShouldClose:), window_should_close as extern "C" fn(&Object, Sel, id) -> bool);
    decl.register()
}
