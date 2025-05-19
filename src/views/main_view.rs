use crate::constants::{WINDOW_WIDTH, WINDOW_HEIGHT, SIDEBAR_WIDTH, RIGHT_VIEW_COLOR};
use cocoa::base::{nil, id};
use cocoa::appkit::NSView;
use cocoa::foundation::{NSRect, NSPoint, NSSize};
use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable};
use crate::views::player_table::{create_player_table, attach_data_source};
use crate::models::player::Player;
use crate::libs::database;
use crate::libs::objc_shims::*;
use objc::runtime::{Object, Sel};


pub fn render_main_view_as_nsview(frame: NSRect) -> *mut Object {
    unsafe {
        println!("📦 Creando vista principal...");
        let view: *mut Object = NSView::alloc(nil).initWithFrame_(frame);

        msg_send_void_bool(view, Sel::register("setWantsLayer:"), true);
        

        msg_send_void_usize(
            view,
            Sel::register("setAutoresizingMask:"),
            (NSViewHeightSizable | NSViewWidthSizable) as usize,
        );
        
        // 1. Crear y añadir el panel de contenido a la derecha
        let content_width = WINDOW_WIDTH - SIDEBAR_WIDTH;
        let content_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(content_width, WINDOW_HEIGHT));
        let content_panel: id = NSView::alloc(nil).initWithFrame_(content_frame);

        msg_send_void_usize(
            content_panel,
            Sel::register("setAutoresizingMask:"),
            (NSViewHeightSizable | NSViewWidthSizable) as usize,
        );
        

        msg_send_void_bool(content_panel, Sel::register("setWantsLayer:"), true);
                
        let content_layer: id = msg_send_id(content_panel, Sel::register("layer"));        
        let nscolor_class = get_class("NSColor") as *mut Object;
        let color: id = msg_send_id_f64_f64_f64_f64(
                nscolor_class,
                Sel::register("colorWithRed:green:blue:alpha:"),
                RIGHT_VIEW_COLOR.0,
                RIGHT_VIEW_COLOR.1,
                RIGHT_VIEW_COLOR.2,
                1.0,);

        let cg_color: id = msg_send_id(color, Sel::register("CGColor"));        
        msg_send_void_id(content_layer, Sel::register("setBackgroundColor:"), cg_color);
        msg_send_void_id(view, Sel::register("addSubview:"), content_panel);
        
        // 3. Cargar la tabla de jugadores en el content_panel
        println!("🗄️ Abriendo base de datos...");
        let conn = database::get_connection().lock().unwrap();

        println!("📊 Cargando jugadores...");
        let players: Vec<Player> = Player::latest_versions(&conn).expect("❌ Error al cargar jugadores");

        println!("🧱 Creando tabla...");
        let table_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(content_width, WINDOW_HEIGHT));
        let (scroll_view, table_view) = create_player_table(table_frame);

        println!("🔗 Conectando data source...");
        attach_data_source(table_view, players);

        println!("📥 Añadiendo tabla al panel de contenido...");
        msg_send_void_id(content_panel, Sel::register("addSubview:"), scroll_view);

        println!("✅ Vista principal creada correctamente.");
        view
    }
}
