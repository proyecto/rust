use cocoa::appkit::{NSView, NSTextField};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use std::sync::Once;
use crate::constants::{SELECTED_BUTTON_COLOR, BUTTON_TEXT_COLOR, LEFT_VIEW_COLOR};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static INIT: Once = Once::new();

#[derive(Copy, Clone)]
struct SafeButtonId(id);
unsafe impl Send for SafeButtonId {}
static BUTTONS: Lazy<Mutex<Vec<SafeButtonId>>> = Lazy::new(|| Mutex::new(Vec::new()));

extern "C" fn mouse_down(this: &Object, _: Sel, _: id) {
    let identifier: id = unsafe { msg_send![this, identifier] };
    if identifier != nil {
        let c_str: *const std::os::raw::c_char = unsafe { msg_send![identifier, UTF8String] };
        let rust_str = unsafe { std::ffi::CStr::from_ptr(c_str).to_string_lossy() };
        println!("Botón clickeado: {}", rust_str);

        let id_this = this as *const _ as *mut Object;

        unsafe {
            let buttons = BUTTONS.lock().unwrap();
            for &SafeButtonId(button) in buttons.iter() {
                let is_same = button == id_this;
                set_active(button, nil, is_same);
            }
        }
    } else {
        println!("Botón clickeado (sin identificador)");
    }
}

pub fn define_sidebar_button_class() {
    INIT.call_once(|| {
        let superclass = class!(NSView);
        let mut decl = ClassDecl::new("ThingsButtonView", superclass).unwrap();
        unsafe {
            decl.add_method(sel!(mouseDown:), mouse_down as extern "C" fn(&Object, Sel, id));
        }
        decl.register();
    });
}

pub unsafe fn create_sidebar_button(text: &str, frame: NSRect) -> (id, id) {
    define_sidebar_button_class();

    let view: id = msg_send![class!(ThingsButtonView), alloc];
    let view: id = msg_send![view, initWithFrame: frame];

    // Establecer el identificador visible desde mouse_down
    let id_str: id = NSString::alloc(nil).init_str(text);
    let _: () = msg_send![view, setIdentifier: id_str];

    let _: () = msg_send![view, setWantsLayer: true];

    let layer: id = msg_send![view, layer];
    let label_frame = NSRect::new(
        NSPoint::new(12.0, 6.0), 
        NSSize::new(frame.size.width - 24.0, 16.0)
    );

    let label: id = msg_send![class!(NSTextField), alloc];
    let label: id = msg_send![label, initWithFrame: label_frame];
    let title = NSString::alloc(nil).init_str(text);
    let _: () = msg_send![label, setStringValue: title];
    let _: () = msg_send![label, setBordered: false];
    let _: () = msg_send![label, setEditable: false];
    let _: () = msg_send![label, setBackgroundColor: nil];
    let _: () = msg_send![label, setAlignment: 0];

    let black: id = msg_send![class!(NSColor), blackColor];
    let _: () = msg_send![label, setTextColor: black];
    let _: () = msg_send![view, addSubview: label];

    BUTTONS.lock().unwrap().push(SafeButtonId(view));

    (view, label)
}

pub unsafe fn set_active(view: id, label: id, active: bool) {
    let layer: id = msg_send![view, layer];

    if active {
        let darkgrey: id = msg_send![
            class!(NSColor),
            colorWithCalibratedRed: SELECTED_BUTTON_COLOR.0 green: SELECTED_BUTTON_COLOR.1 blue: SELECTED_BUTTON_COLOR.2 alpha: 1.0
        ];
        let cg_color: id = msg_send![darkgrey, CGColor];
        let _: () = msg_send![layer, setBackgroundColor: cg_color];

        if label != nil {
            let white: id = msg_send![class!(NSColor), whiteColor];
            let _: () = msg_send![label, setTextColor: white];
        }
    } else {
        let bggray: id = msg_send![
            class!(NSColor),
            colorWithCalibratedRed: LEFT_VIEW_COLOR.0 green: LEFT_VIEW_COLOR.1 blue: LEFT_VIEW_COLOR.2 alpha: 1.0
        ];
        let cg_color: id = msg_send![bggray, CGColor];
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
