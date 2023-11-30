use super::parse_string1;
use crate::prelude::{StyleEncoding, Styles};
use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    multi::many0,
    sequence::preceded,
    IResult,
};
/// Parses `Style:` lines
pub(crate) fn parse_v4_styles(input: &str) -> IResult<&str, Vec<Styles>> {
    map(many0(preceded(opt(multispace0), parse_style)), |fields| {
        let mut style: Vec<Styles> = Vec::new();
        for field in fields {
            match field {
                StyleField::Style(line) => style.push(parse_style_line(line)),
            };
        }
        style
    })(input)
}

fn parse_style(input: &str) -> IResult<&str, StyleField> {
    map(preceded(tag("Style: "), parse_string1), |lss: &str| {
        StyleField::Style(lss.to_string())
    })(input)
}

fn parse_style_line(style: String) -> Styles {
    match style.split_once(',') {
        Some((name, rest)) => {
            let name = name;
            let (font_name, rest) = rest.split_once(',').unwrap();
            let (font_size, rest) = rest.split_once(',').unwrap();
            let (primary_colour, rest) = rest.split_once(',').unwrap();
            let (secondary_colour, rest) = rest.split_once(',').unwrap();
            let (outline_colour, rest) = rest.split_once(',').unwrap();
            let (back_colour, rest) = rest.split_once(',').unwrap();
            let (bold, rest) = rest.split_once(',').unwrap();
            let (italic, rest) = rest.split_once(',').unwrap();
            let (underline, rest) = rest.split_once(',').unwrap();
            let (strikeout, rest) = rest.split_once(',').unwrap();
            let (scale_x, rest) = rest.split_once(',').unwrap();
            let (scale_y, rest) = rest.split_once(',').unwrap();
            let (spacing, rest) = rest.split_once(',').unwrap();
            let (angle, rest) = rest.split_once(',').unwrap();
            let (border_style, rest) = rest.split_once(',').unwrap();
            let (outline, rest) = rest.split_once(',').unwrap();
            let (shadow, rest) = rest.split_once(',').unwrap();
            let (alignment, rest) = rest.split_once(',').unwrap();
            let (margin_l, rest) = rest.split_once(',').unwrap();
            let (margin_r, rest) = rest.split_once(',').unwrap();
            let (margin_v, rest) = rest.split_once(',').unwrap();
            let encoding = rest.parse::<i32>().expect("false parse");
            let encoding = match encoding {
                1 => StyleEncoding::Ansi,
                2 => StyleEncoding::Default,
                77 => StyleEncoding::Mac,
                128 => StyleEncoding::ShiftJis,
                129 => StyleEncoding::Hangeul,
                130 => StyleEncoding::Johab,
                134 => StyleEncoding::GB2312,
                136 => StyleEncoding::ChineseBIG5,
                161 => StyleEncoding::Greek,
                162 => StyleEncoding::Turkish,
                163 => StyleEncoding::Vietnamese,
                177 => StyleEncoding::Hebrew,
                178 => StyleEncoding::Arabic,
                186 => StyleEncoding::Baltic,
                204 => StyleEncoding::Russian,
                222 => StyleEncoding::Thai,
                238 => StyleEncoding::EastEuropean,
                255 => StyleEncoding::Oem,
                _ => StyleEncoding::Default,
            };
            Styles {
                name: name.to_string(),
                font_name: font_name.to_string(),
                font_size: font_size.parse::<i32>().unwrap(),
                primary_colour: primary_colour.to_string(),
                secondary_colour: secondary_colour.to_string(),
                outline_colour: outline_colour.to_string(),
                back_colour: back_colour.to_string(),
                bold: bold.parse::<i32>().unwrap(),
                italic: italic.parse::<i32>().unwrap(),
                underline: underline.parse::<i32>().unwrap(),
                strikeout: strikeout.parse::<i32>().unwrap(),
                scale_x: scale_x.parse::<i32>().unwrap(),
                scale_y: scale_y.parse::<i32>().unwrap(),
                spacing: spacing.parse::<i32>().unwrap(),
                angle: angle.parse::<i32>().unwrap(),
                border_style: border_style.parse::<i32>().unwrap(),
                outline: outline.parse::<f32>().unwrap(),
                shadow: shadow.parse::<i32>().unwrap(),
                alignment: alignment.parse::<i32>().unwrap(),
                margin_l: margin_l.parse::<f32>().unwrap(),
                margin_r: margin_r.parse::<f32>().unwrap(),
                margin_v: margin_v.parse::<f32>().unwrap(),
                encoding,
            }
        }
        None => {
            eprintln!("no fields were given");
            Styles::default()
        }
    }
}

#[derive(Debug, PartialEq)]
enum StyleField {
    Style(String),
}
