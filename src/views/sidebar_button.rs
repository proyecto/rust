use cocoa::appkit::{NSViewWidthSizable, NSViewMaxYMargin};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel};
use objc::{msg_send, sel, sel_impl};
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
    LABEL_FONT_SIZE, 
    BUTTON_WIDTH_MARGIN,
    BUTTON_SPACING,
    BUTTON_HEIGHT,
    BUTTON_MARGIN_LEFT,
    BUTTON_MARGIN_TOP
};

use std::collections::HashMap;
use crate::traits::Action;
use crate::views::clear_scroll_views::clear_scroll_views;
use crate::libs::objc_shims::*;

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
        let superclass = get_class("NSView");
        let superclass_ref = unsafe { &*superclass };
        let mut decl = ClassDecl::new("SidebarButtonView", superclass_ref).unwrap();
        unsafe {
            decl.add_method(Sel::register("mouseDown:"), mouse_down as extern "C" fn(&Object, Sel, id));
        }
        decl.register();
    });
}

pub unsafe fn create_sidebar_button(view: id, text: &str, order: i16, action: Box<dyn crate::traits::Action>) -> (id, id) {
    define_sidebar_button_class();

    let view_frame: NSRect = msg_send![view, frame];

    let y_position = BUTTON_MARGIN_TOP + (BUTTON_HEIGHT + BUTTON_SPACING) * order as f64;
    let button_frame = NSRect::new(
        NSPoint::new(BUTTON_MARGIN_LEFT, y_position),
        NSSize::new(view_frame.size.width - BUTTON_WIDTH_MARGIN, BUTTON_HEIGHT),
    );

    let sidebar_button_class = get_class("SidebarButtonView") as *mut Object;
    let button: id = unsafe {
        msg_send_id(sidebar_button_class, Sel::register("alloc"))
    };    
    let button: id = unsafe {
        msg_send_id_rect(button, Sel::register("initWithFrame:"), button_frame)
    };
    // Establecer el identificador visible desde mouse_down
    let id_str: id = unsafe{NSString::alloc(nil).init_str(text)};
    unsafe {
        msg_send_void_id(button, Sel::register("setIdentifier:"), id_str);
        msg_send_void_bool(button, Sel::register("setWantsLayer:"), true);
    }
    // Configuración adicional de la capa si es necesario

    let label_frame = NSRect::new(
        NSPoint::new(LABEL_MARGIN_LEFT, LABEL_MARGIN_BOTTOM), 
        NSSize::new(button_frame.size.width - 24.0, 16.0)
    );

    let label_class = get_class("NSTextField");

    let label = unsafe {
        let alloc = msg_send_id(label_class as *mut _, Sel::register("alloc"));
        msg_send_id_rect(alloc, Sel::register("initWithFrame:"), label_frame)
    };

    let title = unsafe{NSString::alloc(nil).init_str(text)};



    unsafe {
        // setStringValue: (NSString*)
        msg_send_void_id(label, Sel::register("setStringValue:"), title);

        // setBordered: (BOOL)
        msg_send_void_bool(label, Sel::register("setBordered:"), false);

        // setEditable: (BOOL)
        msg_send_void_bool(label, Sel::register("setEditable:"), false);

        // setBackgroundColor: (NSColor*), puedes pasar nil como `std::ptr::null_mut()`
        msg_send_void_id(label, Sel::register("setBackgroundColor:"), std::ptr::null_mut());

        // setAlignment: (NSInteger), puede usarse usize
        msg_send_void_usize(label, Sel::register("setAlignment:"), 0);
    }

    let color_cls = get_class("NSColor");
    let button_text_color = unsafe{msg_send_id_f64_f64_f64_f64(
        color_cls as *mut _,
        Sel::register("colorWithCalibratedRed:green:blue:alpha:"),
        BUTTON_TEXT_COLOR.0,
        BUTTON_TEXT_COLOR.1,
        BUTTON_TEXT_COLOR.2,
        1.0,
    )};

    unsafe {
        crate::libs::objc_shims::msg_send_void_id(label as *mut _, Sel::register("setTextColor:"), button_text_color);
    }    

    let font_size: f64 = LABEL_FONT_SIZE;
    let font_name = unsafe {
        NSString::alloc(nil).init_str("Helvetica")
    };

    let font = unsafe{msg_send_id_id_f64(
        get_class("NSFont") as *mut _,
        Sel::register("fontWithName:size:"),
        font_name,
        font_size,
    )};

    unsafe{
        msg_send_void_id(label as *mut _, Sel::register("setFont:"), font);
    }
    
    if font != nil {
        unsafe
        {
            msg_send_void_id(label as *mut _, Sel::register("setFont:"), font);
        }
    } else {
        println!("Fuente no encontrada");
    }
    
    unsafe{
        msg_send_void_id(button as *mut _, Sel::register("addSubview:"), label);
        msg_send_void_u64(
            button as *mut _,
            Sel::register("setAutoresizingMask:"),
            NSViewMaxYMargin | NSViewWidthSizable
        );
        msg_send_void_id(view as *mut _, Sel::register("addSubview:"), button);
    }

    ACTIONS.with(|map| {
        map.borrow_mut().insert(button, action);
    });

    BUTTONS.lock().unwrap().push(SafeButtonId(button));

    (button, label)
}

