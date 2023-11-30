# ass_parser
__Disclaimer: This parser is currently in an experimental stage and is a work in progress.__

This Rust crate provides a parser for .ass *files* using the __nom__ crate. Developed primarily for learning purposes, there may be future updates. The parser currently supports reading .ass files structured as follows:

```yaml
[Script Info]
...fields here

[Aegisub Project Garbage]
...fields here

[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
...styles here

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
...events here
```
# Notes
* If certain fields are missing inside `[Script Info]`, the Default trait will be invoked to handle the missing fields. 
* The parser will panic if `[Aegisub Project Garbage]` section does not exist.

# Usage
```rust
use ass_parser::prelude::*;
use std::fs::read_to_string;

let file_content = read_to_string("my.ass").unwrap();
let parsed_file = parse_file(file_content.as_str()).unwrap();

println!("{:#?}", parsed_file);
``````
Feel free to use the parser for your ass files, and if you encounter any issues or have suggestions for improvement, don't hesitate to reach out. Though contributions are not currently accepted, your feedback is valuable for potential future enhancements.