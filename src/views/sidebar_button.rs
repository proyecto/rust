use cocoa::appkit::{NSView, NSTextField, NSViewWidthSizable, NSViewMaxYMargin};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use std::sync::Once;
use std::sync::Mutex;
use std::cell::RefCell;
use once_cell::sync::Lazy;
use crate::constants::{
    SELECTED_BUTTON_COLOR, 
    BUTTON_TEXT_COLOR, 
    LEFT_VIEW_COLOR, 
    LABEL_MARGIN_BOTTOM, 
    LABEL_MARGIN_LEFT, 
    BUTTON_WIDTH_MARGIN,
    BUTTON_SPACING,
    BUTTON_HEIGHT,
    BUTTON_MARGIN_LEFT,
    BUTTON_MARGIN_TOP
};

use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::traits::Action;

thread_local! {
    pub static ACTIONS: RefCell<HashMap<*mut Object, Box<dyn Action>>> = RefCell::new(HashMap::new());
}

static INIT: Once = Once::new();

#[derive(Copy, Clone)]
struct SafeButtonId(id);
unsafe impl Send for SafeButtonId {}
static BUTTONS: Lazy<Mutex<Vec<SafeButtonId>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn define_sidebar_button_class() {
    INIT.call_once(|| {
        let superclass = class!(NSView);
        let mut decl = ClassDecl::new("SidebarButtonView", superclass).unwrap();
        unsafe {
            decl.add_method(sel!(mouseDown:), mouse_down as extern "C" fn(&Object, Sel, id));
        }
        decl.register();
    });
}

pub unsafe fn create_sidebar_button(view: id, text: &str, frame: NSRect, order: i16, action: Box<dyn crate::traits::Action>) -> (id, id) {
    define_sidebar_button_class();

    let view_frame: NSRect = msg_send![view, frame];

    let y_position = BUTTON_MARGIN_TOP + (BUTTON_HEIGHT + BUTTON_SPACING) * order as f64;
    let button_frame = NSRect::new(
        NSPoint::new(BUTTON_MARGIN_LEFT, y_position),
        NSSize::new(view_frame.size.width - BUTTON_WIDTH_MARGIN, BUTTON_HEIGHT),
    );

    let button: id = msg_send![class!(SidebarButtonView), alloc];
    let button: id = msg_send![button, initWithFrame: button_frame];

    // Establecer el identificador visible desde mouse_down
    let id_str: id = NSString::alloc(nil).init_str(text);
    let _: () = msg_send![button, setIdentifier: id_str];
    let _: () = msg_send![button, setWantsLayer: true];

    let layer: id = msg_send![button, layer];
    // Configuración adicional de la capa si es necesario

    let label_frame = NSRect::new(
        NSPoint::new(LABEL_MARGIN_LEFT, LABEL_MARGIN_BOTTOM), 
        NSSize::new(button_frame.size.width - 24.0, 16.0)
    );

    let label: id = msg_send![class!(NSTextField), alloc];
    let label: id = msg_send![label, initWithFrame: label_frame];
    let title = NSString::alloc(nil).init_str(text);
    let _: () = msg_send![label, setStringValue: title];
    let _: () = msg_send![label, setBordered: false];
    let _: () = msg_send![label, setEditable: false];
    let _: () = msg_send![label, setBackgroundColor: nil];
    let _: () = msg_send![label, setAlignment: 0];

    let button_text_color: id = msg_send![
        class!(NSColor),
        colorWithCalibratedRed: BUTTON_TEXT_COLOR.0
        green: BUTTON_TEXT_COLOR.1
        blue: BUTTON_TEXT_COLOR.2
        alpha: 1.0
    ];

    let _: () = msg_send![label, setTextColor: button_text_color];
    let _: () = msg_send![button, addSubview: label];
    let _: () = msg_send![button, setAutoresizingMask: NSViewMaxYMargin | NSViewWidthSizable];
    let _: () = msg_send![view, addSubview: button];

    ACTIONS.with(|map| {
        map.borrow_mut().insert(button, action);
    });

    BUTTONS.lock().unwrap().push(SafeButtonId(button));

    (button, label)
}

extern "C" fn mouse_down(this: &Object, _: Sel, _: id) {
    let identifier: id = unsafe 
    { 
        msg_send![this, identifier] 
    };
    
    if identifier != nil
    {
        let id_this = this as *const _ as *mut Object;

        ACTIONS.with(|map| {
            if let Some(action) = map.borrow().get(&id_this) {
                action.run();
            } else {
                println!("No se encontró acción para este botón.");
            }
        });

        unsafe {
            let buttons = BUTTONS.lock().unwrap();
            for &SafeButtonId(button) in buttons.iter() 
            {
                let is_same = button == id_this;
                set_active(button, nil, is_same);
            }
        }
        
    } else {
        println!("Botón clickeado (sin identificador)");
    }
}

pub unsafe fn set_active(view: id, label: id, active: bool) {
    let layer: id = msg_send![view, layer];

    if active {
        let darkgrey: id = msg_send![
            class!(NSColor),
            colorWithCalibratedRed: 
                SELECTED_BUTTON_COLOR.0 
                green: SELECTED_BUTTON_COLOR.1 
                blue: SELECTED_BUTTON_COLOR.2 
                alpha: 1.0
        ];
        let cg_color: id = msg_send![darkgrey, CGColor];
        let _: () = msg_send![layer, setBackgroundColor: cg_color];

    } else {
        let bggray: id = msg_send![
            class!(NSColor),
            colorWithCalibratedRed: 
                LEFT_VIEW_COLOR.0 
                green: LEFT_VIEW_COLOR.1 
                blue: LEFT_VIEW_COLOR.2 
                alpha: 1.0
        ];
        let cg_color: id = msg_send![bggray, CGColor];
        let _: () = msg_send![layer, setBackgroundColor: cg_color];
    }

    let _: () = msg_send![layer, setCornerRadius: 9.0];
    let _: () = msg_send![layer, setShadowOpacity: 0.08];
    let _: () = msg_send![layer, setShadowOffset: NSSize::new(0.0, -1.0)];
    let _: () = msg_send![layer, setShadowRadius: 3.0];
}


pub unsafe fn sanitize_label(label: &str) -> String {
    label.to_lowercase().replace(" ", "_").replace("-", "_")
}