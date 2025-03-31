// src/main_window.rs

use cocoa::appkit::{NSWindow, NSBackingStoreType, NSWindowStyleMask};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSString};
use objc::runtime::{Class, Object, Sel};
use objc::declare::ClassDecl;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;

pub struct MainWindow {
    window: id,
}

impl MainWindow {
    /// Crea y muestra la ventana principal de forma segura.
    pub fn create() -> Self {
        unsafe {
            let style = NSWindowStyleMask::NSTitledWindowMask
                | NSWindowStyleMask::NSClosableWindowMask
                | NSWindowStyleMask::NSResizableWindowMask
                | NSWindowStyleMask::NSMiniaturizableWindowMask;
            
            let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(1024.0, 768.0)),
                style,
                NSBackingStoreType::NSBackingStoreBuffered,
                0,
            );
            
            window.center();
            window.setTitle_(NSString::alloc(nil).init_str("Mi Ventana"));

            let delegate_class = create_delegate_class();
            let delegate: id = msg_send![delegate_class, new];
            let _: () = msg_send![window, setDelegate: delegate];

            window.makeKeyAndOrderFront_(nil);

            MainWindow { window }
        }
    }
}

/// Crea una clase delegate que intercepta el evento de cierre de la ventana.
/// En el método windowShouldClose: se termina la aplicación llamando a exit(0).
unsafe fn create_delegate_class() -> *const Class {
    static mut CLASS_PTR: *const Class = std::ptr::null();
    if !CLASS_PTR.is_null() {
        return CLASS_PTR;
    }

    let superclass = Class::get("NSObject").unwrap();
    let mut decl = ClassDecl::new("AppDelegate", superclass).unwrap();

    extern "C" fn window_should_close(_this: &Object, _sel: Sel, _sender: id) -> bool {
        std::process::exit(0);
    }

    decl.add_method(sel!(windowShouldClose:), window_should_close as extern "C" fn(&Object, Sel, id) -> bool);
    CLASS_PTR = decl.register();
    CLASS_PTR
}
