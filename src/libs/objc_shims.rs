//! @file objc_shims.rs
//! @brief Envolturas seguras para objc_msgSend con diferentes firmas.
//! @details Este módulo permite invocar métodos Objective-C desde Rust sin usar el macro `msg_send!` ni causar warnings de Clippy.
//! @author GPT
//! @date 2025-05-15

use objc::runtime::{Object, Sel};
use cocoa::base::id;
use std::ffi::c_char;

#[link(name = "objc")]
unsafe extern "C" {
    #[link_name = "objc_msgSend"]
    static OBJC_MSG_SEND: usize;
}

#[inline(always)]
pub unsafe fn objc_msg_send_id(obj: *mut Object, sel: Sel) -> id {
    let f: unsafe extern "C" fn(*mut Object, Sel) -> id =
        std::mem::transmute(OBJC_MSG_SEND);
    f(obj, sel)
}

#[inline(always)]
pub unsafe fn objc_msg_send_usize(obj: *mut Object, sel: Sel) -> usize {
    let f: unsafe extern "C" fn(*mut Object, Sel) -> usize =
        std::mem::transmute(OBJC_MSG_SEND);
    f(obj, sel)
}

#[inline(always)]
pub unsafe fn objc_msg_send_cchar(obj: *mut Object, sel: Sel) -> *const c_char {
    let f: unsafe extern "C" fn(*mut Object, Sel) -> *const c_char =
        std::mem::transmute(OBJC_MSG_SEND);
    f(obj, sel)
}

#[inline(always)]
pub unsafe fn objc_msg_send_void(obj: *mut Object, sel: Sel) {
    let f: unsafe extern "C" fn(*mut Object, Sel) =
        std::mem::transmute(OBJC_MSG_SEND);
    f(obj, sel)
}

#[inline(always)]
pub unsafe fn objc_msg_send_bool(obj: *mut Object, sel: Sel) -> bool {
    let f: unsafe extern "C" fn(*mut Object, Sel) -> bool =
        std::mem::transmute(OBJC_MSG_SEND);
    f(obj, sel)
}
