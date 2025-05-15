use cocoa::base::{nil, id, YES};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSString};
use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable, NSViewMinYMargin};
use objc::runtime::{Class, Object, Sel};
use objc::*;
use objc::declare::ClassDecl;
use crate::models::player::Player;
use std::os::raw::c_void;
use crate::libs::objc_shims::*;

static mut PLAYER_DATA_SOURCE_CLASS: *const Class = std::ptr::null();
static mut STORED_PLAYER_VEC_PTR: Option<*mut Vec<Player>> = None;
static mut SELECTED_ROW_INDEX: isize = -1;

pub fn register_player_data_source_class() -> *const Class {
    unsafe {
        if !PLAYER_DATA_SOURCE_CLASS.is_null() {
            return PLAYER_DATA_SOURCE_CLASS;
        }

        let superclass = get_class("NSObject");
        let mut decl = ClassDecl::new("PlayerDataSource",  &*superclass).unwrap();

        decl.add_ivar::<*mut c_void>("players");

        decl.add_method(sel!(numberOfRowsInTableView:), number_of_rows as extern "C" fn(&Object, Sel, id) -> usize);
        decl.add_method(sel!(tableView:objectValueForTableColumn:row:), value_for_cell as extern "C" fn(&Object, Sel, id, id, usize) -> id);
        decl.add_method(sel!(tableViewSelectionDidChange:), on_row_selected as extern "C" fn(&Object, Sel, id));
        decl.add_method(sel!(tableView:heightOfRow:), row_height as extern "C" fn(&Object, Sel, id, isize) -> f64);

        PLAYER_DATA_SOURCE_CLASS = decl.register();
        PLAYER_DATA_SOURCE_CLASS
    }
}

extern "C" fn number_of_rows(this: &Object, _: Sel, _: id) -> usize {
    unsafe {
        let ptr = *this.get_ivar::<*mut c_void>("players") as *const Vec<Player>;
        if ptr.is_null() {
            return 0;
        }
        (*ptr).len()
    }
}

extern "C" fn value_for_cell(this: &Object, _: Sel, _: id, column: id, row: usize) -> id {
    use cocoa::foundation::NSString;
    unsafe {
        let ptr = *this.get_ivar::<*mut c_void>("players") as *const Vec<Player>;
        if ptr.is_null() {
            return NSString::alloc(nil).init_str("");
        }

        let players = &*ptr;
        if row >= players.len() {
            return NSString::alloc(nil).init_str("");
        }

        let player = &players[row];

        let id_str: id = msg_send![column, identifier];
        let cstr: *const std::os::raw::c_char = msg_send![id_str, UTF8String];
        let key = std::ffi::CStr::from_ptr(cstr).to_str().unwrap_or("");

        let value = match key {
            "col_nombre" => player.name.clone(),
            "col_edad" => format!("{}", player.age),
            "col_posicion" => player.best_position().to_string(),
            "col_tsi" => format!("{}", player.tsi),
            "col_habilidad" => format!("{}", player.playmaker),
            "col_evolucion" => "—".to_string(),
            _ => "".to_string(),
        };

        NSString::alloc(nil).init_str(&value)
    }
}

extern "C" fn on_row_selected(_this: &Object, _: Sel, notification: id) {
    unsafe {
        let table_view: id = msg_send![notification, object];
        let selected: isize = msg_send![table_view, selectedRow];
        SELECTED_ROW_INDEX = selected;
        let _: () = msg_send![table_view, noteHeightOfRowsWithIndexesChanged: nil];
    }
}

extern "C" fn row_height(_this: &Object, _: Sel, _table: id, row: isize) -> f64 {
    unsafe {
        if row == SELECTED_ROW_INDEX {
            80.0
        } else {
            24.0
        }
    }
}

