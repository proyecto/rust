// src/main_sideview.rs

#[link(name = "QuartzCore", kind = "framework")]

use crate::views::sidebar_button as sidebar_button;
use cocoa::appkit::NSView;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::{class, msg_send, sel, sel_impl};
use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable, NSViewMinYMargin, NSViewMaxYMargin};

use crate::constants::{LEFT_VIEW_COLOR};
use crate::traits::action;
use crate::actions::{PrintHello};


pub unsafe fn create(frame: NSRect) -> id {
    let view: id = msg_send![class!(NSView), alloc];
    let view: id = msg_send![view, initWithFrame: frame];
    let _: () = msg_send![view, setFlipped: true];
    let _: () = msg_send![view, setWantsLayer: true];
    let _: () = msg_send![view, setAutoresizingMask: NSViewHeightSizable | NSViewWidthSizable];
   
    // Set the background color
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
    
    let (button1,_) = sidebar_button::create_sidebar_button(view, "Button1", frame, 1, Box::new(PrintHello));
    let (button2,_) = sidebar_button::create_sidebar_button(view, "Button2", frame, 2, Box::new(PrintHello));

    sidebar_button::set_active(button1, nil, true);

    view
}

pub fn render(x: f64, y: f64, width: f64, height: f64) {
    unsafe {
        let frame = NSRect::new(NSPoint::new(x, y), NSSize::new(width, height));
        let _ = create(frame);
    }
}