use cocoa::appkit::NSView;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use std::sync::Once;

// Una sola vez
static INIT: Once = Once::new();

// Constantes para seguimiento del mouse
const NSTrackingActiveAlways: u64 = 0x01;
const NSTrackingMouseEnteredAndExited: u64 = 0x02;
const NSTrackingInVisibleRect: u64 = 0x04;

// Al hacer clic
extern "C" fn mouse_down(this: &Object, _: Sel, _: id) {
    println!("✅ Botón estilo Things clickeado!");
}

// Al entrar el mouse
extern "C" fn mouse_entered(this: &Object, _: Sel, _: id) {
    unsafe {
        let layer: id = msg_send![this, layer];
        let hover_color: id = msg_send![
            class!(NSColor),
            colorWithCalibratedRed: 0.90 green: 0.90 blue: 0.92 alpha: 1.0
        ];
        let cg_color: id = msg_send![hover_color, CGColor];
        let _: () = msg_send![layer, setBackgroundColor: cg_color];
    }
}

// Al salir el mouse
extern "C" fn mouse_exited(this: &Object, _: Sel, _: id) {
    unsafe {
        let layer: id = msg_send![this, layer];
        let base_color: id = msg_send![
            class!(NSColor),
            colorWithCalibratedRed: 0.96 green: 0.96 blue: 0.97 alpha: 1.0
        ];
        let cg_color: id = msg_send![base_color, CGColor];
        let _: () = msg_send![layer, setBackgroundColor: cg_color];
    }
}

pub fn define_things_button_class() {
    INIT.call_once(|| {
        let superclass = class!(NSView);
        let mut decl = ClassDecl::new("ThingsButtonView", superclass).unwrap();
        unsafe {
            decl.add_method(sel!(mouseDown:), mouse_down as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(mouseEntered:), mouse_entered as extern "C" fn(&Object, Sel, id));
            decl.add_method(sel!(mouseExited:), mouse_exited as extern "C" fn(&Object, Sel, id));
        }
        decl.register();
    });
}

pub unsafe fn create_things_button(text: &str, frame: NSRect) -> id {
    define_things_button_class();

    let view: id = msg_send![class!(ThingsButtonView), alloc];
    let view: id = msg_send![view, initWithFrame: frame];
    let _: () = msg_send![view, setWantsLayer: true];

    let layer: id = msg_send![view, layer];
    let base_color: id = msg_send![
        class!(NSColor),
        colorWithCalibratedRed: 0.96 green: 0.96 blue: 0.97 alpha: 1.0
    ];
    let cg_color: id = msg_send![base_color, CGColor];
    let _: () = msg_send![layer, setBackgroundColor: cg_color];
    let _: () = msg_send![layer, setCornerRadius: 9.0];
    let _: () = msg_send![layer, setShadowOpacity: 0.08];
    let _: () = msg_send![layer, setShadowOffset: NSSize::new(0.0, -1.0)];
    let _: () = msg_send![layer, setShadowRadius: 3.0];

    // Texto del botón
    let label_frame = NSRect::new(NSPoint::new(12.0, 10.0), NSSize::new(frame.size.width - 24.0, 20.0));
    let label: id = msg_send![class!(NSTextField), alloc];
    let label: id = msg_send![label, initWithFrame: label_frame];
    let title = NSString::alloc(nil).init_str(text);
    let _: () = msg_send![label, setStringValue: title];
    let _: () = msg_send![label, setBordered: false];
    let _: () = msg_send![label, setEditable: false];
    let _: () = msg_send![label, setBackgroundColor: nil];
    let _: () = msg_send![label, setAlignment: 1];
    let _: () = msg_send![view, addSubview: label];

    // Hover tracking area
    let options = NSTrackingActiveAlways | NSTrackingMouseEnteredAndExited | NSTrackingInVisibleRect;
    let tracking_area: id = msg_send![class!(NSTrackingArea), alloc];
    let tracking_area: id = msg_send![
        tracking_area,
        initWithRect: frame
        options: options
        owner: view
        userInfo: nil
    ];
    let _: () = msg_send![view, addTrackingArea: tracking_area];

    view
}
