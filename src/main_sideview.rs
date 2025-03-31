// src/main_sideview.rs

use cocoa::appkit::{NSButton, NSView};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::{class, msg_send, sel, sel_impl};

const BUTTON_HEIGHT: f64 = 40.0;
const BUTTON_MARGIN_TOP: f64 = 20.0;
const BUTTON_SPACING: f64 = 10.0;

#[repr(u64)]
enum NSButtonType {
    MomentaryPushIn = 0,
}

pub unsafe fn create(frame: NSRect) -> id {
    // Crear la vista contenedora (barra lateral)
    let view: id = NSView::alloc(nil);
    let view: id = cocoa::appkit::NSView::initWithFrame_(view, frame);
    let _: () = msg_send![view, setWantsLayer: true];
    let layer: id = msg_send![view, layer];
    let gray: id = msg_send![class!(NSColor), grayColor];
    let cg_color: id = msg_send![gray, CGColor];
    let _: () = msg_send![layer, setBackgroundColor: cg_color];

    // Crear 5 botones
    for i in 0..5 {
        let label = format!("Etiqueta {}", i + 1);
        let y = frame.size.height - BUTTON_MARGIN_TOP - ((BUTTON_HEIGHT + BUTTON_SPACING) * i as f64);
        let button_frame = NSRect::new(
            NSPoint::new(10.0, y),
            NSSize::new(frame.size.width - 20.0, BUTTON_HEIGHT),
        );

        let button: id = NSButton::alloc(nil);
        let button: id = cocoa::appkit::NSView::initWithFrame_(button, button_frame);
        let _: () = msg_send![button, setButtonType: NSButtonType::MomentaryPushIn as u64];
        let _: () = msg_send![button, setTitle: NSString::alloc(nil).init_str(&label)];

        view.addSubview_(button);
    }

    view
}
