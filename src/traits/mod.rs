use std::fmt::Debug;

pub trait Action: Debug + Send + Sync {
    fn run(&self);
}