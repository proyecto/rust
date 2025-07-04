use cocoa::base::{nil, id};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSString};
use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable, NSViewMinYMargin};
use objc::runtime::{Class, Object, Sel};
use objc::declare::ClassDecl;
use crate::models::player::Player;
use std::os::raw::c_void;
use crate::libs::objc_shims::*;
use std::ffi::CStr;

static mut PLAYER_DATA_SOURCE_CLASS: *const Class = std::ptr::null();
static mut STORED_PLAYER_VEC_PTR: Option<*mut Vec<Player>> = None;
static mut SELECTED_ROW_INDEX: isize = -1;

pub fn register_player_data_source_class() -> *const Class {
    unsafe {
        if !PLAYER_DATA_SOURCE_CLASS.is_null() {
            return PLAYER_DATA_SOURCE_CLASS;
        }

        let superclass = get_class("NSObject");
        let mut decl = ClassDecl::new("PlayerDataSource", &*superclass).expect("Failed to create class declaration");

        decl.add_ivar::<*mut c_void>("players");

        decl.add_method(
            Sel::register("numberOfRowsInTableView:"),
            number_of_rows as extern "C" fn(&Object, Sel, id) -> usize,
        );

        decl.add_method(
            Sel::register("tableView:objectValueForTableColumn:row:"),
            value_for_cell as extern "C" fn(&Object, Sel, id, id, usize) -> id,
        );

        decl.add_method(
            Sel::register("tableViewSelectionDidChange:"),
            on_row_selected as extern "C" fn(&Object, Sel, id),
        );

        decl.add_method(
            Sel::register("tableView:heightOfRow:"),
            row_height as extern "C" fn(&Object, Sel, id, isize) -> f64,
        );

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

        let id_str: id = crate::libs::objc_shims::msg_send_id(column as *mut _, Sel::register("identifier"));        let cstr: *const std::os::raw::c_char = crate::libs::objc_shims::msg_send_c_char(id_str as *mut _, Sel::register("UTF8String"));        let key = std::ffi::CStr::from_ptr(cstr).to_str().unwrap_or("");

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
        let table_view: id = crate::libs::objc_shims::msg_send_id(notification as *mut _, Sel::register("object")); 
        let selected: isize = crate::libs::objc_shims::msg_send_usize(table_view as *mut _, Sel::register("selectedRow")) as isize;
        SELECTED_ROW_INDEX = selected;
        crate::libs::objc_shims::msg_send_void_id(table_view as *mut _, Sel::register("noteHeightOfRowsWithIndexesChanged:"), nil);
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

    let data_source: id = unsafe {
        crate::libs::objc_shims::msg_send_id(class as *mut _, Sel::register("alloc"))
    };    
    let data_source: id = unsafe {
        crate::libs::objc_shims::msg_send_id(data_source as *mut _, Sel::register("init"))
    };

    let boxed_vec = Box::new(players);
    let raw_ptr = Box::into_raw(boxed_vec);
    
    unsafe {
        STORED_PLAYER_VEC_PTR = Some(raw_ptr);
    }
    
    unsafe {
        (*data_source).set_ivar("players", raw_ptr as *mut c_void);
        crate::libs::objc_shims::msg_send_void_id(table_view as *mut _, Sel::register("setDataSource:"), data_source);
        crate::libs::objc_shims::msg_send_void_id(table_view as *mut _, Sel::register("setDelegate:"), data_source);
        crate::libs::objc_shims::msg_send_void(table_view as *mut _, Sel::register("reloadData"));
    }
}

unsafe fn create_title_table(frame: NSRect) -> id { unsafe {
    println!("📌 Buscando clase NSTextField...");
    let cls = get_class("NSTextField");

    if let Some(_cls) = Class::get("NSTextField") {
        println!("✅ NSTextField está registrada");
    } else {
        println!("❌ NSTextField NO está registrada");
    }

    if cls.is_null() {
        println!("❌ Clase NSTextField no encontrada");
        std::process::exit(1);
    }

    let instance: id = {
        let alloc: id = crate::libs::objc_shims::msg_send_id(cls as *mut _, Sel::register("alloc"));
        let obj: id = crate::libs::objc_shims::msg_send_id(alloc as *mut _, Sel::register("init"));
        obj
    };

    // ⚠️ get_class devuelve *const Class, pero necesitamos *mut Object para msg_send
    let name_ptr = crate::libs::objc_shims::msg_send_c_char(instance as *mut _, Sel::register("className"));


    let class_name = CStr::from_ptr(name_ptr).to_string_lossy();
    println!("📛 Instancia creada, clase: {}", class_name);

    if class_name == "NSTextField" {
        println!("✅ cls representa a NSTextField");
    } else {
        println!("⚠️ cls NO representa a NSTextField, es '{}'", class_name);
    }

    println!("✅ Clase encontrada, haciendo alloc...");
    let alloc = msg_send_id(cls as *mut _, Sel::register("alloc"));
    if alloc.is_null() {
        println!("❌ alloc fallido");
        std::process::exit(1);
    }

    println!("✅ alloc ok, haciendo initWithFrame...");
    let label = msg_send_id_rect(alloc as *mut _, Sel::register("initWithFrame:"), frame);
    if label.is_null() {
        println!("❌ initWithFrame fallido");
        std::process::exit(1);
    }

    println!("✅ Label creado correctamente");
    let title = NSString::alloc(nil).init_str("🧍 Jugadores");
    if title.is_null() {
        println!("❌ title fallido");
        std::process::exit(1);
    }

    println!("✅ title creado correctamente");
    msg_send_void_id(label as *mut _, Sel::register("setStringValue:"), title);
    msg_send_void_bool(label as *mut _, Sel::register("setBordered:"), false);
    msg_send_void_bool(label as *mut _, Sel::register("setEditable:"), false);

    let font_class = get_class("NSFont");
    let font_name = NSString::alloc(nil).init_str("Helvetica");
    let font = msg_send_id_id_f64(font_class as *mut _, Sel::register("fontWithName:size:"), font_name, 22.0);
    msg_send_void_id(label as *mut _, Sel::register("setFont:"), font);

    msg_send_void_id(label as *mut _, Sel::register("setBackgroundColor:"), nil);
    msg_send_void_u64(label as *mut _, Sel::register("setAutoresizingMask:"), NSViewMinYMargin | NSViewWidthSizable);

    label
}}

unsafe fn create_separator(frame: NSRect) -> id 
{ unsafe {
    // Crear NSView con initWithFrame
    let cls = get_class("NSView");
    let alloc = msg_send_id(cls as *mut _, Sel::register("alloc"));
    let separator = msg_send_id_rect(alloc as *mut _, Sel::register("initWithFrame:"), frame);

    // Activar capa
    msg_send_void_bool(separator as *mut _, Sel::register("setWantsLayer:"), true);

    // Obtener capa
    let layer = msg_send_id(separator as *mut _, Sel::register("layer"));

    // Crear color NSColor → CGColor
    let color_class = get_class("NSColor");
    let ns_color =  msg_send_id_f64_f64(color_class as *mut _, Sel::register("colorWithWhite:alpha:"), 0.85, 1.0);
    let cg_color = msg_send_id(ns_color as *mut _, Sel::register("CGColor"));

    // Asignar color a la capa
    msg_send_void_id(layer as *mut _, Sel::register("setBackgroundColor:"), cg_color);

    // Autoresizing mask
    msg_send_void_u64(
        separator as *mut _,
        Sel::register("setAutoresizingMask:"),
        NSViewMinYMargin | NSViewWidthSizable,
    );

    separator
}}

pub unsafe fn create_table(scroll_frame: NSRect) -> id 
{
    unsafe
    {
        let cls = get_class("NSTableView");
        let alloc = msg_send_id(cls as *mut Object, Sel::register("alloc"));
        let table_view = msg_send_id_rect(alloc as *mut Object, Sel::register("initWithFrame:"), scroll_frame);

         // Autoresizing
        msg_send_void_u64(
            table_view as *mut Object,
            Sel::register("setAutoresizingMask:"),
            NSViewHeightSizable | NSViewWidthSizable,
        );
    
        // Columnas
        let columns = vec![
            ("Nombre", "col_nombre"),
            ("Edad", "col_edad"),
            ("Posición", "col_posicion"),
            ("TSI", "col_tsi"),
            ("Habilidad", "col_habilidad"),
            ("Evolución", "col_evolucion"),
        ];

        let column_cls = get_class("NSTableColumn");

        for (title, identifier) in columns 
        {
            let identifier_str = NSString::alloc(nil).init_str(identifier);
            let alloc = msg_send_id(column_cls as *mut Object, Sel::register("alloc"));
            let column = msg_send_id_id(alloc as *mut Object, Sel::register("initWithIdentifier:"), identifier_str);
            let header_cell = msg_send_id(column as *mut Object, Sel::register("headerCell"));
            let title_str = NSString::alloc(nil).init_str(title);
            msg_send_void_id(header_cell as *mut Object, Sel::register("setStringValue:"), title_str);
            msg_send_void_f64(column as *mut Object, Sel::register("setWidth:"), 100.0);
            msg_send_void_id(table_view as *mut Object, Sel::register("addTableColumn:"), column);   
        }
        // Apariencia
        msg_send_void_bool(table_view as *mut Object, Sel::register("setUsesAlternatingRowBackgroundColors:"), true);
        msg_send_void_u64(table_view as *mut Object, Sel::register("setGridStyleMask:"), 0);
        msg_send_void_f64(table_view as *mut Object, Sel::register("setRowHeight:"), 24.0);
        msg_send_void_u64(table_view as *mut Object, Sel::register("setFocusRingType:"), 1); // NSFocusRingTypeNone
    
        table_view
    }
}

pub unsafe fn create_scroll_view(frame: NSRect, document_view: id) -> id 
{
    unsafe
    {
        let cls = get_class("NSScrollView");
        let alloc = msg_send_id(cls as *mut Object, Sel::register("alloc"));
        let scroll_view = msg_send_id_rect(alloc as *mut Object, Sel::register("initWithFrame:"), frame);

        msg_send_void_id(scroll_view as *mut Object, Sel::register("setDocumentView:"), document_view);
        msg_send_void_bool(scroll_view as *mut Object, Sel::register("setHasVerticalScroller:"), true);
        msg_send_void_u64(
            scroll_view as *mut Object,
            Sel::register("setAutoresizingMask:"),
            NSViewHeightSizable | NSViewWidthSizable,
        );
        scroll_view
    }
}

unsafe fn create_container(frame: NSRect, title: id, separator: id, scroll_view: id) -> id {
    let cls = get_class("NSView");
    let alloc = unsafe { msg_send_id(cls as *mut _, Sel::register("alloc")) };
    let container = unsafe { msg_send_id_rect(alloc as *mut _, Sel::register("initWithFrame:"), frame) };

    unsafe
    {
        msg_send_void_u64(container as *mut _, Sel::register("setAutoresizingMask:"), NSViewHeightSizable | NSViewWidthSizable);
        msg_send_void_id(container as *mut _, Sel::register("addSubview:"), title);
        msg_send_void_id(container as *mut _, Sel::register("addSubview:"), separator);
        msg_send_void_id(container as *mut _, Sel::register("addSubview:"), scroll_view);
    }
    container
}

pub fn create_player_table(frame: NSRect) -> (id, id) {
    unsafe 
    {
        // Márgenes y alturas
        let title_margin_top = 16.0;
        let title_margin_bottom = 12.0;
        let title_height = 30.0;
        let separator_height = 1.0;
        let total_header_height = title_margin_top + title_height + title_margin_bottom + separator_height;

        // Título (🧍 Jugadores)
        let label_frame = NSRect::new(
            NSPoint::new(0.0, frame.size.height - title_height - title_margin_top),
            NSSize::new(frame.size.width, title_height)
        );
        println!("🔧 Creando etiqueta...");
        let title = create_title_table(label_frame);

        // Separador horizontal gris
        println!("🔧 Creando separador...");
        let separator_frame = NSRect::new(
            NSPoint::new(0.0, frame.size.height - total_header_height),
            NSSize::new(frame.size.width, separator_height)
        );
        let separator = create_separator(separator_frame);

        // Tabla
        println!("🔧 Creando tabla...");
        let table_frame = NSRect::new(
            NSPoint::new(0.0, 0.0),
            NSSize::new(frame.size.width, frame.size.height - total_header_height)
        );
        let table_view = create_table(table_frame);

        // Scroll View
        println!("🔧 Creando scroll view...");
        let scroll_view = create_scroll_view(table_frame, table_view);

        // Contenedor final
        println!("🔧 Creando contenedor...");
        let container = create_container(frame, title, separator, scroll_view);

        println!("✅ Tabla construida correctamente.");
        (container, table_view)
    }
}