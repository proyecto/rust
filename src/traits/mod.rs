use std::error::Error;

pub trait Action {
    fn run(&self) -> Result<(), Box<dyn Error>>;
    fn render_view(&self);
}