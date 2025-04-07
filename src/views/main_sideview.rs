// src/main_sideview.rs

#[link(name = "QuartzCore", kind = "framework")]
extern "C" {}

use crate::constants::{BUTTON_HEIGHT, BUTTON_MARGIN_TOP, BUTTON_SPACING,LEFT_VIEW_COLOR};

use cocoa::appkit::NSView;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::{class, msg_send, sel, sel_impl};


pub unsafe fn create(frame: NSRect) -> id {
    let view: id = msg_send![class!(NSView), alloc];
    let view: id = msg_send![view, initWithFrame: frame];
    let _: () = msg_send![view, setWantsLayer: true];

    let background_color: id = msg_send![
        class!(NSColor),
        colorWithCalibratedRed: LEFT_VIEW_COLOR.0
        green: LEFT_VIEW_COLOR.1
        blue: LEFT_VIEW_COLOR.2
        alpha: 1.0
    ];
    let cg_color: id = msg_send![background_color, CGColor];
    let layer: id = msg_send![class!(CALayer), alloc];
    let layer: id = msg_send![layer, init];
    let _: () = msg_send![layer, setBackgroundColor: cg_color];
    let _: () = msg_send![view, setLayer: layer];

    view.addSubview_(create_sideview_button("Botón Cocoa", frame));
    view
}

pub unsafe fn create_sideview_button(text: &str, container_frame: NSRect) -> id {
    let button_frame = NSRect::new(
        NSPoint::new(10.0, container_frame.size.height - BUTTON_MARGIN_TOP),
        NSSize::new(container_frame.size.width - BUTTON_SPACING, BUTTON_HEIGHT),
    );

    let button: id = msg_send![class!(NSButton), alloc];
    let button: id = msg_send![button, initWithFrame: button_frame];
    let title = NSString::alloc(nil).init_str(text);
    let _: () = msg_send![button, setTitle: title];
    button
}

pub fn render(x: f64, y: f64, width: f64, height: f64) {
    unsafe {
        let frame = NSRect::new(NSPoint::new(x, y), NSSize::new(width, height));
        let _ = create(frame);
    }
}