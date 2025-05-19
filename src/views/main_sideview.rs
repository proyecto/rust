// src/main_sideview.rs

#[link(name = "QuartzCore", kind = "framework")]
unsafe extern "C" {}

use crate::views::sidebar_button as sidebar_button;
use cocoa::base::{id};
use cocoa::foundation::NSRect;
use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable};
use crate::constants::{LEFT_VIEW_COLOR, get_buttons};
use crate::libs::objc_shims::*;
use objc::runtime::{Object, Sel};


pub unsafe fn create(frame: NSRect) -> id 
{
    let calayer_class = get_class("CALayer") as *mut Object;

    unsafe 
    {
        let view: id = msg_send_id_rect(
            msg_send_id(get_class("NSView") as *mut Object, Sel::register("alloc")),
            Sel::register("initWithFrame:"),
            frame,
        );
        msg_send_void_bool(view, Sel::register("setFlipped:"), true);
        msg_send_void_bool(view, Sel::register("setWantsLayer:"), true);
        msg_send_void_usize(view,Sel::register("setAutoresizingMask:"),
        (NSViewHeightSizable | NSViewWidthSizable) as usize);

        let nscolor_class = get_class("NSColor") as *mut Object;
        let background_color: id = 
            msg_send_id_f64_f64_f64_f64(
                nscolor_class,
                Sel::register("colorWithCalibratedRed:green:blue:alpha:"),
                LEFT_VIEW_COLOR.0,
                LEFT_VIEW_COLOR.1,
                LEFT_VIEW_COLOR.2,
                1.0,
        );
        let cg_color: id = msg_send_id(background_color, Sel::register("CGColor"));
        let layer: id = msg_send_id(calayer_class, Sel::register("alloc"));
        let layer: id = msg_send_id(layer, Sel::register("init"));
        msg_send_void_id(layer, Sel::register("setBackgroundColor:"), cg_color);
        msg_send_void_id(view, Sel::register("setLayer:"), layer);

        for (index, label, action) in get_buttons() 
        {
            //let button_id = format!("button_{}", unsafe{sidebar_button::sanitize_label(label)});
            let (button_id,_) = sidebar_button::create_sidebar_button(view, label, index, action);
            if index == 1
            {
                sidebar_button::set_active(button_id, true);
            }
        }    
    view    
    }
}