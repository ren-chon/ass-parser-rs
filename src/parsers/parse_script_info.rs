use super::{
    ScriptInfoField, {integer, parse_string1},
};
use crate::prelude::{ScriptInfo, WrapStyle, YcbcrMatrix};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    multi::many0,
    sequence::preceded,
    IResult,
};

/// Parses `[Script info]` fields.
/// Defaults missing fields.
pub(crate) fn script_info(input: &str) -> IResult<&str, ScriptInfo> {
    map(
        many0(preceded(
            opt(multispace0),
            alt((
                title,
                original_script,
                wrap_style,
                scaled_border_and_shadow,
                ycbcr_matrix,
                play_res_x,
                play_res_y,
                original_translation,
                original_editing,
                original_timing,
                synch_point,
                script_updated_by,
                update_details,
                parse_script_type,
            )),
        )),
        |fields| {
            let mut script_info = ScriptInfo::default();
            for field in fields {
                match field {
                    ScriptInfoField::Title(title) => script_info.title = title,
                    ScriptInfoField::OriginalScript(ogs) => script_info.original_script = ogs,
                    ScriptInfoField::WrapStyle(wrap_style) => script_info.wrap_style = wrap_style,
                    ScriptInfoField::ScaledBorderAndShadow(scaled_border_and_shadow) => {
                        script_info.scaled_border_and_shadow = scaled_border_and_shadow
                    }
                    ScriptInfoField::YcbcrMatrix(ycbcr_matrix) => {
                        script_info.ycbcr_matrix = ycbcr_matrix
                    }

                    ScriptInfoField::PlayResX(play_res_x) => script_info.play_res_x = play_res_x,
                    ScriptInfoField::PlayResY(play_res_y) => script_info.play_res_y = play_res_y,
                    ScriptInfoField::OriginalTranslation(original_translation) => {
                        script_info.original_translation = original_translation
                    }
                    ScriptInfoField::OriginalEditing(original_editing) => {
                        script_info.original_editing = original_editing
                    }
                    ScriptInfoField::OriginalTiming(original_timing) => {
                        script_info.original_timing = original_timing
                    }
                    ScriptInfoField::SynchPoint(synch_point) => {
                        script_info.synch_point = synch_point
                    }
                    ScriptInfoField::ScriptUpdatedBy(script_updated_by) => {
                        script_info.script_updated_by = script_updated_by
                    }
                    ScriptInfoField::UpdateDetails(update_details) => {
                        script_info.update_details = update_details
                    }
                    ScriptInfoField::ScriptType(st) => script_info.script_type = st,
                }
            }
            script_info
        },
    )(input)
}

/// Parses "Title" field in the "Script Info" section.
pub(crate) fn title(input: &str) -> IResult<&str, ScriptInfoField> {
    map(preceded(tag("Title: "), parse_string1), |t: &str| {
        ScriptInfoField::Title(t.to_string())
    })(input)
}
/// Parses "Original Script" field in the "Script Info" section.
pub(crate) fn original_script(input: &str) -> IResult<&str, ScriptInfoField> {
    map(
        preceded(tag("Original Script: "), parse_string1),
        |t: &str| ScriptInfoField::OriginalScript(t.to_string()),
    )(input)
}
/// Parses "Script Type" field in the "Script Info" section.
pub(crate) fn parse_script_type(input: &str) -> IResult<&str, ScriptInfoField> {
    map(preceded(tag("ScriptType: "), parse_string1), |st: &str| {
        ScriptInfoField::ScriptType(st.to_string())
    })(input)
}
/// Parses "Original Translation" field in the "Script Info" section.
pub(crate) fn original_translation(input: &str) -> IResult<&str, ScriptInfoField> {
    map(
        preceded(tag("Original Translation: "), parse_string1),
        |ot: &str| ScriptInfoField::OriginalTranslation(ot.to_string()),
    )(input)
}

/// Parses "Original Editing" field in the "Script Info" section.
pub(crate) fn original_editing(input: &str) -> IResult<&str, ScriptInfoField> {
    map(preceded(tag("Original Editing: "), parse_string1), |oe| {
        ScriptInfoField::OriginalEditing(oe.to_string())
    })(input)
}

/// Parses "Original Timing" field in the "Script Info" section.
pub(crate) fn original_timing(input: &str) -> IResult<&str, ScriptInfoField> {
    map(preceded(tag("Original Timing: "), parse_string1), |ot| {
        ScriptInfoField::OriginalTiming(ot.to_string())
    })(input)
}

