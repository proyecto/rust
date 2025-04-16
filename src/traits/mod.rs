use std::error::Error;
use cocoa::base::id;


pub trait Action {
    fn run(&self) -> Result<(), Box<dyn Error>>;
    fn render_view(&self, content_view: id);
}