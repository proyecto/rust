use std::error::Error;
use chrono::Local;

use crate::traits::Action;
use crate::models::news::NewsItem;

#[derive(Debug)]
pub struct Updates;

impl Action for Updates {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("¡Hola desde Update!");
        Ok(())
    }
}
