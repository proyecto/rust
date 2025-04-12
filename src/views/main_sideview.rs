// src/main_sideview.rs

#[link(name = "QuartzCore", kind = "framework")]
unsafe extern "C" {}

use crate::constants::{
        BUTTON_HEIGHT, 
        BUTTON_PADDING_TOP, 
        BUTTON_SPACING,
        LEFT_VIEW_COLOR, 
        BUTTON_MARGIN_LEFT, 
        BUTTON_MARGIN_TOP,
        BUTTON_WIDTH_MARGIN
};

use crate::views::sidebar_button as sidebar_button;
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
    
    let button1 = create_button(view, "Button1", 1);
    let button2 = create_button(view, "Button2", 2);

    sidebar_button::set_active(button1, nil, true);

    view
}

fn create_button(view: id, label: &str, order: i16) -> id {
    unsafe {
        // Obtenemos el frame del contenedor
        let view_frame: NSRect = msg_send![view, frame];

        // Calculamos el ancho basado en el ancho del contenedor, restando el margen
        let width = view_frame.size.width - BUTTON_WIDTH_MARGIN;

        let button_frame = NSRect::new(
            NSPoint::new(BUTTON_MARGIN_LEFT, BUTTON_MARGIN_TOP + (BUTTON_HEIGHT + BUTTON_SPACING) * order as f64),
            NSSize::new(width, BUTTON_HEIGHT),
        );

        let (button, _) = sidebar_button::create_sidebar_button(label, button_frame);
        let _: () = msg_send![button, setAutoresizingMask: NSViewMaxYMargin | NSViewWidthSizable];
        let _: () = msg_send![view, addSubview: button];
        button
    }
}

pub fn render(x: f64, y: f64, width: f64, height: f64) {
    unsafe {
        let frame = NSRect::new(NSPoint::new(x, y), NSSize::new(width, height));
        let _ = create(frame);
    }
}