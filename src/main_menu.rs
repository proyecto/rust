// src/main_menu.rs

use cocoa::appkit::{NSApp, NSMenu, NSMenuItem, NSApplication};
use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;

pub struct MainMenu;

impl MainMenu {
    pub fn setup() {
        unsafe {
            let main_menu: id = NSMenu::new(nil);

            let app_menu = create_menu("Acerca de MiApp", "Salir", "q");
            main_menu.addItem_(app_menu.0);
            let _: () = msg_send![app_menu.0, setSubmenu: app_menu.1];

            let archivo_menu = create_menu_item("Archivo");
            main_menu.addItem_(archivo_menu);

            let edicion_menu = create_menu_item("Edición");
            main_menu.addItem_(edicion_menu);

            let ayuda_menu = create_menu_item("Ayuda");
            main_menu.addItem_(ayuda_menu);

            NSApplication::setMainMenu_(NSApp(), main_menu);
        }
    }
}

unsafe fn create_menu(about_text: &str, quit_text: &str, quit_key: &str) -> (id, id) {
    let app_menu_item: id = NSMenuItem::new(nil);
    let app_menu: id = NSMenu::new(nil);

    let about_item: id = NSMenuItem::alloc(nil)
        .initWithTitle_action_keyEquivalent_(
            NSString::alloc(nil).init_str(about_text),
            sel!(orderFrontStandardAboutPanel:),
            NSString::alloc(nil).init_str(""),
        );
    app_menu.addItem_(about_item);
    app_menu.addItem_(NSMenuItem::separatorItem(nil));

    let quit_item: id = NSMenuItem::alloc(nil)
        .initWithTitle_action_keyEquivalent_(
            NSString::alloc(nil).init_str(quit_text),
            sel!(terminate:),
            NSString::alloc(nil).init_str(quit_key),
        );
    app_menu.addItem_(quit_item);

    (app_menu_item, app_menu)
}

unsafe fn create_menu_item(title: &str) -> id {
    let menu_item: id = NSMenuItem::new(nil);
    let menu: id = NSMenu::new(nil);
    let _: () = msg_send![menu_item, setTitle: NSString::alloc(nil).init_str(title)];
    let _: () = msg_send![menu_item, setSubmenu: menu];
    menu_item
}
