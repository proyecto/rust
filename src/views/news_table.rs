use cocoa::appkit::{NSView, NSViewHeightSizable, NSViewWidthSizable};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSRect, NSString};
use objc::{class, msg_send, sel, sel_impl};

pub fn create_news_table_view(frame: NSRect) -> id {
    unsafe {
        // Crear el NSTableView
        let table_view: id = msg_send![class!(NSTableView), alloc];
        let table_view: id = msg_send![table_view, initWithFrame:frame];
        let _: () = msg_send![table_view, setHeaderView:nil];
        let _: () = msg_send![table_view, setColumnAutoresizingStyle: 1];

        // Definir columnas
        let columns = vec![
            ("Fecha", 120.0),
            ("Título", 200.0),
            ("Descripción", 400.0),
        ];

        for (title, width) in columns {
            let identifier = NSString::alloc(nil).init_str(title);
            let col: id = msg_send![class!(NSTableColumn), alloc];
            let col: id = msg_send![col, initWithIdentifier:identifier];
            let header_cell: id = msg_send![col, headerCell];
            let title_str = NSString::alloc(nil).init_str(title);
            let _: () = msg_send![header_cell, setStringValue:title_str];
            let _: () = msg_send![col, setWidth:width];
            let _: () = msg_send![table_view, addTableColumn:col];
        }

        // Crear NSScrollView que contenga la tabla
        let scroll_view: id = msg_send![class!(NSScrollView), alloc];
        let scroll_view: id = msg_send![scroll_view, initWithFrame:frame];
        let _: () = msg_send![scroll_view, setDocumentView:table_view];
        let _: () = msg_send![scroll_view, setHasVerticalScroller:true];
        let _: () = msg_send![scroll_view, setAutoresizingMask:
            NSViewHeightSizable | NSViewWidthSizable];

        scroll_view
    }
}
