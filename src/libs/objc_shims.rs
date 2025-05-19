//! @file objc_shims.rs
//! @brief Envolturas seguras para objc_msgSend con diferentes firmas.
//! @details Este módulo permite invocar métodos Objective-C desde Rust sin usar el macro `msg_send!` ni causar warnings de Clippy.
//! @author GPT
//! @date 2025-05-15

use objc::runtime::{Object, Sel};
use cocoa::base::id;
use std::os::raw::{c_char, c_int};
use cocoa::foundation::NSRect;
use objc::runtime::Class;
use cocoa::foundation::NSSize;

#[link(name = "objc")]
unsafe extern "C" {
    // Basic message send variants
    #[link_name = "objc_msgSend"]
    fn objc_msgSend_raw(receiver: *mut Object, sel: Sel, ...) -> *mut Object;
}

/// Safe wrapper for objc_msgSend with different signatures
pub unsafe fn msg_send_id(obj: *mut Object, sel: Sel) -> id {
    unsafe { objc_msgSend_raw(obj, sel) as id }
}

pub unsafe fn msg_send_usize(obj: *mut Object, sel: Sel) -> usize {
    unsafe { objc_msgSend_raw(obj, sel) as usize }
}

pub unsafe fn msg_send_c_char(obj: *mut Object, sel: Sel) -> *const c_char {
    unsafe { objc_msgSend_raw(obj, sel) as *const c_char }
}

pub unsafe fn msg_send_void(obj: *mut Object, sel: Sel) {
    unsafe { let _ = objc_msgSend_raw(obj, sel); }
}

pub unsafe fn msg_send_bool(obj: *mut Object, sel: Sel) -> bool {
    unsafe { !objc_msgSend_raw(obj, sel).is_null() }
}

pub unsafe fn msg_send_bool_sel(obj: *mut Object, sel: Sel, arg: Sel) -> bool {
    unsafe { !objc_msgSend_raw(obj, sel, arg).is_null() }
}

pub unsafe fn msg_send_void_id(obj: *mut Object, sel: Sel, arg: id) {
    unsafe { let _ = objc_msgSend_raw(obj, sel, arg); }
}

pub unsafe fn msg_send_void_usize(obj: *mut Object, sel: Sel, arg: usize) {
    unsafe { let _ = objc_msgSend_raw(obj, sel, arg); }
}

pub unsafe fn msg_send_void_bool(obj: *mut Object, sel: Sel, arg: bool) {
    unsafe { let _ = objc_msgSend_raw(obj, sel, arg as c_int); }
}

pub unsafe fn msg_send_id_rect(obj: *mut Object, sel: Sel, arg: NSRect) -> id {
    unsafe { objc_msgSend_raw(obj, sel, arg) as id }
}

pub unsafe fn msg_send_void_f64(obj: *mut Object, sel: Sel, arg: f64) {
    unsafe { let _ = objc_msgSend_raw(obj, sel, arg); }
}

pub unsafe fn msg_send_id_id(obj: *mut Object, sel: Sel, arg: id) -> id {
    unsafe { objc_msgSend_raw(obj, sel, arg) as id }
}

pub unsafe fn msg_send_void_u64(obj: *mut Object, sel: Sel, arg: u64) {
    unsafe { let _ = objc_msgSend_raw(obj, sel, arg); }
}

pub unsafe fn msg_send_id_id_f64(obj: *mut Object, sel: Sel, arg1: id, arg2: f64) -> id {
    unsafe { objc_msgSend_raw(obj, sel, arg1, arg2) as id }
}

pub unsafe fn msg_send_id_f64_f64(obj: *mut Object, sel: Sel, arg1: f64, arg2: f64) -> id {
    unsafe { objc_msgSend_raw(obj, sel, arg1, arg2) as id }
}

pub unsafe fn msg_send_id_f64_f64_f64_f64(obj: *mut Object, sel: Sel, arg1: f64, arg2: f64, arg3: f64, arg4: f64) -> id {
    unsafe { objc_msgSend_raw(obj, sel, arg1, arg2, arg3, arg4) as id }
}

pub unsafe fn msg_send_void_ns_size(obj: *mut Object, sel: Sel, arg: NSSize) {
    unsafe {
        let _ = objc_msgSend_raw(obj, sel, arg);
    }
}

pub unsafe fn msg_send_id_no_args(obj: id, sel: Sel) -> id {
    unsafe{
        objc_msgSend_raw(obj, sel)
    }
}

pub fn get_class(name: &str) -> *const Class {
    Class::get(name).expect("Clase no encontrada")
}