/// Parses "Synch-Point" field in the "Script Info" section.
pub(crate) fn synch_point(input: &str) -> IResult<&str, ScriptInfoField> {
    map(
        preceded(tag("Synch Point: "), parse_string1),
        |sync_point: &str| ScriptInfoField::SynchPoint(sync_point.to_string()),
    )(input)
}

/// Parses "Script Updated By" field in the "Script Info" section.
pub(crate) fn script_updated_by(input: &str) -> IResult<&str, ScriptInfoField> {
    map(preceded(tag("Script Updated By: "), parse_string1), |sub| {
        ScriptInfoField::ScriptUpdatedBy(sub.to_string())
    })(input)
}

/// Parses "Update Details" field in the "Script Info" section.
pub(crate) fn update_details(input: &str) -> IResult<&str, ScriptInfoField> {
    map(preceded(tag("Update Details: "), parse_string1), |ud| {
        ScriptInfoField::UpdateDetails(ud.to_string())
    })(input)
}

/// Parses "PlayResX" field in the "Script Info" section.
pub(crate) fn play_res_x(input: &str) -> IResult<&str, ScriptInfoField> {
    map(
        preceded(tag("PlayResX: "), integer),
        ScriptInfoField::PlayResX,
    )(input)
}

/// Parses "PlayResY" field in the "Script Info" section.
pub(crate) fn play_res_y(input: &str) -> IResult<&str, ScriptInfoField> {
    map(
        preceded(tag("PlayResY: "), integer),
        ScriptInfoField::PlayResY,
    )(input)
}

/// Parses "WrapStyle" field in the "Script Info" section and returns `WrapStyle` enum.
pub(crate) fn wrap_style(input: &str) -> IResult<&str, ScriptInfoField> {
    map(preceded(tag("WrapStyle: "), integer), |ws| match ws {
        0 => ScriptInfoField::WrapStyle(WrapStyle::WrapStyle0),
        1 => ScriptInfoField::WrapStyle(WrapStyle::WrapStyle1),
        2 => ScriptInfoField::WrapStyle(WrapStyle::WrapStyle2),
        3 => ScriptInfoField::WrapStyle(WrapStyle::WrapStyle3),
        i32::MIN..=-1_i32 | 4_i32..=i32::MAX => ScriptInfoField::WrapStyle(WrapStyle::WrapStyle0),
    })(input)
}

/// Parses "ScaledBorderAndShadow" field in the "Script Info" section.
pub(crate) fn scaled_border_and_shadow(input: &str) -> IResult<&str, ScriptInfoField> {
    map(
        preceded(tag("ScaledBorderAndShadow: "), parse_string1),
        |sbas| match sbas {
            "yes" => ScriptInfoField::ScaledBorderAndShadow(true),
            "no" => ScriptInfoField::ScaledBorderAndShadow(false),
            &_ => ScriptInfoField::ScaledBorderAndShadow(true),
        },
    )(input)
}

/// Parses "YCbCr Matrix" field in the "Script Info" section and returns `YcbcrMatrix` enum.
pub(crate) fn ycbcr_matrix(input: &str) -> IResult<&str, ScriptInfoField> {
    map(
        preceded(tag("YCbCr Matrix: "), parse_string1),
        |ycbcr| match ycbcr {
            "None" => ScriptInfoField::YcbcrMatrix(None),
            "TV.601" => ScriptInfoField::YcbcrMatrix(Some(YcbcrMatrix::Tv601)),
            "PC.601" => ScriptInfoField::YcbcrMatrix(Some(YcbcrMatrix::Pc601)),
            "TV.709" => ScriptInfoField::YcbcrMatrix(Some(YcbcrMatrix::Tv709)),
            "PC.709" => ScriptInfoField::YcbcrMatrix(Some(YcbcrMatrix::Pc709)),
            "TV.FCC" => ScriptInfoField::YcbcrMatrix(Some(YcbcrMatrix::Tvfcc)),
            "PC.FCC" => ScriptInfoField::YcbcrMatrix(Some(YcbcrMatrix::Pcfcc)),
            "TV.240M" => ScriptInfoField::YcbcrMatrix(Some(YcbcrMatrix::Tv240m)),
            "PC.240M" => ScriptInfoField::YcbcrMatrix(Some(YcbcrMatrix::Pc240m)),
            unknown => {
                eprintln!("This YCbCr ({}) is unknown or not implemented, Please file an issue.\nReturning `None` as the default.", unknown);
                ScriptInfoField::YcbcrMatrix(None)
            }
        },
    )(input)
}
