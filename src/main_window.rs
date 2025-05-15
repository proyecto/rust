// src/main_window.rs

use crate::constants::{
    SIDEBAR_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH, MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT
};

use cocoa::appkit::{NSBackingStoreType, NSView, NSWindow, NSWindowStyleMask, NSViewHeightSizable, NSViewWidthSizable, NSViewMaxXMargin};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{msg_send, sel, sel_impl, class};
use crate::views::main_sideview;
use crate::views::main_view;

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
            let _: () = msg_send![window, setTitlebarAppearsTransparent: true];
            let _: () = msg_send![window, setMovableByWindowBackground: true];
            window.setTitle_(NSString::alloc(nil).init_str(""));

            let content_view: id = window.contentView();
            content_view.addSubview_(create_split_view(frame));

            let delegate: id = msg_send![create_delegate_class(), new];
            let _: () = msg_send![window, setDelegate: delegate];

            window.makeKeyAndOrderFront_(nil);

            MainWindow { }
        }
    }
}

unsafe fn create_split_view(frame: NSRect) -> id {
    let split_view: id = msg_send![class!(NSSplitView), alloc];
    let split_view: id = msg_send![split_view, initWithFrame: frame];
    let _: () = msg_send![split_view, setAutoresizingMask: NSViewHeightSizable | NSViewWidthSizable];
    let _: () = msg_send![split_view, setVertical: true];
    let _: () = msg_send![split_view, setDividerStyle: 2]; // Thin divider

    let left_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(SIDEBAR_WIDTH, WINDOW_HEIGHT));
    let right_frame = NSRect::new(
        NSPoint::new(SIDEBAR_WIDTH, 0.0),
        NSSize::new(WINDOW_WIDTH - SIDEBAR_WIDTH, WINDOW_HEIGHT),
    );

    let left_view = main_sideview::create(left_frame);
    let _: () = msg_send![left_view, setAutoresizingMask: NSViewHeightSizable | NSViewMaxXMargin];
    let _: () = msg_send![left_view, setFrameSize: NSSize::new(SIDEBAR_WIDTH, frame.size.height)];
    split_view.addSubview_(left_view);

    let right_view = main_view::render_main_view_as_nsview(right_frame);
    let _: () = msg_send![right_view, setAutoresizingMask: NSViewHeightSizable | NSViewWidthSizable];
    split_view.addSubview_(right_view);

    let _: () = msg_send![split_view, adjustSubviews];

    split_view
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
        sel!(windowShouldClose:),
        window_should_close as extern "C" fn(&Object, Sel, id) -> bool,
    );

    CLASS_PTR = decl.register();
    CLASS_PTR
}}
