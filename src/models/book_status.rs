use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum BookStatus {
    Available,
    Maintenance,
    Loaned,
    Reserved,
}

impl fmt::Display for BookStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
