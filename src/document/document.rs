use std::fmt::{Debug, Formatter, Result};

/// Represents an AdvancedSubStation document, consisting of script info, an optional aegisub project garbage, styles, and events.
#[derive(Clone, Default, Debug)]
pub struct SubtitlesFile {
    pub script_info: ScriptInfo,
    pub project_garbage: Option<ProjectGarbage>,
    pub v4styles: Vec<Styles>,
    pub events: Vec<Dialogue>,
}

impl SubtitlesFile {
    fn _new(fields: Self) -> Self {
        Self { ..fields }
    }
}

/// `[Script Info]`
#[derive(Clone, Debug, PartialEq)]
pub struct ScriptInfo {
    pub comments: Vec<String>,
    pub title: String,
    pub script_type: String,
    pub wrap_style: WrapStyle,
    /// `true` if `"yes"` and `false` if `"no"`
    pub scaled_border_and_shadow: bool,
    pub ycbcr_matrix: Option<YcbcrMatrix>,
    pub original_script: String,
    pub play_res_x: i32,
    pub play_res_y: i32,
    pub original_translation: String,
    pub original_editing: String,
    pub original_timing: String,
    pub synch_point: String,
    pub script_updated_by: String,
    pub update_details: String,
}

impl ScriptInfo {
    pub fn new(fields: Self) -> Self {
        Self { ..fields }
    }
    pub fn print(&self) -> String {
        let _ass_comments = self
            .comments
            .iter()
            .map(|comment| format!("; {:#?}\n", comment))
            .collect::<String>();
        let readable_scaled_border_and_shadow = match self.scaled_border_and_shadow {
            true => "yes",
            false => "no",
        };
        format!(
            r"Title: {}\nScriptType: {}\nWrapStyle: {:?}\nScaledBorderAndShadow: {}\nYCbCr Matrix: {:?}\nOriginal Script: {:?}\nPlayResX: {}\nPlayResY: {}\nOriginal Translation: {:?}\nOriginal Editing: {:?}\nOriginal Timing: {:?}\nSynch Point: {:?}\nScript Updated By: {:?}\nUpdate Details: {:?}",
            self.title,
            self.script_type,
            self.wrap_style,
            readable_scaled_border_and_shadow,
            self.ycbcr_matrix,
            self.original_script,
            self.play_res_x,
            self.play_res_y,
            self.original_translation,
            self.original_editing,
            self.original_timing,
            self.synch_point,
            self.script_updated_by,
            self.update_details,
        )
    }
}

impl Default for ScriptInfo {
    fn default() -> Self {
        Self {
            title: String::from("Default Aegisub file"),
            comments: vec![],
            script_type: String::from("v4.00+"),
            wrap_style: WrapStyle::WrapStyle0,
            scaled_border_and_shadow: false,
            ycbcr_matrix: None,
            original_script: String::from(""),
            play_res_x: 0,
            play_res_y: 0,
            original_translation: String::from(""),
            original_editing: String::from(""),
            original_timing: String::from(""),
            synch_point: String::from(""),
            script_updated_by: String::from(""),
            update_details: String::from(""),
        }
    }
}

/// `[Aegisub Project Garbage]`
#[derive(Clone, Debug, Default)]
pub struct ProjectGarbage {
    pub last_style_storage: Option<String>,
    pub video_file: Option<String>,
    pub audio_file: Option<String>,
    pub video_ar_mode: f32,
    pub video_ar_value: f32,
    pub video_zoom_percent: f32,
    pub scroll_position: i32,
    pub active_line: i32,
    pub video_position: i32,
}

impl ProjectGarbage {
    fn _new(fields: Self, dummy_clip: Option<String>) -> Self {
        if dummy_clip.is_none() {
            let dummy_clip = Self::_new_dummy_video(1920, 1080, 40000, 23.976);
            Self {
                last_style_storage: fields.last_style_storage,
                video_ar_value: 0.0,
                video_file: Some(dummy_clip),
                audio_file: None,
                video_ar_mode: 0.0,
                video_zoom_percent: fields.video_zoom_percent,
                video_position: fields.video_position,
                active_line: fields.active_line,
                scroll_position: fields.scroll_position,
            }
        } else {
            Self {
                last_style_storage: fields.last_style_storage,
                video_ar_value: fields.video_ar_value,
                video_file: fields.video_file,
                video_zoom_percent: fields.video_zoom_percent,
                audio_file: fields.audio_file,
                video_position: fields.video_position,
                video_ar_mode: fields.video_ar_mode,
                active_line: fields.active_line,
                scroll_position: fields.scroll_position,
            }
        }
    }
    fn _new_dummy_video(res_x: i32, res_y: i32, duration_frames: i32, fps: f32) -> String {
        let dummy_clip = format!(
            "?dummy:{}:{}:{}:{}:135:139:143:",
            fps, duration_frames, res_y, res_x
        );
        dummy_clip
    }
    
