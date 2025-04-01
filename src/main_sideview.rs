// src/main_sideview.rs

#[link(name = "QuartzCore", kind = "framework")]
extern "C" {}

use crate::constants::LEFT_VIEW_COLOR;
use cocoa::appkit::{NSButton, NSColor, NSView};
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
        NSPoint::new(10.0, container_frame.size.height - 60.0),
        NSSize::new(container_frame.size.width - 20.0, 40.0),
    );

    let button: id = msg_send![class!(NSButton), alloc];
    let button: id = msg_send![button, initWithFrame: button_frame];
    let title = NSString::alloc(nil).init_str(text);
    let _: () = msg_send![button, setTitle: title];
    button
}
