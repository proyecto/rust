use cocoa::base::{id, nil, YES};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::runtime::{Class, Object};
use objc::*;
use crate::models::player::Player;
use rusqlite::Connection;

/// Crea una NSTableView nativa con scroll y columnas: Nombre, Edad, Posición, TSI, Habilidad, Evolución
/// Esta función devuelve un puntero a un NSScrollView que contiene la tabla y puede añadirse a un NSView existente
pub fn create_player_table(frame: NSRect) -> id {
    unsafe {
        // Paso 1: Crear NSTableView vacía
        let table_view: id = msg_send![class!(NSTableView), alloc];
        let table_view: id = msg_send![table_view, init];

        // Paso 2: Crear columnas con identificadores únicos
        let columns = vec![
            ("Nombre", "col_nombre"),
            ("Edad", "col_edad"),
            ("Posición", "col_posicion"),
            ("TSI", "col_tsi"),
            ("Habilidad", "col_habilidad"),
            ("Evolución", "col_evolucion"),
        ];

        for (title, identifier) in columns.iter() {
            let identifier_str = NSString::alloc(nil).init_str(identifier);
            let column: id = msg_send![class!(NSTableColumn), alloc];
            let column: id = msg_send![column, initWithIdentifier: identifier_str];

            let header_cell: id = msg_send![column, headerCell];
            let title_str = NSString::alloc(nil).init_str(title);
            let _: () = msg_send![header_cell, setStringValue: title_str];

            let _: () = msg_send![column, setWidth: 100.0];
            let _: () = msg_send![table_view, addTableColumn: column];
        }

        // Paso 3: Configurar propiedades visuales
        let _: () = msg_send![table_view, setUsesAlternatingRowBackgroundColors: YES];
        let _: () = msg_send![table_view, setGridStyleMask: 1u64]; // NSTableViewSolidHorizontalGridLineMask
        let _: () = msg_send![table_view, setRowHeight: 24.0];

        // Paso 4: Crear scroll view y colocar dentro la tabla
        let scroll_view: id = msg_send![class!(NSScrollView), alloc];
        let scroll_view: id = msg_send![scroll_view, initWithFrame: frame];
        let _: () = msg_send![scroll_view, setFrameOrigin: NSPoint::new(0.0, 0.0)];

        let _: () = msg_send![scroll_view, setHasVerticalScroller: YES];
        let _: () = msg_send![scroll_view, setHasHorizontalScroller: YES];
        let _: () = msg_send![scroll_view, setAutohidesScrollers: YES];
        let _: () = msg_send![scroll_view, setBorderType: 0u64];
        let _: () = msg_send![scroll_view, setAutoresizingMask: 18u64];

        // Ajustar el tamaño de la tabla
        let adjusted_frame = NSRect::new(NSPoint::new(0.0, 0.0), frame.size);
        let _: () = msg_send![table_view, setAutoresizingMask: 18u64];
        let _: () = msg_send![table_view, setFrame: adjusted_frame];

        let _: () = msg_send![scroll_view, setDocumentView: table_view];

        // Paso 5: Añadir datos
    

        scroll_view
    }
}
