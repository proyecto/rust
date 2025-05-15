//! @file objc_shims.rs
//! @brief Envolturas seguras para objc_msgSend con diferentes firmas.
//! @details Este módulo permite invocar métodos Objective-C desde Rust sin usar el macro `msg_send!` ni causar warnings de Clippy.
//! @author GPT
//! @date 2025-05-15

use objc::runtime::{Object, Sel};
use cocoa::base::id;
use std::ffi::c_char;
use cocoa::foundation::NSRect;
use objc::runtime::Class;


#[link(name = "objc")]
unsafe extern "C" {
    #[link_name = "objc_msgSend"]
    pub static OBJC_MSG_SEND: usize;
}

#[inline(always)]
pub unsafe fn objc_msg_send_id(obj: *mut Object, sel: Sel) -> id { unsafe {
    let f: unsafe extern "C" fn(*mut Object, Sel) -> id =
        std::mem::transmute(OBJC_MSG_SEND);
    f(obj, sel)
}}

#[inline(always)]
pub unsafe fn objc_msg_send_usize(obj: *mut Object, sel: Sel) -> usize { unsafe {
    let f: unsafe extern "C" fn(*mut Object, Sel) -> usize =
        std::mem::transmute(OBJC_MSG_SEND);
    f(obj, sel)
}}

#[inline(always)]
pub unsafe fn objc_msg_send_cchar(obj: *mut Object, sel: Sel) -> *const c_char { unsafe {
    let f: unsafe extern "C" fn(*mut Object, Sel) -> *const c_char =
        std::mem::transmute(OBJC_MSG_SEND);
    f(obj, sel)
}}

#[inline(always)]
pub unsafe fn objc_msg_send_void(obj: *mut Object, sel: Sel) { unsafe {
    let f: unsafe extern "C" fn(*mut Object, Sel) =
        std::mem::transmute(OBJC_MSG_SEND);
    f(obj, sel)
}}

#[inline(always)]
pub unsafe fn objc_msg_send_bool(obj: *mut Object, sel: Sel) -> bool { unsafe {
    let f: unsafe extern "C" fn(*mut Object, Sel) -> bool =
        std::mem::transmute(OBJC_MSG_SEND);
    f(obj, sel)
}}

#[link(name = "objc")]
unsafe extern "C" {
    #[link_name = "objc_msgSend"]
    pub fn objc_msg_send_bool_sel(obj: *mut Object, sel: Sel, arg: Sel) -> bool;
}

#[inline(always)]
pub unsafe fn objc_msg_send_void_id(obj: *mut Object, sel: Sel, arg: id) {
    let f: unsafe extern "C" fn(*mut Object, Sel, id) =
        unsafe { std::mem::transmute(OBJC_MSG_SEND) };
    unsafe { f(obj, sel, arg) }
}

#[inline(always)]
pub unsafe fn objc_msg_send_id_no_args(obj: *mut Object, sel: Sel) -> id {
    let f: unsafe extern "C" fn(*mut Object, Sel) -> id =
        unsafe { std::mem::transmute(OBJC_MSG_SEND) };
    unsafe { f(obj, sel) }
}

#[inline(always)]
pub unsafe fn objc_msg_send_void_usize(obj: *mut Object, sel: Sel, arg: usize) {
    let f: unsafe extern "C" fn(*mut Object, Sel, usize) =
        unsafe { std::mem::transmute(OBJC_MSG_SEND) };
    unsafe { f(obj, sel, arg) }
}

#[inline(always)]
pub unsafe fn objc_msg_send_void_bool(obj: *mut Object, sel: Sel, arg: bool) {
    let f: unsafe extern "C" fn(*mut Object, Sel, bool) =
        unsafe { std::mem::transmute(OBJC_MSG_SEND) };
    unsafe { f(obj, sel, arg) }
}

#[inline(always)]
pub unsafe fn objc_msg_send_id_rect(obj: *mut Object, sel: Sel, arg: NSRect) -> id {
    let f: unsafe extern "C" fn(*mut Object, Sel, NSRect) -> id =
        unsafe { std::mem::transmute(OBJC_MSG_SEND) };
    unsafe { f(obj, sel, arg) }
}

pub fn get_class(name: &str) -> *const Class {
    objc::runtime::Class::get(name).expect("Clase no encontrada") 
}

#[inline(always)]
pub unsafe fn objc_msg_send_void_f64(obj: *mut Object, sel: Sel, arg: f64) {
    let f: unsafe extern "C" fn(*mut Object, Sel, f64) =
        unsafe { std::mem::transmute(OBJC_MSG_SEND) };
    unsafe { f(obj, sel, arg) }
}

#[inline(always)]
pub unsafe fn objc_msg_send_id_id(obj: *mut Object, sel: Sel, arg: id) -> id {
    let f: unsafe extern "C" fn(*mut Object, Sel, id) -> id =
        unsafe { std::mem::transmute(OBJC_MSG_SEND) };
    unsafe { f(obj, sel, arg) }
}