extern "C" fn mouse_down(this: &Object, _: Sel, _: id) {
    unsafe { 
        let app = cocoa::appkit::NSApp();
        let window: id = msg_send_id(app as *mut _, Sel::register("mainWindow"));
        let content_view: id = msg_send_id(window as *mut _, Sel::register("contentView"));

        clear_scroll_views(content_view); // limpia lo que había antes
        
        let identifier: id = msg_send_id(this as *const _ as *mut _, Sel::register("identifier"));
        
        if identifier != nil
        {
            let id_this = this as *const _ as *mut Object;

            ACTIONS.with(|map| {
                if let Some(action) = map.borrow().get(&id_this) {
                    let _ = action.run();
                    action.render_view(); // pinta lo que corresponde a ese botón
                } else {
                    println!("No se encontró acción para este botón.");
                }
            });


            let buttons = BUTTONS.lock().unwrap();
            for &SafeButtonId(button) in buttons.iter() 
            {
                let is_same = button == id_this;
                set_active(button, is_same);
            }
            
        } else {
            println!("Botón clickeado (sin identificador)");
        }
    };
}

pub unsafe fn set_active(view: id, active: bool) {

        let layer: id =   unsafe {msg_send_id(view as *mut _, Sel::register("layer"))};


    if active {
        unsafe {
            let ns_color_class = get_class("NSColor");
            let darkgrey = msg_send_id_f64_f64_f64_f64(
                ns_color_class as *mut _,
                Sel::register("colorWithCalibratedRed:green:blue:alpha:"),
                SELECTED_BUTTON_COLOR.0,
                SELECTED_BUTTON_COLOR.1,
                SELECTED_BUTTON_COLOR.2,
                1.0,
            );
            let cg_color: id = msg_send_id(darkgrey as *mut _, Sel::register("CGColor"));            
            msg_send_void_id(layer as *mut _, Sel::register("setBackgroundColor:"), cg_color);       
        }

    } else {
        unsafe {
            let ns_color_class = get_class("NSColor");
            let bggray = msg_send_id_f64_f64_f64_f64(
                ns_color_class as *mut _,
                Sel::register("colorWithCalibratedRed:green:blue:alpha:"),
                LEFT_VIEW_COLOR.0,
                LEFT_VIEW_COLOR.1,
                LEFT_VIEW_COLOR.2,
                1.0,
            );

            let cg_color = msg_send_id(bggray as *mut _, Sel::register("CGColor"));
            msg_send_void_id(layer as *mut _, Sel::register("setBackgroundColor:"), cg_color);
        }
    }

    unsafe {
        msg_send_void_f64(layer as *mut _, Sel::register("setCornerRadius:"), 5.0);
        msg_send_void_f64(layer as *mut _, Sel::register("setShadowOpacity:"), 0.08);
        msg_send_void_ns_size(layer as *mut _,
        Sel::register("setShadowOffset:"),NSSize::new(0.0, -1.0));
        msg_send_void_f64(layer as *mut _, Sel::register("setShadowRadius:"), 0.5);
    }
}

pub unsafe fn sanitize_label(label: &str) -> String {
    label.to_lowercase().replace(" ", "_").replace("-", "_")
}


