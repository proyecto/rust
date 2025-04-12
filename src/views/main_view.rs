use crate::constants::{WINDOW_WIDTH, WINDOW_HEIGHT, SIDEBAR_WIDTH, RIGHT_VIEW_COLOR};
use cocoa::base::nil;
use objc::{class, msg_send, sel, sel_impl};
use crate::views::main_sideview as side_view;
use cocoa::appkit::NSView;
use cocoa::foundation::NSRect;
use objc::runtime::Object;
use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable};

pub fn render_main_view() {
    // Renderiza el fondo blanco
    render_background((1.0, 1.0, 1.0)); // Blanco

    // Renderiza el side_view en el lado derecho
    let side_view_x_position = WINDOW_WIDTH - SIDEBAR_WIDTH;
    side_view::render(side_view_x_position, 0.0, SIDEBAR_WIDTH, WINDOW_HEIGHT);
}

pub fn render_main_view_as_nsview(frame: NSRect) -> *mut Object {
    unsafe {
        let view: *mut Object = NSView::alloc(nil).initWithFrame_(frame);
        let _: () = msg_send![view, setWantsLayer: true];
        let _: () = msg_send![view, setAutoresizingMask: NSViewHeightSizable | NSViewWidthSizable];
        let layer: *mut Object = msg_send![view, layer];

        let color: *mut Object = msg_send![
            class!(NSColor),
            colorWithRed: RIGHT_VIEW_COLOR.0 green: RIGHT_VIEW_COLOR.1 blue: RIGHT_VIEW_COLOR.2 alpha: 1.0
        ];
        let cg_color: *mut Object = msg_send![color, CGColor];
        let _: () = msg_send![layer, setBackgroundColor: cg_color];

        view
    }
}

fn render_background(color: (f64, f64, f64)) {
    // Implementación para renderizar un fondo con el color especificado
}
