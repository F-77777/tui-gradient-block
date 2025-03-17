use crate::types::E;
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Hash,
)]
/// A set of symbols that define the appearance of a border.  
/// Most symbols are optional, allowing for customization of corners, edges, and intersections.
pub struct BorderSymbolsSet {
    /// The character used for the top-left corner of the border (e.g., '┌' for a box).  
    /// If not set, a default may be used.
    pub top_left: Option<char>,

    /// The character used for the top-right corner of the border (e.g., '┐' for a box).
    pub top_right: Option<char>,

    /// The character used for the bottom-left corner of the border (e.g., '└' for a box).
    pub bottom_left: Option<char>,

    /// The character used for the bottom-right corner of the border (e.g., '┘' for a box).
    pub bottom_right: Option<char>,

    /// The character placed at the center of the top border.  
    /// Defaults to `top_horizontal` if not specified.
    pub top_center: Option<char>,

    /// The character placed at the center of the bottom border.  
    /// Defaults to `bottom_horizontal` if not specified.
    pub bottom_center: Option<char>,

    /// The character placed at the center of the left border.  
    /// Defaults to `left_vertical` if not specified.
    pub left_center: Option<char>,

    /// The character placed at the center of the right border.  
    /// Defaults to `right_vertical` if not specified.
    pub right_center: Option<char>,

    /// The character used for the horizontal segments of the top border (e.g., '─' for a solid line).  
    /// Defaults to '─' if not set.
    pub top_horizontal: Option<char>,

    /// The character used for the vertical segments of the left border (e.g., '│' for a solid line).  
    /// Defaults to '│' if not set.
    pub left_vertical: Option<char>,

    /// The character used for the horizontal segments of the bottom border (e.g., '─' for a solid line).  
    /// Defaults to '─' if not set.
    pub bottom_horizontal: Option<char>,

    /// The character used for the vertical segments of the right border (e.g., '│' for a solid line).  
    /// Defaults to '│' if not set.
    pub right_vertical: Option<char>,

    /// The character repeated for the right side of the top border.  
    /// Defaults to `top_horizontal` if not set.
    pub top_horizontal_right: Option<char>,

    /// The character repeated for the right side of the bottom border.  
    /// Defaults to `bottom_horizontal` if not set.
    pub bottom_horizontal_right: Option<char>,

    /// The character repeated for the left side of the top border.  
    /// Defaults to `top_horizontal` if not set.
    pub top_horizontal_left: Option<char>,

    /// The character repeated for the left side of the bottom border.  
    /// Defaults to `bottom_horizontal` if not set.
    pub bottom_horizontal_left: Option<char>,

    /// The character repeated for the top section of the right border.  
    /// Defaults to `right_vertical` if not set.
    pub top_vertical_right: Option<char>,

    /// The character repeated for the bottom section of the right border.  
    /// Defaults to `right_vertical` if not set.
    pub bottom_vertical_right: Option<char>,

    /// The character repeated for the top section of the left border.  
    /// Defaults to `left_vertical` if not set.
    pub top_vertical_left: Option<char>,

    /// The character repeated for the bottom section of the left border.  
    /// Defaults to `left_vertical` if not set.
    pub bottom_vertical_left: Option<char>,
}
/// A struct that defines the characters used for different parts of a border's visual appearance.
/// Each field specifies a character to represent a particular section of the border.
///
/// # Fields
/// - `top_left`: The character used for the top-left corner of the border.
/// - `bottom_left`: The character used for the bottom-left corner of the border.
/// - `top_right`: The character used for the top-right corner of the border.
/// - `bottom_right`: The character used for the bottom-right corner of the border.
/// - `top`: The character used for the top edge of the border.
/// - `bottom`: The character used for the bottom edge of the border.
/// - `left`: The character used for the left side of the border.
/// - `right`: The character used for the right side of the border.
/// - `bottom_center`: The character used for the bottom center section of the border.
/// - `top_center`: The character used for the top center section of the border.
/// - `right_center`: The character used for the right center section of the border.
/// - `left_center`: The character used for the left center section of the border.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Hash,
    PartialEq,
)]
pub struct BorderSymbolsSetMin {
    pub top_left: char,
    pub bottom_left: char,
    pub top_right: char,
    pub bottom_right: char,
    pub top: char,
    pub bottom: char,
    pub left: char,
    pub right: char,
    pub bottom_center: char,
    pub top_center: char,
    pub right_center: char,
    pub left_center: char,
}
impl BorderSymbolsSetMin {
    pub fn from_json(
        path: &str,
    ) -> Result<Self, E> {
        crate::generate_from_json!(path, Self)
    }
    pub fn to_json(self) -> Result<String, E> {}
}

impl BorderSymbolsSet {
    pub fn from_json(
        path: &str,
    ) -> Result<Self, E> {
        crate::generate_from_json!(path, Self)
    }
    pub fn new() -> Self {
        Self {
            top_left: None,
            top_right: None,
            bottom_right: None,
            bottom_left: None,
            top_center: None,
            bottom_center: None,
            left_center: None,
            right_center: None,
            bottom_horizontal: None,
            top_horizontal: None,
            left_vertical: None,
            right_vertical: None,
            top_horizontal_right: None,
            top_horizontal_left: None,
            bottom_horizontal_left: None,
            bottom_horizontal_right: None,
            bottom_vertical_right: None,
            bottom_vertical_left: None,
            top_vertical_left: None,
            top_vertical_right: None,
        }
    }
}
