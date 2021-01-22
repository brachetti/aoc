extern crate derive_builder;
extern crate strum;
extern crate strum_macros;
extern crate thiserror;

pub mod prelude {
    pub use super::batch_file::*;
    pub use super::scanner::*;
    pub use derive_builder::*;
    pub use once_cell::sync::OnceCell;
    pub use recap::Recap;
    pub use regex::Regex;
    pub use serde::Deserialize;
    use thiserror::Error;
    pub use std::{
        collections::HashMap,
        error::Error,
        fmt::{self, Display, Formatter},
        str::FromStr,
        string::ParseError,
    };
}

pub mod batch_file;
pub mod scanner;
