pub mod document;


pub enum _StyleTagsFields {
    Fn(f64),
    Fs(f64),
    I(i8, String),
    P1(String),
    Blur(f64),
    An(_Alignment),
    B(bool),
    C(i8, i32),
    Pos(f32, f32),
    Q2,
}

#[derive(Debug, PartialEq)]
pub enum _Alignment {
    _TopLeft,
    _TopCenter,
    _TopRight,
    _CenterLeft,
    _CenterCenter,
    _CenterRight,
    _BottomLeft,
    _BottomCenter,
    _BottomRight,
}