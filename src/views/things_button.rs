use cocoa::appkit::{NSView, NSTextField};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use std::sync::Once;

static INIT: Once = Once::new();

extern "C" fn mouse_down(this: &Object, _: Sel, _: id) {
    println!("🟦 Botón clickeado");
}

pub fn define_things_button_class() {
    INIT.call_once(|| {
        let superclass = class!(NSView);
        let mut decl = ClassDecl::new("ThingsButtonView", superclass).unwrap();
        unsafe {
            decl.add_method(sel!(mouseDown:), mouse_down as extern "C" fn(&Object, Sel, id));
        }
        decl.register();
    });
}

pub unsafe fn create_things_button(text: &str, frame: NSRect) -> (id, id) {
    define_things_button_class();

    let view: id = msg_send![class!(ThingsButtonView), alloc];
    let view: id = msg_send![view, initWithFrame: frame];
    let _: () = msg_send![view, setWantsLayer: true];

    let layer: id = msg_send![view, layer];
    set_active(view, nil, false); // temp label = nil

    // Texto centrado
    let label_frame = NSRect::new(NSPoint::new(12.0, 10.0), NSSize::new(frame.size.width - 24.0, 20.0));
    let label: id = msg_send![class!(NSTextField), alloc];
    let label: id = msg_send![label, initWithFrame: label_frame];
    let title = NSString::alloc(nil).init_str(text);
    let _: () = msg_send![label, setStringValue: title];
    let _: () = msg_send![label, setBordered: false];
    let _: () = msg_send![label, setEditable: false];
    let _: () = msg_send![label, setBackgroundColor: nil];
    let _: () = msg_send![label, setAlignment: 1];

    let black: id = msg_send![class!(NSColor), blackColor];
    let _: () = msg_send![label, setTextColor: black];
    let _: () = msg_send![view, addSubview: label];

    (view, label)
}

pub unsafe fn set_active(view: id, label: id, active: bool) {
    let layer: id = msg_send![view, layer];

    if active {
        let blue: id = msg_send![
            class!(NSColor),
            colorWithCalibratedRed: 0.00 green: 0.48 blue: 1.00 alpha: 1.0
        ];
        let cg_color: id = msg_send![blue, CGColor];
        let _: () = msg_send![layer, setBackgroundColor: cg_color];

        if label != nil {
            let white: id = msg_send![class!(NSColor), whiteColor];
            let _: () = msg_send![label, setTextColor: white];
        }
    } else {
        let gray: id = msg_send![
            class!(NSColor),
            colorWithCalibratedRed: 0.96 green: 0.96 blue: 0.97 alpha: 1.0
        ];
        let cg_color: id = msg_send![gray, CGColor];
        let _: () = msg_send![layer, setBackgroundColor: cg_color];

        if label != nil {
            let black: id = msg_send![class!(NSColor), blackColor];
            let _: () = msg_send![label, setTextColor: black];
        }
    }

    let _: () = msg_send![layer, setCornerRadius: 9.0];
    let _: () = msg_send![layer, setShadowOpacity: 0.08];
    let _: () = msg_send![layer, setShadowOffset: NSSize::new(0.0, -1.0)];
    let _: () = msg_send![layer, setShadowRadius: 3.0];
}
