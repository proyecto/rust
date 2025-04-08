// src/main_sideview.rs

#[link(name = "QuartzCore", kind = "framework")]
unsafe extern "C" {}

use crate::constants::{BUTTON_HEIGHT, BUTTON_MARGIN_TOP, BUTTON_SPACING,LEFT_VIEW_COLOR};
use crate::views::things_button as things_button;
use cocoa::appkit::NSView;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::{class, msg_send, sel, sel_impl};
use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable, NSViewMinYMargin, NSViewMaxYMargin};


pub unsafe fn create(frame: NSRect) -> id {
    let view: id = msg_send![class!(NSView), alloc];
    let view: id = msg_send![view, initWithFrame: frame];
    let _: () = msg_send![view, setFlipped: true];
    let _: () = msg_send![view, setWantsLayer: true];
    let _: () = msg_send![view, setAutoresizingMask: NSViewHeightSizable | NSViewWidthSizable];
   

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

    // Create the button
    let button1_frame = NSRect::new(
        NSPoint::new(10.0, 10.0),
        NSSize::new(frame.size.width - 20.0, 40.0)
    );
    let (button1, _) = things_button::create_things_button("Button1", button1_frame);
    let _: () = msg_send![button1, setAutoresizingMask: NSViewMaxYMargin | NSViewWidthSizable];
    let _: () = msg_send![view, addSubview: button1];
    
    let button2_frame = NSRect::new(
        NSPoint::new(10.0, 60.0),
        NSSize::new(frame.size.width - 20.0, 40.0)
    );
    // Create the second button
    let (button2, _) = things_button::create_things_button("Button2", button2_frame);
    let _: () = msg_send![button2, setAutoresizingMask: NSViewMaxYMargin | NSViewWidthSizable];
    let _: () = msg_send![view, addSubview: button2];

    things_button::set_active(button1, nil, true);

    view
}


pub fn render(x: f64, y: f64, width: f64, height: f64) {
    unsafe {
        let frame = NSRect::new(NSPoint::new(x, y), NSSize::new(width, height));
        let _ = create(frame);
    }
}