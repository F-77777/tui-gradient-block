pub struct BorderSegment {
    /// The text representation of this border segment.  
    /// - For a top border, this might be `"┌──────┐"`.  
    /// - For a right border, this might be `"┐││││┘"` (each character is rendered on a separate line from top to bottom, as newlines cannot be rendered directly).
    pub segment_text:
        ratatui::text::Line<'static>,

    /// An optional gradient applied to this border segment.  
    pub gradient:
        Option<Box<dyn colorgrad::Gradient>>,

    /// Determines whether this segment should be rendered with a gradient.  
    /// If `false`, the gradient (if present) will be ignored.
    pub should_use_gradient: bool,

    /// Determines whether the segment should be rendered at all.  
    /// If `false`, this segment will not be displayed.
    pub is_vertical: bool,

    /// The X-coordinate of the segment's position.
    pub x: u16,
    /// The Y-coordinate of the segment's position.
    pub y: u16,
}
// #[derive(serde::Serialize, serde::Deserialize, Clone)]
/// A collection of border segments representing different parts of a bordered structure.  
///
/// This struct holds individual `BorderSegment` instances for each section of the border
pub struct BorderSegments {
    /// The full top border segment.
    pub top_ln: BorderSegment,
    /// The full bottom border segment.
    pub bottom_ln: BorderSegment,
    /// The full left border segment.
    pub left_ln: BorderSegment,
    /// The full right border segment.
    pub right_ln: BorderSegment,
}
impl BorderSegments {
    /// Creates a new set of `BorderSegments` for a given rectangular area.
    ///
    /// This constructor initializes border segments based on the provided `Rect` dimensions,
    /// ensuring correct positioning for all parts of the border.
    ///
    /// # Arguments
    /// * `area` - A reference to a `Rect` defining the bounds of the border.
    ///
    /// # Returns
    /// A `BorderSegments` instance with all segments initialized at their respective positions.
    pub fn new(
        area: &ratatui::prelude::Rect,
    ) -> Self {
        Self {
            top_ln: BorderSegment::new(
                area.left() + 1,
                area.top(),
                false,
            ),
            bottom_ln: BorderSegment::new(
                area.left() + 1,
                area.bottom() - 1,
                false,
            ),
            left_ln: BorderSegment::new(
                area.left(),
                area.top() + 1,
                true,
            ),
            right_ln: BorderSegment::new(
                area.right() - 1,
                area.top() + 1,
                true,
            ),
        }
    }
}
impl BorderSegment {
    /// Creates a new, empty border segment at the specified position.  
    ///
    /// The segment starts with no text, no gradient, and is not set to be rendered by default.
    ///
    /// # Parameters
    /// - `x`: The X-coordinate of the segment's position.
    /// - `y`: The Y-coordinate of the segment's position.
    ///
    /// # Returns
    /// A `BorderSegment` instance with default values.
    pub fn new(
        x: u16,
        y: u16,
        is_vertical: bool,
    ) -> Self {
        Self {
            segment_text:
                ratatui::text::Line::raw(""),
            gradient: None,
            should_use_gradient: false,
            is_vertical,
            should_be_rendered: true,
            x,
            y,
        }
    }
}