    pub fn print(&self) -> String {
        format!(
            r#"Last Style Storage: {:?}Audio File: {:?}Video File: {:?}Video AR Mode: {}Video AR Value: {}Video Zoom Percent: {}Scroll Position: {}Active Line: {}Video Position: {}"#,
            self.last_style_storage,
            self.audio_file,
            self.video_file,
            self.video_ar_mode,
            self.video_ar_value,
            self.video_zoom_percent,
            self.scroll_position,
            self.active_line,
            self.video_position
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Styles {
    pub name: String,
    pub font_name: String,
    pub font_size: i32,
    pub primary_colour: String,
    pub secondary_colour: String,
    pub outline_colour: String,
    pub back_colour: String,
    /// `0` for false
    /// `-1` for true
    pub bold: i32,
    /// `0` for false
    /// `-1` for true
    pub italic: i32,
    /// `0` for false
    /// `-1` for true
    pub underline: i32,
    /// `0` for false
    /// `-1` for true
    pub strikeout: i32,
    pub scale_x: i32,
    pub scale_y: i32,
    pub spacing: i32,
    pub angle: i32,
    pub border_style: i32,
    pub outline: f32,
    pub shadow: i32,
    pub alignment: i32,
    pub margin_l: f32,
    pub margin_r: f32,
    pub margin_v: f32,
    pub encoding: StyleEncoding,
}

impl Styles {
    pub fn new(fields: Self) -> Self {
        Self { ..fields }
    }
    pub fn print(&self) -> String {
        format!(
            "Style: {},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:?}",
            self.name,
            self.font_name,
            self.font_size,
            self.primary_colour,
            self.secondary_colour,
            self.outline_colour,
            self.back_colour,
            self.bold,
            self.italic,
            self.underline,
            self.strikeout,
            self.scale_x,
            self.scale_y,
            self.spacing,
            self.angle,
            self.border_style,
            self.outline,
            self.shadow,
            self.alignment,
            self.margin_l,
            self.margin_r,
            self.margin_v,
            self.encoding,
        )
    }
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            name: String::from("Default"),
            font_name: String::from("Arial"),
            font_size: 48,
            primary_colour: String::from("&H00FFFFFF"),
            secondary_colour: String::from("&H000000FF"),
            outline_colour: String::from("&H00000000"),
            back_colour: String::from("&H00000000"),
            bold: 0,
            italic: 0,
            underline: 0,
            strikeout: 0,
            scale_x: 100,
            scale_y: 100,
            spacing: 0,
            angle: 0,
            border_style: 1,
            outline: 2.0,
            shadow: 2,
            alignment: 2,
            margin_l: 10.0,
            margin_r: 10.0,
            margin_v: 10.0,
            encoding: StyleEncoding::Default,
        }
    }
}

#[doc = "Only useful in unicode if the font doesn't have the proper unicode mapping."]
#[doc = "The default encoding is `Default`"]
#[derive(Clone, PartialEq)]
pub enum StyleEncoding {
    Ansi,
    Default,
    Mac,
    ShiftJis,
    Hangeul,
    Johab,
    GB2312,
    ChineseBIG5,
    Greek,
    Turkish,
    Vietnamese,
    Hebrew,
    Arabic,
    Baltic,
    Russian,
    Thai,
    EastEuropean,
    Oem,
}

impl Debug for StyleEncoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Ansi => write!(f, "{}", 1),
            Self::Default => write!(f, "{}", 2),
            Self::Mac => write!(f, "{}", 77),
            Self::ShiftJis => write!(f, "{}", 128),
            Self::Hangeul => write!(f, "{}", 129),
            Self::Johab => write!(f, "{}", 130),
            Self::GB2312 => write!(f, "{}", 134),
            Self::ChineseBIG5 => write!(f, "{}", 136),
            Self::Greek => write!(f, "{}", 161),
            Self::Turkish => write!(f, "{}", 162),
            Self::Vietnamese => write!(f, "{}", 163),
            Self::Hebrew => write!(f, "{}", 177),
            Self::Arabic => write!(f, "{}", 178),
            Self::Baltic => write!(f, "{}", 186),
            Self::Russian => write!(f, "{}", 204),
            Self::Thai => write!(f, "{}", 222),
            Self::EastEuropean => write!(f, "{}", 238),
            Self::Oem => write!(f, "{}", 255),
        }
    }
}
impl Default for StyleEncoding {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Dialogue,
    Comment,
}

