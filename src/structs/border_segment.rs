use crate::structs::border_symbols::SegmentSet;
use ratatui::prelude::Alignment;
use tui_rule::{
    presets::borders::plain::*, Rule, Set, VerticalAlignment,
};
pub struct BorderSegment {
    pub should_be_rendered: bool,
    pub seg: Rule,
}
/// A collection of border segments representing different parts of a bordered structure.  
///
/// This struct holds individual `BorderSegment` instances for each section of the border
pub struct BorderSegments {
    /// The full top border segment.
    pub top: BorderSegment,
    /// The full bottom border segment.
    pub bottom: BorderSegment,
    /// The full left border segment.
    pub left: BorderSegment,
    /// The full right border segment.
    pub right: BorderSegment,
}
impl Default for BorderSegments {
    fn default() -> Self {
        Self::new()
    }
}

impl BorderSegments {
    /// Creates a new set of `BorderSegments`
    /// # Returns
    /// A `BorderSegments` instance with all segments initialized at their respective positions.
    pub fn new() -> Self {
        let mut new_self = Self {
            top: BorderSegment::new(false, TOP),
            bottom: BorderSegment::new(false, BOTTOM),
            left: BorderSegment::new(true, LEFT),
            right: BorderSegment::new(true, RIGHT),
        };
        new_self.right.seg.horizontal_alignment = Alignment::Right;
        new_self.left.seg.horizontal_alignment = Alignment::Left;
        new_self.bottom.seg.vertical_alignment =
            VerticalAlignment::Bottom;
        new_self.top.seg.vertical_alignment = VerticalAlignment::Top;
        new_self
    }
    pub fn from_segment_set(mut self, set: SegmentSet) -> Self {
        self.right.seg = self.right.seg.with_set(set.right);
        self.left.seg = self.left.seg.with_set(set.left);
        self.top.seg = self.top.seg.with_set(set.top);
        self.bottom.seg = self.bottom.seg.with_set(set.bottom);
        self
    }
}
impl BorderSegment {
    /// The segment starts with a plain rule
    /// x and y are 0 by default,
    /// # Returns
    /// A `BorderSegment` instance with default values
    pub fn new(is_vertical: bool, set: Set) -> Self {
        Self {
            should_be_rendered: true,
            seg: match is_vertical {
                true => Rule::from_set(set).vertical(),
                false => Rule::from_set(set).horizontal(),
            }
            .area_margin(ratatui::layout::Margin::new(0, 0)),
        }
    }
}
