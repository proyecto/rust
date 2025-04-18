// src/main_sideview.rs

#[link(name = "QuartzCore", kind = "framework")]

use crate::views::sidebar_button as sidebar_button;
use cocoa::appkit::NSView;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::{class, msg_send, sel, sel_impl};
use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable, NSViewMinYMargin, NSViewMaxYMargin};

use crate::constants::{LEFT_VIEW_COLOR, get_buttons};
use crate::actions::PrintHello;
use crate::actions::PrintHello2;
use crate::traits::Action;


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
    

    for (index, label, action) in get_buttons() 
    {
        let button_id = format!("button_{}", sidebar_button::sanitize_label(label));
        let (button_id,_) = sidebar_button::create_sidebar_button(view, label, frame, index, action);
        if(index == 1)
        {
            sidebar_button::set_active(button_id, nil, true);
        }
    }

    view
}

pub fn render(x: f64, y: f64, width: f64, height: f64) {
    unsafe {
        let frame = NSRect::new(NSPoint::new(x, y), NSSize::new(width, height));
        let _ = create(frame);
    }
}