use super::EventTypeField;
use crate::prelude::{Dialogue, EventType};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, not_line_ending},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

/// Parses Dialogues and Comments.
pub fn parse_events(input: &str) -> IResult<&str, Vec<Dialogue>> {
    map(
        many0(preceded(
            opt(multispace0),
            alt((parse_dialogue, parse_comment)),
        )),
        |d| {
            let mut evt: Vec<Dialogue> = Vec::new();
            for field in d {
                match field {
                    EventTypeField::Dialogue(line) => {
                        evt.push(parse_dialogue_line(line, EventType::Dialogue))
                    }
                    EventTypeField::Comment(line) => {
                        evt.push(parse_dialogue_line(line, EventType::Comment))
                    }
                }
            }
            evt
        },
    )(input)
}

fn parse_dialogue(input: &str) -> IResult<&str, EventTypeField> {
    map(
        delimited(tag("Dialogue: "), not_line_ending, line_ending),
        |t: &str| EventTypeField::Dialogue(t.to_string()),
    )(input)
}

fn parse_comment(input: &str) -> IResult<&str, EventTypeField> {
    map(
        delimited(tag("Comment: "), not_line_ending, line_ending),
        |t: &str| EventTypeField::Comment(t.to_string()),
    )(input)
}

fn parse_dialogue_line(d: String, type_: EventType) -> Dialogue {
    let (layer, start, end, style, name, margin_l, margin_r, margin_v, effect, text) =
        match d.split_once(',') {
            Some((layer, rest)) => {
                let layer = layer;
                let (start, rest) = rest.split_once(',').unwrap();
                let (end, rest) = rest.split_once(',').unwrap();
                let (style, rest) = rest.split_once(',').unwrap();
                let (name, rest) = rest.split_once(',').unwrap();
                let (margin_l, rest) = rest.split_once(',').unwrap();
                let (margin_r, rest) = rest.split_once(',').unwrap();
                let (margin_v, rest) = rest.split_once(',').unwrap();
                let (effect, text) = match rest.split_once(',') {
                    Some((effect, text)) => (Some(effect), text),
                    None => (None, rest),
                };
                (
                    layer.parse::<i64>().unwrap(),
                    start,
                    end,
                    style.to_owned(),
                    name.to_owned(),
                    margin_l.parse::<f64>().unwrap(),
                    margin_r.parse::<f64>().unwrap(),
                    margin_v.parse::<f64>().unwrap(),
                    effect.unwrap_or_default().to_owned(),
                    text.to_owned(),
                )
            }
            None => {
                let (layer, rest) = d.split_once(',').unwrap();
                let (start, rest) = rest.split_once(',').unwrap();
                let (end, rest) = rest.split_once(',').unwrap();
                let (style, rest) = rest.split_once(',').unwrap();
                let (name, text) = match rest.split_once(',') {
                    Some((name, text)) => (name, text),
                    None => ("", rest),
                };
                (
                    layer.parse::<i64>().unwrap(),
                    start,
                    end,
                    style.to_owned(),
                    name.to_owned(),
                    0.0,
                    0.0,
                    0.0,
                    "".to_owned(),
                    text.to_owned(),
                )
            }
        };
    Dialogue {
        type_,
        layer,
        start: start.to_string(),
        end: end.to_string(),
        style,
        name,
        margin_l,
        margin_r,
        margin_v,
        effect,
        text,
    }
}
