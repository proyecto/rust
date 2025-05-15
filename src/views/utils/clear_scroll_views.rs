use objc::runtime::{Object, Sel};
use std::os::raw::c_char;
use std::ffi::CStr;
use cocoa::foundation::NSArray;
use cocoa::base::id;
use crate::cocoa::appkit::NSView;
use crate::libs::objc_shims::*;


pub fn clear_scroll_views(view: id) {
    println!("🔍 Eliminando subviews...");

    unsafe {
        // subviews = [view subviews]
        let subviews = objc_msg_send_id(view as *mut Object, Sel::register("subviews"));

        // count = [subviews count]
        let count = objc_msg_send_usize(subviews as *mut Object, Sel::register("count"));

        for i in 0..count {
            let subview: id = NSArray::objectAtIndex(subviews, i.try_into().unwrap());

            let class_name_ptr = objc_msg_send_cchar(subview as *mut Object, Sel::register("className"));
            let rust_str = CStr::from_ptr(class_name_ptr).to_string_lossy();

            if rust_str.contains("NSScrollView") {
                objc_msg_send_void(subview as *mut Object, Sel::register("removeFromSuperview"));
                println!("🔍 Eliminando subview: {}", rust_str);
            }
        }
    }
}
