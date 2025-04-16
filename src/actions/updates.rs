use std::error::Error;
use crate::traits::Action;
use crate::models::news::NewsItem;
use cocoa::base::id;
use objc::{class, msg_send, sel, sel_impl};
use crate::views::utils::clear_scroll_views;
use crate::actions::updates::clear_scroll_views::clear_scroll_views;
use cocoa::foundation::NSRect;
use cocoa::foundation::NSPoint;
use cocoa::foundation::NSSize;
use crate::views::news_table::create_news_table_view;


#[derive(Debug)]
pub struct Updates;

impl Action for Updates {
    fn render_view(&self, content_view: id) {
        unsafe {
            clear_scroll_views(content_view);
            let frame = NSRect::new(NSPoint::new(210.0, 10.0), NSSize::new(700.0, 600.0));
            let news_view = create_news_table_view(frame);
            let _: () = msg_send![content_view, addSubview: news_view];
        }
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut noticias = NewsItem::get_all(10)?;

        // Si no hay noticias, insertamos una de bienvenida
        if noticias.is_empty() {
            let bienvenida = NewsItem {
                id: 0,
                date: chrono::Local::now().format("%Y-%m-%d").to_string(),
                category: "sistema".to_string(),
                title: "Bienvenido".to_string(),
                description: "Has iniciado la aplicación por primera vez.".to_string(),
            };
            bienvenida.insert()?;
            noticias = NewsItem::get_all(10)?;
        }

        for noticia in noticias {
            println!(
                "[{}] {} - {}
{}
",
                noticia.date, noticia.category, noticia.title, noticia.description
            );
        }

        Ok(())
    }
}