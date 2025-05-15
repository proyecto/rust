use objc::runtime::{Object, Sel};
use std::os::raw::c_char;
use std::ffi::CStr;
use cocoa::foundation::NSArray;
use cocoa::base::id;
use crate::cocoa::appkit::NSView;
use objc::{msg_send, sel, sel_impl};


pub fn clear_scroll_views(view: id) 
{
    println!("🔍 Eliminando subviews...");

    unsafe extern "C" {
        fn objc_msgSend(obj: *mut Object, sel: Sel) -> *const c_char;
    }

    unsafe 
    {
        let subviews: id = msg_send![view, subviews];
        let count: usize = msg_send![subviews, count];

        for i in 0..count {
            let subviews = subviews as id; // Asegura que es un id para que NSArray extienda el trait
            let subview: id = NSArray::objectAtIndex(subviews, i.try_into().unwrap());
            let class_name = {
                let f = objc_msgSend as unsafe extern "C" fn(*mut Object, Sel) -> *const c_char;
                f(subview, sel!(className))
            };
            let rust_str = CStr::from_ptr(class_name).to_string_lossy();

            if rust_str.contains("NSScrollView") 
            {
                subview.removeFromSuperview();
            }
        }
    }
}