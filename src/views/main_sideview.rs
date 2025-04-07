// src/main_sideview.rs

#[link(name = "QuartzCore", kind = "framework")]
unsafe extern "C" {}

use crate::constants::{BUTTON_HEIGHT, BUTTON_MARGIN_TOP, BUTTON_SPACING,LEFT_VIEW_COLOR};
use crate::views::things_button as things_button;

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

    let button_frame = NSRect::new(NSPoint::new(10.0, frame.size.height - 60.0), NSSize::new(frame.size.width - 20.0, 40.0));
    let things_button = things_button::create_things_button("Hoy", button_frame);
    let _: () = msg_send![view, addSubview: things_button];

    view
}


pub fn render(x: f64, y: f64, width: f64, height: f64) {
    unsafe {
        let frame = NSRect::new(NSPoint::new(x, y), NSSize::new(width, height));
        let _ = create(frame);
    }
}