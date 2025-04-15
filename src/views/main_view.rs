use crate::constants::{WINDOW_WIDTH, WINDOW_HEIGHT, SIDEBAR_WIDTH, RIGHT_VIEW_COLOR};
use cocoa::base::{nil, id};
use objc::{class, msg_send, sel, sel_impl};
use crate::views::main_sideview as side_view;
use cocoa::appkit::NSView;
use cocoa::foundation::{NSRect, NSPoint, NSSize};
use objc::runtime::Object;
use cocoa::appkit::{NSViewHeightSizable, NSViewWidthSizable};
use crate::views::player_table::{create_player_table, attach_data_source};
use crate::models::player::Player;
use rusqlite::Connection;

pub fn render_main_view() {
    render_background((1.0, 1.0, 1.0));
    let side_view_x_position = WINDOW_WIDTH - SIDEBAR_WIDTH;
    side_view::render(side_view_x_position, 0.0, SIDEBAR_WIDTH, WINDOW_HEIGHT);
}

pub fn render_main_view_as_nsview(frame: NSRect) -> *mut Object {
    unsafe {
        println!("📦 Creando vista principal...");
        let view: *mut Object = NSView::alloc(nil).initWithFrame_(frame);

        println!("🗄️ Abriendo base de datos...");
        let conn = Connection::open("data/test.db").expect("❌ No se pudo abrir la base de datos");

        println!("📊 Cargando jugadores...");
        let players: Vec<Player> = Player::latest_versions(&conn).expect("❌ Error al cargar jugadores");

        println!("🧱 Creando tabla...");
        let table_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(frame.size.width, frame.size.height));
        let (container_view, table_view) = create_player_table(table_frame);

        println!("🔗 Conectando data source...");
        attach_data_source(table_view, players);

        println!("📥 Añadiendo tabla a la vista principal...");
        let _: () = msg_send![view, addSubview: container_view];

        let _: () = msg_send![view, setWantsLayer: true];
        let _: () = msg_send![view, setAutoresizingMask: NSViewHeightSizable | NSViewWidthSizable];

        let layer: *mut Object = msg_send![view, layer];
        let color: *mut Object = msg_send![
            class!(NSColor),
            colorWithRed: RIGHT_VIEW_COLOR.0 green: RIGHT_VIEW_COLOR.1 blue: RIGHT_VIEW_COLOR.2 alpha: 1.0
        ];
        let cg_color: *mut Object = msg_send![color, CGColor];
        let _: () = msg_send![layer, setBackgroundColor: cg_color];

        println!("✅ Vista principal creada correctamente.");
        view
    }
}

fn render_background(_color: (f64, f64, f64)) {
    // Si deseas implementar un fondo personalizado en el futuro
}
