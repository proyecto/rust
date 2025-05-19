// src/main_window.rs

use crate::constants::{
    SIDEBAR_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH, MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT
};

use cocoa::appkit::{NSBackingStoreType, NSView, NSWindow, NSWindowStyleMask, NSViewHeightSizable, NSViewWidthSizable, NSViewMaxXMargin};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use crate::views::main_sideview;
use crate::views::main_view;
use crate::libs::objc_shims::*;

pub struct MainWindow {
}

impl MainWindow {
    pub fn create() -> Self {
        unsafe {
            let style = NSWindowStyleMask::NSTitledWindowMask
            | NSWindowStyleMask::NSClosableWindowMask
            | NSWindowStyleMask::NSResizableWindowMask
            | NSWindowStyleMask::NSMiniaturizableWindowMask
            | NSWindowStyleMask::NSFullSizeContentViewWindowMask;

            let frame = NSRect::new(
                NSPoint::new(0.0, 0.0),
                NSSize::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            );

            let window: id = NSWindow::alloc(nil)
                .initWithContentRect_styleMask_backing_defer_(
                    frame,
                    style,
                    NSBackingStoreType::NSBackingStoreBuffered,
                    0,
                );

            window.setMinSize_(NSSize::new(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT));
            window.center();
            msg_send_void_bool(window, Sel::register("setTitlebarAppearsTransparent:"), true);
            msg_send_void_bool(window, Sel::register("setMovableByWindowBackground:"), true);
            window.setTitle_(NSString::alloc(nil).init_str(""));

            let content_view: id = window.contentView();
            content_view.addSubview_(create_split_view(frame));

            let delegate_class = create_delegate_class() as *mut Object;
            let delegate: id = msg_send_id(delegate_class, Sel::register("new"));            
            msg_send_void_id(window, Sel::register("setDelegate:"), delegate);

            window.makeKeyAndOrderFront_(nil);

            MainWindow { }
        }
    }
}

unsafe fn create_split_view(frame: NSRect) -> id {
    unsafe{
        let split_view_class = get_class("NSSplitView") as *mut Object;
        let split_view: id = 
            msg_send_id(split_view_class, Sel::register("alloc"));   
        let split_view: id = 
            msg_send_id_rect(split_view, Sel::register("initWithFrame:"), frame);    

        msg_send_void_usize(
            split_view,
            Sel::register("setAutoresizingMask:"),
            (NSViewHeightSizable | NSViewWidthSizable) as usize,
        );

        msg_send_void_bool(split_view, Sel::register("setVertical:"), true);    
        msg_send_void_usize(split_view, Sel::register("setDividerStyle:"), 2);
        
        let left_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(SIDEBAR_WIDTH, WINDOW_HEIGHT));
        let right_frame = NSRect::new(
            NSPoint::new(SIDEBAR_WIDTH, 0.0),
            NSSize::new(WINDOW_WIDTH - SIDEBAR_WIDTH, WINDOW_HEIGHT),
        );

        let left_view = main_sideview::create(left_frame);

        msg_send_void_usize(
            left_view,
            Sel::register("setAutoresizingMask:"),
            (NSViewHeightSizable | NSViewMaxXMargin) as usize,
        );

        let new_size = NSSize::new(SIDEBAR_WIDTH, frame.size.height);

        msg_send_void_ns_size(left_view, Sel::register("setFrameSize:"), new_size);
        split_view.addSubview_(left_view);

        let right_view = main_view::render_main_view_as_nsview(right_frame);
        msg_send_void_usize(
            right_view,
            Sel::register("setAutoresizingMask:"),
            (NSViewHeightSizable | NSViewWidthSizable) as usize,
        );
        split_view.addSubview_(right_view);

        msg_send_void(split_view, Sel::register("adjustSubviews"));

        split_view
    }
}

unsafe fn create_delegate_class() -> *const Class { unsafe {
    static mut CLASS_PTR: *const Class = std::ptr::null();

    if !CLASS_PTR.is_null() {
        return CLASS_PTR;
    }

    let superclass = Class::get("NSObject").unwrap();
    let mut decl = ClassDecl::new("AppDelegate", superclass).unwrap();

    extern "C" fn window_should_close(_this: &Object, _sel: Sel, _sender: id) -> bool {
        std::process::exit(0);
    }

    decl.add_method(
        Sel::register("windowShouldClose:"),
        window_should_close as extern "C" fn(&Object, Sel, id) -> bool,
    );

    CLASS_PTR = decl.register();
    CLASS_PTR
}}
