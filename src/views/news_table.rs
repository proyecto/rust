use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable, NSApp};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSRect, NSString, NSPoint, NSSize};
use objc::runtime::{Sel};
use crate::actions::Updates;
use crate::traits::Action;
use crate::views::clear_scroll_views::clear_scroll_views;
use objc::runtime::Object;
use crate::libs::objc_shims::*;

pub fn create_news_table_view(frame: NSRect) -> id {
    unsafe {

        let cls = get_class("NSTableView");

        let table_view: id = msg_send_id(cls as *mut _, Sel::register("alloc"));
        
        msg_send_id(cls as *mut _, Sel::register("alloc"));

        let table_view: id = msg_send_id_rect(table_view as *mut _, Sel::register("initWithFrame:"), frame);

        msg_send_void_usize(table_view as *mut _, Sel::register("setColumnAutoresizingStyle:"), 1);

        let columns = vec![
            ("Fecha", 120.0),
            ("Título", 200.0),
            ("Descripción", 400.0),
        ];

        for (title, width) in columns {
            let identifier = NSString::alloc(nil).init_str(title);
            let cls = get_class("NSTableColumn");
            let col: id = msg_send_id(cls as *mut _, Sel::register("alloc")) ;            
            let col: id =  msg_send_id_id(col as *mut _, Sel::register("initWithIdentifier:"), identifier);
            let header: id = msg_send_id(col as *mut _, Sel::register("headerCell"));            
            let title_str = NSString::alloc(nil).init_str(title);
            msg_send_void_id(header as *mut _, Sel::register("setStringValue:"), title_str);
            msg_send_void_f64(col as *mut _, Sel::register("setWidth:"), width);
            msg_send_void_id(table_view as *mut _, Sel::register("addTableColumn:"), col);   
        }   

        let class = get_class("NSScrollView");
        let scroll_view: id = msg_send_id(class as *mut _, Sel::register("alloc"));
        
        let scroll_view: id = msg_send_id_rect(scroll_view as *mut _, Sel::register("initWithFrame:"), frame) ;

        msg_send_void_id(scroll_view as *mut _, Sel::register("setDocumentView:"), table_view);

        msg_send_void_bool(scroll_view as *mut _, Sel::register("setHasVerticalScroller:"), true);
      
        let mask = NSViewHeightSizable | NSViewWidthSizable;
        msg_send_void_usize(scroll_view as *mut _, Sel::register("setAutoresizingMask:"), mask as usize);

        scroll_view
    }
}


pub fn show_news_table_in_main_view() {
    unsafe {
        // Ejecutar lógica de actualización (solo datos)
        let _ = Updates.run();

        // Obtener la ventana y su contentView
        let app = NSApp();
        let window: id = msg_send_id(app as *mut _, Sel::register("mainWindow")) ;

        let content_view: id = msg_send_id(window as *mut Object, Sel::register("contentView")) ;

        // Limpiar lo que haya en la parte derecha
        println!("🔍 Eliminando subviews...");
        clear_scroll_views(content_view);

        // Crear y añadir la nueva vista de noticias
        let frame = NSRect::new(NSPoint::new(210.0, 10.0), NSSize::new(700.0, 600.0));
        let news_table = create_news_table_view(frame);

        msg_send_void_id(content_view, Sel::register("addSubview:"), news_table);
    }
}