pub unsafe fn attach_data_source(table_view: id, players: Vec<Player>) {
    let class = register_player_data_source_class();

    let data_source: id = msg_send![class, alloc];
    let data_source: id = msg_send![data_source, init];

    let boxed_vec = Box::new(players);
    let raw_ptr = Box::into_raw(boxed_vec);
    
    unsafe {
        STORED_PLAYER_VEC_PTR = Some(raw_ptr);
    }
    
    unsafe {
        (*data_source).set_ivar("players", raw_ptr as *mut c_void);
    }

    let _: () = msg_send![table_view, setDataSource: data_source];
    let _: () = msg_send![table_view, setDelegate: data_source];
    let _: () = msg_send![table_view, reloadData];
}

pub fn create_player_table(frame: NSRect) -> (id, id) {
    unsafe {
        // Márgenes
        let title_margin_top = 16.0;
        let title_margin_bottom = 12.0;
        let title_height = 30.0;
        let separator_height = 1.0;
        let total_header_height = title_margin_top + title_height + title_margin_bottom + separator_height;

        // Título superior
        let label_frame = NSRect::new(
            NSPoint::new(0.0, frame.size.height - title_height - title_margin_top),
            NSSize::new(frame.size.width, title_height)
        );
        let label: id = msg_send![class!(NSTextField), alloc];
        let label: id = msg_send![label, initWithFrame: label_frame];
        let _: () = msg_send![label, setStringValue: NSString::alloc(nil).init_str("🧍 Jugadores")];
        let _: () = msg_send![label, setBordered: false];
        let _: () = msg_send![label, setEditable: false];
        let font: id = msg_send![class!(NSFont), fontWithName:NSString::alloc(nil).init_str("Helvetica") size:22.0];
        let _: () = msg_send![label, setFont: font];
        let _: () = msg_send![label, setBackgroundColor: nil];
        let _: () = msg_send![label, setAutoresizingMask: NSViewMinYMargin | NSViewWidthSizable];

        // Separador
        let separator_frame = NSRect::new(
            NSPoint::new(0.0, frame.size.height - total_header_height),
            NSSize::new(frame.size.width, separator_height)
        );
        let separator: id = msg_send![class!(NSView), alloc];
        let separator: id = msg_send![separator, initWithFrame: separator_frame];
        let _: () = msg_send![separator, setWantsLayer: true];
        let layer: id = msg_send![separator, layer];
        let color: id = msg_send![class!(NSColor), colorWithWhite: 0.85 alpha: 1.0];
        let cg_color: id = msg_send![color, CGColor];
        let _: () = msg_send![layer, setBackgroundColor: cg_color];
        let _: () = msg_send![separator, setAutoresizingMask: NSViewMinYMargin | NSViewWidthSizable];

        // Tabla
        let scroll_frame = NSRect::new(
            NSPoint::new(0.0, 0.0),
            NSSize::new(frame.size.width, frame.size.height - total_header_height)
        );
        let table_view: id = msg_send![class!(NSTableView), alloc];
        let table_view: id = msg_send![table_view, initWithFrame: scroll_frame];
        let _: () = msg_send![table_view, setAutoresizingMask: NSViewHeightSizable | NSViewWidthSizable];

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

        let _: () = msg_send![table_view, setUsesAlternatingRowBackgroundColors: YES];
        let _: () = msg_send![table_view, setGridStyleMask: 0u64];
        let _: () = msg_send![table_view, setRowHeight: 24.0];
        let _: () = msg_send![table_view, setFocusRingType: 1]; // NSFocusRingTypeNone

        let scroll_view: id = msg_send![class!(NSScrollView), alloc];
        let scroll_view: id = msg_send![scroll_view, initWithFrame: scroll_frame];
        let _: () = msg_send![scroll_view, setDocumentView: table_view];
        let _: () = msg_send![scroll_view, setHasVerticalScroller: true];
        let _: () = msg_send![scroll_view, setAutoresizingMask: NSViewHeightSizable | NSViewWidthSizable];

        let container: id = msg_send![class!(NSView), alloc];
        let container: id = msg_send![container, initWithFrame: frame];
        let _: () = msg_send![container, setAutoresizingMask: NSViewHeightSizable | NSViewWidthSizable];
        let _: () = msg_send![container, addSubview: label];
        let _: () = msg_send![container, addSubview: separator];
        let _: () = msg_send![container, addSubview: scroll_view];

        (container, table_view)
    }
}
