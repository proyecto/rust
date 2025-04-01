// src/main_window.rs

use crate::constants::{
    LEFT_VIEW_COLOR, RIGHT_VIEW_COLOR, SIDEBAR_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use cocoa::appkit::{NSBackingStoreType, NSView, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::class;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{msg_send, sel, sel_impl};
use crate::main_sideview;

pub struct MainWindow {
    window: id,
}

impl MainWindow {
    pub fn create() -> Self {
        unsafe {
            let style = NSWindowStyleMask::NSTitledWindowMask
                | NSWindowStyleMask::NSClosableWindowMask
                | NSWindowStyleMask::NSResizableWindowMask
                | NSWindowStyleMask::NSMiniaturizableWindowMask;

            let frame = NSRect::new(
                NSPoint::new(0.0, 0.0),
                NSSize::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            );

            let window: id = NSWindow::alloc(nil)
                .initWithContentRect_styleMask_backing_defer_(
                    frame,
                    style,
                    NSBackingStoreType::NSBackingStoreBuffered,
                    0,
                );

            window.center();
            window.setTitle_(NSString::alloc(nil).init_str("MiApp"));

            let content_view: id = window.contentView();
            content_view.addSubview_(create_split_view(frame));

            let delegate: id = msg_send![create_delegate_class(), new];
            let _: () = msg_send![window, setDelegate: delegate];

            window.makeKeyAndOrderFront_(nil);

            MainWindow { window }
        }
    }
}

/// Crea un NSSplitView con dos subviews: una gris (sidebar) y una blanca (contenido)
unsafe fn create_split_view(frame: NSRect) -> id {
    let split_view: id = msg_send![class!(NSSplitView), alloc];
    let split_view: id = msg_send![split_view, initWithFrame: frame];
    let _: () = msg_send![split_view, setDividerStyle: 1];
    let _: () = msg_send![split_view, setVertical: true];

    let left_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(SIDEBAR_WIDTH, WINDOW_HEIGHT));
    let right_frame = NSRect::new(
        NSPoint::new(SIDEBAR_WIDTH, 0.0),
        NSSize::new(WINDOW_WIDTH - SIDEBAR_WIDTH, WINDOW_HEIGHT),
    );

    split_view.addSubview_(main_sideview::create(left_frame));
    split_view.addSubview_(create_colored_view(right_frame, RIGHT_VIEW_COLOR));
    let _: () = msg_send![split_view, adjustSubviews];

    split_view
}

/// Crea una vista con color de fondo personalizado (usando NSColor)
unsafe fn create_colored_view(frame: NSRect, rgb: (f32, f32, f32)) -> id {
    let view: id = NSView::alloc(nil).initWithFrame_(frame);
    let _: () = msg_send![view, setWantsLayer: true];
    let layer: id = msg_send![view, layer];

    let color: id = msg_send![
        class!(NSColor),
        colorWithRed: rgb.0
        green: rgb.1
        blue: rgb.2
        alpha: 1.0
    ];
    let cg_color: id = msg_send![color, CGColor];
    let _: () = msg_send![layer, setBackgroundColor: cg_color];

    view
}

/// Crea la clase delegate que termina la app al cerrar la ventana
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

    decl.add_method(
        sel!(windowShouldClose:),
        window_should_close as extern "C" fn(&Object, Sel, id) -> bool,
    );

    CLASS_PTR = decl.register();
    CLASS_PTR
}
