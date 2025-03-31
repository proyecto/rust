// src/main_menu.rs

use cocoa::appkit::{NSApp, NSMenu, NSMenuItem, NSApplication};
use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;

pub struct MainMenu;

impl MainMenu {
    /// Configura y asigna el menú principal de la aplicación.
    pub fn setup() {
        unsafe {
            let main_menu: id = NSMenu::new(nil);

            // --- Menú de la aplicación ---
            let app_menu_item: id = NSMenuItem::new(nil);
            main_menu.addItem_(app_menu_item);
            let app_menu: id = NSMenu::new(nil);

            let about_title = NSString::alloc(nil).init_str("Acerca de MiApp");
            let about_item: id = NSMenuItem::alloc(nil)
                .initWithTitle_action_keyEquivalent_(
                    about_title,
                    sel!(orderFrontStandardAboutPanel:),
                    NSString::alloc(nil).init_str(""),
                );
            app_menu.addItem_(about_item);

            app_menu.addItem_(NSMenuItem::separatorItem(nil));

            let quit_title = NSString::alloc(nil).init_str("Salir");
            let quit_item: id = NSMenuItem::alloc(nil)
                .initWithTitle_action_keyEquivalent_(
                    quit_title,
                    sel!(terminate:),
                    NSString::alloc(nil).init_str("q"),
                );
            app_menu.addItem_(quit_item);

            let _: () = msg_send![app_menu_item, setSubmenu: app_menu];

            // --- Menú "Archivo" ---
            let archivo_menu_item: id = NSMenuItem::new(nil);
            main_menu.addItem_(archivo_menu_item);
            let archivo_menu: id = NSMenu::new(nil);
            let _: () = msg_send![archivo_menu_item, setTitle: NSString::alloc(nil).init_str("Archivo")];
            let _: () = msg_send![archivo_menu_item, setSubmenu: archivo_menu];

            // --- Menú "Edición" ---
            let edicion_menu_item: id = NSMenuItem::new(nil);
            main_menu.addItem_(edicion_menu_item);
            let edicion_menu: id = NSMenu::new(nil);
            let _: () = msg_send![edicion_menu_item, setTitle: NSString::alloc(nil).init_str("Edición")];
            let _: () = msg_send![edicion_menu_item, setSubmenu: edicion_menu];

            // --- Menú "Ayuda" ---
            let ayuda_menu_item: id = NSMenuItem::new(nil);
            main_menu.addItem_(ayuda_menu_item);
            let ayuda_menu: id = NSMenu::new(nil);
            let _: () = msg_send![ayuda_menu_item, setTitle: NSString::alloc(nil).init_str("Ayuda")];
            let _: () = msg_send![ayuda_menu_item, setSubmenu: ayuda_menu];

            NSApplication::setMainMenu_(NSApp(), main_menu);
        }
    }
}