/// `[Events]`
#[derive(Clone, Debug)]
pub struct Dialogue {
    pub type_: EventType,
    pub layer: i64,
    pub start: String,
    pub end: String,
    pub style: String,
    pub name: String,
    pub margin_l: f64,
    pub margin_r: f64,
    pub margin_v: f64,
    pub effect: String,
    pub text: String,
}

impl Dialogue {
    pub fn new(fields: Self) -> Self {
        Self { ..fields }
    }
    pub fn print(&self) -> String {
        format!(
            "{:?}: {},{},{},{},{},{},{},{},{},{}",
            self.type_,
            self.layer,
            self.start,
            self.end,
            self.style,
            self.name,
            self.margin_l,
            self.margin_r,
            self.margin_v,
            self.effect,
            self.text
        )
    }
}
impl Default for Dialogue {
    fn default() -> Self {
        Self {
            type_: EventType::Dialogue,
            layer: 0,
            start: String::from("0:00:00.00"),
            end: String::from("0:00:05.00"),
            style: String::from("Default"),
            name: "".to_string(),
            margin_l: 0.0,
            margin_r: 0.0,
            margin_v: 0.0,
            effect: "".to_string(),
            text: "".to_string(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum WrapStyle {
    /// `0`: Smart wrapping, top line is wider
    WrapStyle0,
    /// `1`: End-of-line word wrapping, only `\N` breaks
    WrapStyle1,
    /// `2`: No word wrapping, both `\n` and `\N` breaks
    WrapStyle2,
    /// `3`: Smart wrapping, bottom line is wider
    WrapStyle3,
}

impl Debug for WrapStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::WrapStyle0 => write!(f, "{}", 0),
            Self::WrapStyle1 => write!(f, "{}", 1),
            Self::WrapStyle2 => write!(f, "{}", 2),
            Self::WrapStyle3 => write!(f, "{}", 3),
        }
    }
}
impl Default for WrapStyle {
    fn default() -> Self {
        Self::WrapStyle0
    }
}

#[derive(Clone, PartialEq)]
pub enum YcbcrMatrix {
    Tv601,
    Pc601,
    Tv709,
    Pc709,
    Tvfcc,
    Pcfcc,
    Tv240m,
    Pc240m,
}

impl Debug for YcbcrMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Tv601 => write!(f, "TV.601"),
            Self::Pc601 => write!(f, "PC.601"),
            Self::Tv709 => write!(f, "TV.709"),
            Self::Pc709 => write!(f, "PC.709"),
            Self::Tvfcc => write!(f, "TV.FCC"),
            Self::Pcfcc => write!(f, "PC.FCC"),
            Self::Tv240m => write!(f, "TV.240M"),
            Self::Pc240m => write!(f, "PC.240M"),
        }
    }
}

#[cfg(unstable)]
pub trait StyleTags {
    fn fn_(font_name: f64) -> String {
        format!("fn({})", font_name)
    }
    fn fs(font_size: f64) -> String {
        format!("fs({})", font_size)
    }
    fn i(italisize: bool, _text: &str) -> String {
        if italisize == false {
            format!(r#"{{i{italisize}}}{}"#, 0)
        } else {
            format!(r#"{{i{italisize}}}{}"#, 1)
        }
    }
    fn p1(vector_drawing: &str) -> String {
        format!("q1({})", vector_drawing)
    }
    fn blur(blur_value: f64) -> String {
        format!("q1({})", blur_value)
    }
    fn an(angle: i8) -> String {
        if angle > 9 {
            format!("an{}", 9)
        } else if angle < 1 {
            format!("an{}", 1)
        } else {
            format!("an{}", angle)
        }
    }
    fn b(is_bold: bool) -> String {
        match is_bold {
            true => format!("b{}", 1),
            false => format!("b{}", 0),
        }
    }
    fn c(c: i8, color: i32) -> String {
        match c {
            1 => format!("1c&{}", color),
            2 => format!("2c&{}", color),
            3 => format!("3c&{}", color),
            i8::MIN..=0_i8 => format!("c&{}", color),
            4_i8..=i8::MAX => unimplemented!(),
        }
    }
    fn pos(x: f32, y: f32) -> String {
        format!("pos({},{})", x, y)
    }
    fn q2() -> String {
        "q2".to_string()
    }
}
