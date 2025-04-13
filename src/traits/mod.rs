use std::fmt::Debug;

// src/traits/mod.rs
pub trait Action {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}