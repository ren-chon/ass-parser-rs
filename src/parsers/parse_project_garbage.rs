use super::{
    ProjectGarbageField, {floating, integer, parse_string1},
};
use crate::prelude::ProjectGarbage;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    multi::many0,
    sequence::preceded,
    IResult,
};

pub(crate) fn parse_apg(input: &str) -> IResult<&str, ProjectGarbage> {
    map(
        many0(preceded(
            opt(multispace0),
            alt((
                last_style_storage,
                audio_file,
                video_file,
                video_ar_mode,
                video_ar_value,
                video_zoom_percent,
                scroll_position,
                active_line,
                video_position,
            )),
        )),
        |fields| {
            let mut project_garbage = ProjectGarbage::default();
            for field in fields {
                match field {
                    ProjectGarbageField::LastStyleStorage(lss) => {
                        project_garbage.last_style_storage = Some(lss)
                    }
                    ProjectGarbageField::AudioFile(af) => project_garbage.audio_file = Some(af),
                    ProjectGarbageField::VideoFile(vf) => project_garbage.video_file = Some(vf),
                    ProjectGarbageField::VideoArMode(varm) => project_garbage.video_ar_mode = varm,
                    ProjectGarbageField::VideoArValue(varv) => {
                        project_garbage.video_ar_value = varv
                    }
                    ProjectGarbageField::VideoZoomPercent(vzp) => {
                        project_garbage.video_zoom_percent = vzp
                    }
                    ProjectGarbageField::ScrollPosition(sp) => project_garbage.scroll_position = sp,
                    ProjectGarbageField::ActiveLine(al) => project_garbage.active_line = al,
                    ProjectGarbageField::VideoPosition(vp) => project_garbage.video_position = vp,
                }
            }
            project_garbage
        },
    )(input)
}

fn last_style_storage(input: &str) -> IResult<&str, ProjectGarbageField> {
    map(
        preceded(tag("Last Style Storage: "), parse_string1),
        |lss: &str| ProjectGarbageField::LastStyleStorage(lss.to_string()),
    )(input)
}
fn audio_file(input: &str) -> IResult<&str, ProjectGarbageField> {
    map(preceded(tag("Audio File: "), parse_string1), |af: &str| {
        ProjectGarbageField::AudioFile(af.to_string())
    })(input)
}
fn video_file(input: &str) -> IResult<&str, ProjectGarbageField> {
    map(preceded(tag("Video File: "), parse_string1), |vf: &str| {
        ProjectGarbageField::VideoFile(vf.to_string())
    })(input)
}
fn video_ar_mode(input: &str) -> IResult<&str, ProjectGarbageField> {
    map(preceded(tag("Video AR Mode: "), floating), |varm| {
        ProjectGarbageField::VideoArMode(varm)
    })(input)
}
fn video_ar_value(input: &str) -> IResult<&str, ProjectGarbageField> {
    map(preceded(tag("Video AR Value: "), floating), |varv| {
        ProjectGarbageField::VideoArValue(varv)
    })(input)
}
fn video_zoom_percent(input: &str) -> IResult<&str, ProjectGarbageField> {
    map(preceded(tag("Video Zoom Percent: "), floating), |vzp| {
        ProjectGarbageField::VideoZoomPercent(vzp)
    })(input)
}
fn scroll_position(input: &str) -> IResult<&str, ProjectGarbageField> {
    map(
        preceded(tag("Scroll Position: "), integer),
        ProjectGarbageField::ScrollPosition,
    )(input)
}
fn active_line(input: &str) -> IResult<&str, ProjectGarbageField> {
    map(preceded(tag("Active Line: "), integer), |al| {
        ProjectGarbageField::ActiveLine(al)
    })(input)
}
fn video_position(input: &str) -> IResult<&str, ProjectGarbageField> {
    map(preceded(tag("Video Position: "), integer), |vp| {
        ProjectGarbageField::VideoPosition(vp)
    })(input)
}
