use std::fs::File;
use std::io::Write;
use reqwest::blocking::get;
use crate::traits::Action;
use std::error::Error;


#[derive(Debug)]
pub struct ListPlayers;

impl Action for ListPlayers {
    fn run(&self) -> Result<(), Box<dyn Error>> 
    {
        let respuesta = get("https://custm.es/players.xml")?;
        let contenido = respuesta.bytes()?;
        let mut archivo = File::create("data/players.xml")?;
        archivo.write_all(&contenido)?;
        println!("Archivo descargado");
        Ok(())
    }
}


