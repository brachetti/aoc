extern crate derive_builder;
extern crate strum;
extern crate strum_macros;

pub mod prelude {
    pub use super::scanner::*;
    pub use derive_builder::*;
    pub use once_cell::sync::OnceCell;
    pub use recap::Recap;
    pub use regex::Regex;
    pub use serde::Deserialize;
    pub use std::{
        error::Error,
        fmt::{self, Display, Formatter},
        str::FromStr,
        string::ParseError,
    };
}

mod batch_file;
mod scanner;
