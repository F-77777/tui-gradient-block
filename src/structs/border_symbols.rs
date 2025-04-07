use derive_builder::Builder;
use getset::{Getters, Setters};
use tui_rule::Set;
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Getters, Setters, Builder, Clone, Debug)]
pub struct SegmentSet {
    pub top: Set,
    pub bottom: Set,
    pub right: Set,
    pub left: Set,
}
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Getters, Setters, Builder, Clone, Debug)]
pub struct Corners {
    pub bottom_right: char,
    pub bottom_left: char,
    pub top_left: char,
    pub top_right: char,
}
impl SegmentSet {
    #[cfg(feature = "serde")]
    pub fn from_json(path: &str) -> Result<Self, E> {
        crate::generate_from_json!(path, Self)
    }
    pub fn from_ratatui_set(
        set: ratatui::symbols::border::Set,
    ) -> Self {
        macro_rules! parsed {
            ($s:expr) => {
                $s.parse::<char>().unwrap_or(' ')
            };
        }
        let top = parsed!(set.horizontal_top);
        let bottom = parsed!(set.horizontal_bottom);
        let right = parsed!(set.vertical_right);
        let left = parsed!(set.vertical_left);
        let top_right = parsed!(set.top_right);
        let top_left = parsed!(set.top_left);
        let bottom_right = parsed!(set.bottom_right);
        let bottom_left = parsed!(set.bottom_left);
        Self {
            top: Set {
                start: top_left,
                rep_1: top,
                center: top,
                rep_2: top,
                end: top_right,
            },
            bottom: Set {
                start: bottom_left,
                rep_1: bottom,
                center: bottom,
                rep_2: bottom,
                end: bottom_right,
            },
            right: Set {
                start: top_right,
                rep_1: right,
                center: right,
                rep_2: right,
                end: bottom_right,
            },
            left: Set {
                start: top_left,
                rep_1: left,
                center: left,
                rep_2: left,
                end: bottom_left,
            },
        }
    }
}
