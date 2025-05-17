use crate::constants::{WINDOW_WIDTH, WINDOW_HEIGHT, SIDEBAR_WIDTH, RIGHT_VIEW_COLOR};
use cocoa::base::{nil, id};
use objc::{class, msg_send, sel, sel_impl};
use cocoa::appkit::NSView;
use cocoa::foundation::{NSRect, NSPoint, NSSize};
use objc::runtime::Object;
use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable};
use crate::views::player_table::{CreatePlayerTable, attach_data_source};
use crate::models::player::Player;
use crate::libs::database;

pub fn render_main_view_as_nsview(frame: NSRect) -> *mut Object {
    unsafe {
        println!("📦 Creando vista principal...");
        let view: *mut Object = NSView::alloc(nil).initWithFrame_(frame);
        let _: () = msg_send![view, setWantsLayer: true];
        let _: () = msg_send![view, setAutoresizingMask: NSViewHeightSizable | NSViewWidthSizable];

        // 1. Crear y añadir el panel de contenido a la derecha
        let content_width = WINDOW_WIDTH - SIDEBAR_WIDTH;
        let content_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(content_width, WINDOW_HEIGHT));
        let content_panel: id = NSView::alloc(nil).initWithFrame_(content_frame);
        let _: () = msg_send![content_panel, setAutoresizingMask: NSViewHeightSizable | NSViewWidthSizable];
        let _: () = msg_send![content_panel, setWantsLayer: true];
        let content_layer: id = msg_send![content_panel, layer];
        let color: id = msg_send![
            class!(NSColor),
            colorWithRed: RIGHT_VIEW_COLOR.0 green: RIGHT_VIEW_COLOR.1 blue: RIGHT_VIEW_COLOR.2 alpha: 1.0
        ];
        let cg_color: id = msg_send![color, CGColor];
        let _: () = msg_send![content_layer, setBackgroundColor: cg_color];

        let _: () = msg_send![view, addSubview: content_panel];

        // 3. Cargar la tabla de jugadores en el content_panel
        println!("🗄️ Abriendo base de datos...");
        let conn = database::get_connection().lock().unwrap();

        println!("📊 Cargando jugadores...");
        let players: Vec<Player> = Player::latest_versions(&conn).expect("❌ Error al cargar jugadores");

        println!("🧱 Creando tabla...");
        let table_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(content_width, WINDOW_HEIGHT));
        let (scroll_view, table_view) = CreatePlayerTable(table_frame);

        println!("🔗 Conectando data source...");
        attach_data_source(table_view, players);

        println!("📥 Añadiendo tabla al panel de contenido...");
        let _: () = msg_send![content_panel, addSubview: scroll_view];

        println!("✅ Vista principal creada correctamente.");
        view
    }
}
