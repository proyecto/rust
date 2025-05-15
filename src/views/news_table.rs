use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable, NSApp};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSRect, NSString, NSPoint, NSSize};
use objc::{class, msg_send, sel, sel_impl};
use crate::actions::Updates;
use crate::traits::Action;
use crate::views::clear_scroll_views::clear_scroll_views;

pub fn create_news_table_view(frame: NSRect) -> id {
    unsafe {
        let table_view: id = msg_send![class!(NSTableView), alloc];
        let table_view: id = msg_send![table_view, initWithFrame:frame];
        let _: () = msg_send![table_view, setColumnAutoresizingStyle:1];

        let columns = vec![
            ("Fecha", 120.0),
            ("Título", 200.0),
            ("Descripción", 400.0),
        ];

        for (title, width) in columns {
            let identifier = NSString::alloc(nil).init_str(title);
            let col: id = msg_send![class!(NSTableColumn), alloc];
            let col: id = msg_send![col, initWithIdentifier:identifier];
            let header: id = msg_send![col, headerCell];
            let title_str = NSString::alloc(nil).init_str(title);
            let _: () = msg_send![header, setStringValue:title_str];
            let _: () = msg_send![col, setWidth:width];
            let _: () = msg_send![table_view, addTableColumn:col];
        }

        let scroll_view: id = msg_send![class!(NSScrollView), alloc];
        let scroll_view: id = msg_send![scroll_view, initWithFrame:frame];
        let _: () = msg_send![scroll_view, setDocumentView:table_view];
        let _: () = msg_send![scroll_view, setHasVerticalScroller:true];
        let _: () = msg_send![scroll_view, setAutoresizingMask:
            NSViewHeightSizable | NSViewWidthSizable];

        scroll_view
    }
}


pub fn show_news_table_in_main_view() {
    unsafe {
        // Ejecutar lógica de actualización (solo datos)
        let _ = Updates.run();

        // Obtener la ventana y su contentView
        let app = NSApp();
        let window: id = msg_send![app, mainWindow];
        let content_view: id = msg_send![window, contentView];

        // Limpiar lo que haya en la parte derecha
        println!("🔍 Eliminando subviews...");
        clear_scroll_views(content_view);

        // Crear y añadir la nueva vista de noticias
        let frame = NSRect::new(NSPoint::new(210.0, 10.0), NSSize::new(700.0, 600.0));
        let news_table = create_news_table_view(frame);
        let _: () = msg_send![content_view, addSubview:news_table];
    }
}