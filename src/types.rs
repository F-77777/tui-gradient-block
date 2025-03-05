pub mod typedefinitions {
    use super::{enums::GradientSegments, structs::Gradient};

    pub type GradientTheme = Vec<(GradientSegments, Gradient)>;
}

pub mod enums {
    #[derive(Clone)]
    /// Contains miscellaneous border types
    pub enum MiscBorderTypes {
        /// ```
        /// +=====+
        /// |     |
        /// |     |
        /// |     |
        /// +=====+
        /// ```
        Misc1,
        /// ```
        /// &-----&
        /// |     |
        /// +     +
        /// |     |
        /// &-----&
        /// ```
        Misc2,
        /// ``````
        /// ╬═════╬
        /// ║     ║
        /// ║     ║
        /// ║     ║
        /// ╬═════╬
        /// ```
        Misc3,
        /// ```
        /// $──~──$
        /// |     |
        /// ~     ~
        /// |     |
        /// $──~──$
        /// ```
        Misc4,
    }
    #[derive(Clone)]
    /// contains the custom border types
    /// Defines different border styles that can be applied.
    pub enum BorderStyle {
        /// A simple, single-line border (e.g., `│─┌┐└┘`).
        Plain,
        /// A double-line border for a more structured appearance (e.g., `║═╔╗╚╝`).
        Double,
        /// A thick border for strong emphasis (may vary depending on rendering support).
        Thick,
        /// A rounded border with smooth corners (e.g., `╭╮╰╯`).
        Rounded,
        /// A completely empty, user-defined custom border.
        CustomBorderType,
        /// A collection of miscellaneous border types.
        MiscBorder(MiscBorderTypes),
    }
    #[derive(Debug, Clone)]
    /// contains the custom border types
    /// Defines different border styles that can be applied.
    pub enum BorderStyleArgs {
        /// A simple, single-line border (e.g., `│─┌┐└┘`).
        Plain,
        /// A double-line border for a more structured appearance (e.g., `║═╔╗╚╝`).
        Double,
        /// A thick border for strong emphasis (may vary depending on rendering support).
        Thick,
        /// A rounded border with smooth corners (e.g., `╭╮╰╯`).
        Rounded,
    }
    #[derive(Clone, Debug)]
    /// Represents different segments of a border where gradients can be applied.
    ///
    /// This enum is used to control gradient effects for specific border sections,
    /// allowing for finer customization of visual styles.
    pub enum GradientSegments {
        /// The top border segment.
        Top,
        /// The bottom border segment.
        Bottom,
        /// The left border segment.
        Left,
        /// The right border segment.
        Right,
        /// The right portion of the top horizontal border.
        TopHorizontalRightLn,
        /// The left portion of the top horizontal border.
        TopHorizontalLeftLn,
        /// The right portion of the bottom horizontal border.
        BottomHorizontalRightLn,
        /// The left portion of the bottom horizontal border.
        BottomHorizontalLeftLn,
        /// The right portion of the top vertical border.
        TopVerticalRightLn,
        /// The left portion of the top vertical border.
        TopVerticalLeftLn,
        /// The right portion of the bottom vertical border.
        BottomVerticalRightLn,
        /// The left portion of the bottom vertical border.
        BottomVerticalLeftLn,
    }
    pub enum TitleDirection {
        Top,
        Bottom,
    }
}

pub mod structs {
    use bitflags::bitflags;
    use ratatui::layout::Alignment;

    use super::typedefinitions::GradientTheme;
    #[derive(Debug, Clone)]
    /// A set of symbols that define the appearance of a border.  
    /// Most symbols are optional, allowing for customization of corners, edges, and intersections.
    pub struct BorderSymbols {
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
    pub struct BorderSymbolSet {
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

    #[derive(Debug, Clone)]
    /// controls the optional fill with an optional gradient
    pub struct Fill {
        pub fill_string: Option<String>,
        pub gradient: Option<Gradient>,
    }

    #[derive(Debug, Clone)]
    pub struct BorderSegment {
        /// The text representation of this border segment.  
        /// - For a top border, this might be `"┌──────┐"`.  
        /// - For a right border, this might be `"┐││││┘"` (each character is rendered on a separate line from top to bottom, as newlines cannot be rendered directly).
        pub segment_text: String,

        /// An optional gradient applied to this border segment.  
        /// The gradient consists of a vector of RGB color values and a scaling factor.
        pub gradient: Gradient,

        /// Determines whether this segment should be rendered with a gradient.  
        /// If `false`, the gradient (if present) will be ignored.
        pub should_use_gradient: bool,

        /// Determines whether the segment should be rendered at all.  
        /// If `false`, this segment will not be displayed.
        pub should_be_rendered: bool,
        pub is_vertical: bool,

        /// The X-coordinate of the segment's position.
        pub x: u16,

        /// The Y-coordinate of the segment's position.
        pub y: u16,
    }
    pub struct BorderGradients {
        pub left: Vec<(u8, u8, u8)>,
        pub right: Vec<(u8, u8, u8)>,
        pub bottom: Vec<(u8, u8, u8)>,
        pub top: Vec<(u8, u8, u8)>,
        pub top_fac: f32,
        pub bottom_fac: f32,
        pub left_fac: f32,
        pub right_fac: f32,
    }

