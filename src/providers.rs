use std::fmt;
use serde::Deserialize;
#[derive(Debug, Deserialize)] 
pub enum Providers {
    Reddit
}

impl fmt::Display for Providers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}