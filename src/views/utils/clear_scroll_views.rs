use cocoa::base::id;
use objc::{msg_send, sel, sel_impl};
use std::ffi::CStr;

pub fn clear_scroll_views(view: id) {
    unsafe {
        let subviews: id = msg_send![view, subviews];
        let count: usize = msg_send![subviews, count];

        for i in 0..count {
            let subview: id = msg_send![subviews, objectAtIndex: i];
            let class_name: *const std::os::raw::c_char = msg_send![subview, className];
            let rust_str = CStr::from_ptr(class_name).to_string_lossy();

            if rust_str.contains("NSScrollView") {
                let _: () = msg_send![subview, removeFromSuperview];
            }
        }
    }
}