    #[derive(Debug, Clone)]
    /// A collection of border segments representing different parts of a bordered structure.  
    ///
    /// This struct holds individual `BorderSegment` instances for each section of the border,  
    /// ensuring flexibility in rendering complex layouts.
    pub struct BorderSegments {
        /// The left portion of the top horizontal border.
        pub top_horizontal_left_ln: BorderSegment,
        /// The right portion of the top horizontal border.
        pub top_horizontal_right_ln: BorderSegment,
        /// The left portion of the bottom horizontal border.
        pub bottom_horizontal_left_ln: BorderSegment,
        /// The right portion of the bottom horizontal border.
        pub bottom_horizontal_right_ln: BorderSegment,
        /// The upper portion of the left vertical border.
        pub top_vertical_left_ln: BorderSegment,
        /// The upper portion of the right vertical border.
        pub top_vertical_right_ln: BorderSegment,
        /// The lower portion of the left vertical border.
        pub bottom_vertical_left_ln: BorderSegment,
        /// The lower portion of the right vertical border.
        pub bottom_vertical_right_ln: BorderSegment,
        /// The full top border segment.
        pub top_ln: BorderSegment,
        /// The full bottom border segment.
        pub bottom_ln: BorderSegment,
        /// The full left border segment.
        pub left_ln: BorderSegment,
        /// The full right border segment.
        pub right_ln: BorderSegment,
    }
    #[derive(Clone)]
    pub struct GradientThemeSet {
        pub top_left: GradientTheme,
        pub top_right: GradientTheme,
        pub bottom_left: GradientTheme,
        pub bottom_right: GradientTheme,
        pub double_right: GradientTheme,
        pub double_left: GradientTheme,
        pub double_vertical: GradientTheme,
        pub double_horizontal: GradientTheme,
        pub up: GradientTheme,
        pub down: GradientTheme,
        pub left: GradientTheme,
        pub right: GradientTheme,
        pub base1: GradientTheme,
        pub base2: GradientTheme,
    }
    bitflags! {
        /// Represents individual border segments that can be split or modified.
        #[derive(PartialEq, Clone)]
        pub struct SplitBorderSegments: u8 {
            /// The left border segment is split.
            const LEFT   = 0b0001;
            /// The right border segment is split.
            const RIGHT  = 0b0010;
            /// The top border segment is split.
            const TOP    = 0b0100;
            /// The bottom border segment is split.
            const BOTTOM = 0b1000;
            /// All border segments are split.
            const ALL    = Self::LEFT.bits() | Self::RIGHT.bits() | Self::TOP.bits() | Self::BOTTOM.bits();
        }
    }
    #[derive(Clone, Debug)]
    pub struct Gradient {
        pub gradient_list: Vec<(u8, u8, u8)>,
        pub factor: f32,
    }
    #[derive(Clone, Debug)]
    pub struct Title {
        pub title_text: String,
        pub alignment: Alignment,
        pub gradients: Option<Gradient>,
    }
    #[derive(Clone, Debug)]
    pub struct TitleSet {
        pub top: Title,
        pub bottom: Title,
        pub left: Title,
        pub right: Title,
        pub top_left: Title,
        pub top_right: Title,
        pub bottom_left: Title,
        pub bottom_right: Title,
        pub double_right: Title,
        pub double_left: Title,
        pub double_vertical: Title,
        pub double_horizontal: Title,
        pub base1: Title,
        pub base2: Title,
    }
}

pub mod impls {
    use super::structs::{BorderSegment, BorderSegments, Gradient};
    use ratatui::layout::Rect;

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
        pub fn new(x: u16, y: u16, is_vertical: bool) -> Self {
            Self {
                segment_text: String::new(),
                gradient: Gradient {
                    gradient_list: Vec::new(),
                    factor: 0.0,
                },
                should_use_gradient: false,
                should_be_rendered: false,
                is_vertical,
                x,
                y,
            }
        }
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
        pub fn new(area: &Rect) -> Self {
            Self {
                top_horizontal_left_ln: BorderSegment::new(
                    area.left(),
                    area.top(),
                    false,
                ),
                top_horizontal_right_ln: BorderSegment::new(
                    area.right() / 2,
                    area.top(),
                    false,
                ),
                bottom_horizontal_left_ln: BorderSegment::new(
                    area.left(),
                    area.bottom() - 1,
                    false,
                ),
                bottom_horizontal_right_ln: BorderSegment::new(
                    area.right() / 2,
                    area.bottom() - 1,
                    false,
                ),
                top_vertical_left_ln: BorderSegment::new(
                    area.left(),
                    area.top(),
                    true,
                ),
                top_vertical_right_ln: BorderSegment::new(
                    area.right() - 1,
                    area.top(),
                    true,
                ),
                bottom_vertical_left_ln: BorderSegment::new(
                    area.left(),
                    ((area.height as usize + 1) / 2) as u16,
                    true,
                ),
                bottom_vertical_right_ln: BorderSegment::new(
                    area.right() - 1,
                    ((area.height as usize + 1) / 2) as u16,
                    true,
                ),
                top_ln: BorderSegment::new(
                    area.left(),
                    area.top(),
                    false,
                ),
                bottom_ln: BorderSegment::new(
                    area.left(),
                    area.bottom() - 1,
                    false,
                ),
                left_ln: BorderSegment::new(
                    area.left(),
                    area.top(),
                    true,
                ),
                right_ln: BorderSegment::new(
                    area.right() - 1,
                    area.top(),
                    true,
                ),
            }
        }
    }
}
