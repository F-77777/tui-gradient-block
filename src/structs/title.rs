use crate::types::T;
pub struct TitleSet<'a> {
    pub up: T<'a>,
    pub down: T<'a>,
    pub left: T<'a>,
    pub right: T<'a>,
    pub top_left: T<'a>,
    pub top_right: T<'a>,
    pub bottom_left: T<'a>,
    pub bottom_right: T<'a>,
    pub double_corners_right: T<'a>,
    pub double_corners_left: T<'a>,
    pub vertical: T<'a>,
    pub horizontal: T<'a>,
    pub misc1: T<'a>,
    pub misc2: T<'a>,
}
