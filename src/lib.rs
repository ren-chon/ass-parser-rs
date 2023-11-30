
/// # Usage
/// ```rust
/// use ass_parser::prelude::*;
/// use std::fs::read_to_string;
/// 
/// let f = read_to_string("my.ass").unwrap();
/// let f = parse_file(f.as_str()).unwrap();
///
/// println!("{:#?}", f);
/// ```
pub mod prelude;
mod parsers;
mod document;
