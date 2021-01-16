pub mod prelude {
    pub use super::scanner::*;
    pub use std::{
        str::FromStr,
        fmt::{self, Display, Formatter},
        error::Error,
        string::ParseError,
    };
    pub use serde::Deserialize;
    pub use regex::Regex;
    pub use once_cell::sync::OnceCell;
    pub use recap::Recap;
}

mod batch_file;
mod scanner;
