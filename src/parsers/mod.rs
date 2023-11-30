mod parse_events;
mod parse_project_garbage;
mod parse_script_info;
mod parse_v4_styles;
use std::io::Error;

use crate::prelude::{
    Dialogue, ProjectGarbage, ScriptInfo, Styles, SubtitlesFile, WrapStyle, YcbcrMatrix,
};

use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::{digit0, line_ending},
    combinator::{map_res, opt},
    multi::many0,
    number::complete::float,
    sequence::{terminated, tuple},
    IResult,
};

// https://github.com/zkat/miette/discussions/282

pub fn parse_script_info_section(input: &str) -> IResult<&str, ScriptInfo> {
    let (input, _) = tuple((tag("\u{feff}[Script Info]"), line_ending))(input)?;
    let (input, si) = terminated(parse_script_info::script_info, many0(line_ending))(input)?;
    Ok((input, si))
}

pub fn parse_apg_section(input: &str) -> IResult<&str, ProjectGarbage> {
    let (input, _) = tuple((tag("[Aegisub Project Garbage]"), line_ending))(input)?;
    let (input, apg) = terminated(parse_project_garbage::parse_apg, many0(line_ending))(input)?;
    Ok((input, apg))
}

pub fn parse_styles_section(input: &str) -> IResult<&str, Vec<Styles>> {
    let (input, _) = tuple((tag("[V4+ Styles]"),line_ending,tag(
        "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding",
    ),line_ending))(input)?;
    let (input, vfs) = terminated(parse_v4_styles::parse_v4_styles, many0(line_ending))(input)?;
    Ok((input, vfs))
}

pub fn parse_events_section(input: &str) -> IResult<&str, Vec<Dialogue>> {
    let (input, _) = tuple((
        tag("[Events]"),
        line_ending,
        tag("Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text"),
        line_ending,
    ))(input)?;
    let (input, evt) = terminated(parse_events::parse_events, many0(line_ending))(input)?;
    Ok((input, evt))
}

/// Parses an ASS ***file***.
pub fn parse_file(input: &str) -> Result<SubtitlesFile, Error> {
    let (input, si) = parse_script_info_section(input).expect("parse_script_info_section() failed");

    // TODO: None if not found...
    let (input, apg) = opt(parse_apg_section)(input).expect("parse_apg_section() failed");

    let (input, vfs) = parse_styles_section(input).expect("parse_styles_section() failed");

    let (_, evt) = parse_events_section(input).expect("parse_events_section() failed");

    Ok(SubtitlesFile {
            script_info: si,
            project_garbage: apg,
            v4styles: vfs,
            events: evt,
        },
    )
}

pub(crate) fn parse_string1(input: &str) -> IResult<&str, &str> {
    take_until1("\r\n")(input)
}

/// Parses an integer.
pub(crate) fn integer(input: &str) -> IResult<&str, i32> {
    map_res(digit0, |s: &str| s.parse::<i32>())(input)
}
/// Parses a float.
pub(crate) fn floating(input: &str) -> IResult<&str, f32> {
    map_res(float, |s| s.to_string().parse::<f32>())(input)
}

#[derive(Debug, PartialEq)]
pub enum EventTypeField {
    Dialogue(String),
    Comment(String),
}

#[derive(Debug, PartialEq)]
pub enum ScriptInfoField {
    Title(String),
    WrapStyle(WrapStyle),
    ScaledBorderAndShadow(bool),
    YcbcrMatrix(Option<YcbcrMatrix>),
    OriginalScript(String),
    PlayResX(i32),
    PlayResY(i32),
    OriginalTranslation(String),
    OriginalEditing(String),
    OriginalTiming(String),
    SynchPoint(String),
    ScriptUpdatedBy(String),
    UpdateDetails(String),
    ScriptType(String),
}

#[derive(Debug, PartialEq)]
pub enum ProjectGarbageField {
    LastStyleStorage(String),
    VideoFile(String),
    AudioFile(String),
    VideoArMode(f32),
    VideoArValue(f32),
    VideoZoomPercent(f32),
    ScrollPosition(i32),
    ActiveLine(i32),
    VideoPosition(i32),
}

#[cfg(test)]
mod tests {
    use super::parse_script_info_section;
    use crate::prelude::{ScriptInfo, WrapStyle};
    #[test]
    fn test_script_info_parser() {
        let pretend_this_is_a_file = "\u{feff}[Script Info]\r\nTitle: Translation File Test Doc\r\nScriptType: v4.00+\r\nWrapStyle: 0\r\nScaledBorderAndShadow: yes\r\nYCbCr Matrix: None\r\nOriginal Script: OGS\r\nPlayResX: 1920\r\nPlayResY: 1080\r\nOriginal Translation: TL By John Doe\r\nOriginal Editing: ED By John Doe\r\nOriginal Timing: TIMING By John Doe\r\nSynch Point: SYNCING By John Doe\r\nScript Updated By: UPDATED BY By John Doe\r\nUpdate Details: UPDATED DETAILS By John Doe\r\n";
        let expected_output = ScriptInfo {
            title: "Translation File Test Doc".to_owned(),
            comments: vec![],
            script_type: "v4.00+".to_owned(),
            wrap_style: WrapStyle::WrapStyle0,
            scaled_border_and_shadow: true,
            ycbcr_matrix: None,
            original_script: "OGS".to_owned(),
            play_res_x: 1920,
            play_res_y: 1080,
            original_translation: "TL By John Doe".to_owned(),
            original_editing: "ED By John Doe".to_owned(),
            original_timing: "TIMING By John Doe".to_owned(),
            synch_point: "SYNCING By John Doe".to_owned(),
            script_updated_by: "UPDATED BY By John Doe".to_owned(),
            update_details: "UPDATED DETAILS By John Doe".to_owned(),
        };
        assert_eq!(parse_script_info_section(pretend_this_is_a_file), Ok(("", expected_output)));
    }
}
