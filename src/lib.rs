use bitflags::bitflags;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols::border::{DOUBLE, PLAIN, ROUNDED, THICK},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};
use std::rc::Rc;
use unicode_width::UnicodeWidthChar;
/// A struct that represents a customizable block with gradient text, borders, and other visual elements.
///
/// This struct allows you to create and manage blocks that have a gradient color effect for text,
/// customizable borders, and areas with specific alignments and fill styles.
///
/// # Fields
/// - `bordertype`: Specifies the type of border style used for the block.
/// - `fill`: Defines the fill style for the block's area (e.g., solid or gradient).
/// - `top_titles`: A vector of tuples where each tuple contains:
///   - A `String` representing the title text.
///   - A `TitleAlignment` indicating the alignment of the title.
///   - An optional tuple of gradient colors (represented as a vector of `(u8, u8, u8)` tuples) and
///     a factor that controls the spread of the gradient effect.
/// - `bottom_titles`: A similar vector to `top_titles`, but for titles placed at the bottom of the block.
/// - `border_symbols`: Defines the symbols used for the block's borders.
/// - `border_segments`: Defines the segments of the block's border.
/// - `split_segments`: Specifies how the block's border is split into different sections.
/// - `area`: A `Rect` representing the block's area, typically used for positioning and layout.
///
/// # Example
/// ```rust
/// let gradient_block = TuiGradientblock {
///     bordertype: BorderStyle::Solid,
///     fill: Fill::SolidColor((255, 0, 0)),
///     top_titles: vec![("Top Title".to_owned(), TitleAlignment::Center, None)],
///     bottom_titles: vec![("Bottom Title".to_owned(), TitleAlignment::Right, None)],
///     border_symbols: BorderSymbols::Default,
///     border_segments: BorderSegments::Full,
///     split_segments: SplitBorderSegments::None,
///     area: Rect::new(0, 0, 10, 5),
/// };
/// ```

#[derive(Clone)]
pub struct TuiGradientblock {
    bordertype: BorderStyle,
    fill: Fill,
    top_titles: Vec<(String, TitleAlignment, Option<(Vec<(u8, u8, u8)>, f32)>)>,
    bottom_titles: Vec<(String, TitleAlignment, Option<(Vec<(u8, u8, u8)>, f32)>)>,
    border_symbols: BorderSymbols,
    border_segments: BorderSegments,
    split_segments: SplitBorderSegments,
    area: Rect,
}
/// A constant error message displayed when there are not enough colors provided for a gradient.
/// It encourages the user to use at least two colors for a gradient effect or repeat the same color
/// if a solid color is desired.
///
/// Example of how to provide colors for a solid color:
/// ❌ `[(250, 2, 238)]`
/// ✅ `[(250, 2, 238), (250, 2, 238)]`
pub const ERROR_MESSAGE: &str = "
╓───── IMPORTANT ─────╖
║                     ║
║ Use at least two    ║
║ colors.             ║
║                     ║
║ If you want to use  ║
║ a solid color,      ║
║ enter the same      ║
║ color more than once║
║                     ║
║ Example: Solid pink ║
║                     ║
║ ❌ [(250, 2, 238)]  ║
║ ✅ [                ║
║     (250, 2, 238),  ║
║     (250, 2, 238),  ║
║    ]                ║
║                     ║
╙─────────────────────╜
";

/// A set of predefined border styles for different visual aesthetics. Each `BorderStyleInfo`
/// instance defines the characters to be used for different parts of the border (corners, sides, and centers).
///
/// # Variants:
/// - `MISC1`: A style with standard "+" corners, and "=" for top/bottom edges, with "|" for side edges.
/// - `MISC2`: A style with "╘" and "╛" for the bottom corners, and "=" for top and bottom edges.
/// - `MISC3`: A style with "╬" for the corners and sides, and "═" for the top and bottom edges.
/// - `MISC4`: A unique style with "$" corners, "~" for center sides, and "─" for top and bottom edges.
///
/// These styles can be used to customize the appearance of borders for blocks, areas, or text layouts.
pub const MISC1: BorderStyleInfo = BorderStyleInfo {
    top_left: '+',
    bottom_left: '+',
    top_right: '+',
    bottom_right: '+',
    top: '=',
    bottom: '=',
    left: '|',
    right: '|',
    bottom_center: '=',
    top_center: '=',
    right_center: '|',
    left_center: '|',
};

/// A border style with special bottom corners "╘" and "╛" and "═" edges for a different aesthetic.
pub const MISC2: BorderStyleInfo = BorderStyleInfo {
    top_left: '╔',
    bottom_left: '╚',
    top_right: '╗',
    bottom_right: '╝',
    top: '═',
    bottom: '═',
    right_center: '╬',
    left_center: '╬',
    left: '║',
    right: '║',
    bottom_center: '╬',
    top_center: '╬',
};

/// A border style with "╬" for the corners and sides, giving a more solid and ornate border.
pub const MISC3: BorderStyleInfo = BorderStyleInfo {
    top_left: '╬',
    bottom_left: '╬',
    top_right: '╬',
    bottom_right: '╬',
    top: '═',
    bottom: '═',
    left: '║',
    right: '║',
    bottom_center: '═',
    top_center: '═',
    right_center: '║',
    left_center: '║',
};

/// A more unique border style featuring "$" for the corners, "~" for the center sides, and "─" for the edges.
pub const MISC4: BorderStyleInfo = BorderStyleInfo {
    top_left: '$',
    bottom_left: '$',
    top_right: '$',
    bottom_right: '$',
    top: '─',
    bottom: '─',
    left: '│',
    right: '│',
    bottom_center: '$',
    top_center: '~',
    right_center: '~',
    left_center: '~',
};

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
pub struct BorderStyleInfo {
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

impl TuiGradientblock {
    pub fn new(area: &Rect, split_segments: SplitBorderSegments) -> Self {
        Self {
            bordertype: BorderStyle::Plain,
            fill: Fill {
                fill_string: None,
                gradient: None,
            },
            top_titles: Vec::new(),
            bottom_titles: Vec::new(),
            border_symbols: BorderSymbols {
                top_left: None,
                top_right: None,
                bottom_left: None,
                bottom_right: None,
                top_center: None,
                bottom_center: None,
                left_center: None,
                right_center: None,
                bottom_horizontal: None,
                top_horizontal: None,
                left_vertical: None,
                right_vertical: None,
                top_horizontal_right: None,
                bottom_horizontal_right: None,
                top_horizontal_left: None,
                bottom_horizontal_left: None,
                top_vertical_right: None,
                bottom_vertical_right: None,
                top_vertical_left: None,
                bottom_vertical_left: None,
            },
            border_segments: BorderSegments::new(area),
            split_segments,
            area: *area,
        }
    }
    /// Interpolates between two colors based on a factor and a time value (t).
    /// This function computes a smooth transition between the `start` and `end` RGB colors
    /// using the given parameter `t` (which represents the position between the colors)
    /// and `factor` (which controls the curve of the interpolation).
    ///
    /// # Parameters
    /// - `start`: A tuple representing the RGB values of the starting color (each value is a `u8`).
    /// - `end`: A tuple representing the RGB values of the ending color (each value is a `u8`).
    /// - `t`: A floating-point value (`f32`) between 0 and 1 that represents the interpolation factor,
    ///   where `t = 0` gives the `start` color, and `t = 1` gives the `end` color.
    /// - `factor`: A floating-point value (`f32`) that influences the smoothness or intensity of the interpolation.
    ///   Higher values make the interpolation curve sharper, while lower values create a more gradual transition.
    ///
    /// # Returns
    /// A tuple `(u8, u8, u8)` representing the interpolated RGB color, where each value is a byte (0-255).
    ///
    /// # Example
    /// ```rust
    /// let start_color = (255, 0, 0); // Red
    /// let end_color = (0, 0, 255); // Blue
    /// let t = 0.5; // Midway between the colors
    /// let factor = 1.0; // Linear interpolation
    /// let result = interpolate_color(start_color, end_color, t, factor);
    /// ```
    /// In this example, `result` will contain an RGB value that is a mix of red and blue, producing purple.
    /// # Note
    /// - The interpolation is calculated using a smooth function based on `t` raised to the power of `factor`,
    ///   which controls the rate of transition between the `start` and `end` colors.
    pub fn interpolate_color(
        start: (u8, u8, u8),
        end: (u8, u8, u8),
        t: f32,
        factor: f32,
    ) -> (u8, u8, u8) {
        let lerp = |s, e| ((1.0 - t.powf(factor)) * (s as f32) + t.powf(factor) * (e as f32)) as u8;
        (
            lerp(start.0, end.0),
            lerp(start.1, end.1),
            lerp(start.2, end.2),
        )
    }
    /// Creates a gradient effect on a given text by interpolating between a list of colors
    /// based on the position of each character in the string.
    ///
    /// # Parameters
    /// - `text`: A reference to the input `String` for which the gradient effect will be applied.
    /// - `colors`: A vector of tuples representing the RGB values of the gradient colors. Each tuple
    ///   contains three `u8` values representing the Red, Green, and Blue components of a color.
    /// - `factor`: A floating-point value (`f32`) that determines the intensity or spread of the gradient.
    ///   Higher values will increase the spread of the gradient colors.
    ///
    /// # Returns
    /// A `Vec<Span<'static>>` containing `Span` elements, where each `Span` represents a styled portion
    /// of the input text with the corresponding color from the gradient.
    ///
    /// # Panics
    /// This function will panic if the number of colors in `colors` is less than 2. A gradient requires
    /// at least two colors to interpolate between. If fewer colors are provided, the following error message
    /// will be displayed:
    ///
    /// ```text
    /// ╓───── IMPORTANT ─────╖
    /// ║                     ║
    /// ║ Use at least two    ║
    /// ║ colors.             ║
    /// ║                     ║
    /// ║ If you want to use  ║
    /// ║ a solid color,      ║
    /// ║ enter the same      ║
    /// ║ color more than once║
    /// ║                     ║
    /// ║ Example: Solid pink ║
    /// ║                     ║
    /// ║ ❌ [(250, 2, 238)]  ║
    /// ║ ✅ [                ║
    /// ║     (250, 2, 238),  ║
    /// ║     (250, 2, 238),  ║
    /// ║    ]                ║
    /// ║                     ║
    /// ╙─────────────────────╜
    /// ```

    /// # Example
    /// ```rust
    /// let text = "Hello, World!".to_string();
    /// let colors = vec![(255, 0, 0), (0, 255, 0), (0, 0, 255)];
    /// let factor = 1.0;
    /// let gradient_text = create_gradient_text(&text, colors, factor);
    /// ```
    /// In the above example, the `gradient_text` will be a vector of `Span`s with the text "Hello, World!"
    /// styled with a gradient transitioning from red to green to blue.
    /// # Note
    /// - The `interpolate_color` function is used internally to calculate the intermediate colors based on the
    ///   position of the character relative to the total width of the text.

    pub fn create_gradient_text(
        text: &str,
        colors: Vec<(u8, u8, u8)>,
        factor: f32,
    ) -> Vec<Span<'static>> {
        let mut gradient_colors = Vec::new();

        let char_widths: Vec<usize> = text.chars().map(|c| c.width().unwrap_or(1)).collect();
        let total_width: usize = char_widths.iter().sum();

        let num_colors = colors.len();

        if num_colors < 2 {
            ratatui::restore();
            panic!("{}", ERROR_MESSAGE);
        }
        let mut accumulated_width = 0.0;
        for (i, _) in text.chars().enumerate() {
            let char_width = char_widths[i] as f32;
            let relative_pos = accumulated_width / total_width as f32;
            let section_index = (relative_pos * (num_colors - 1) as f32).floor() as usize;
            let t = (relative_pos * (num_colors - 1) as f32) % 1.0;
            let next_color_index = (section_index + 1).min(num_colors - 1);
            gradient_colors.push(Self::interpolate_color(
                colors[section_index],
                colors[next_color_index],
                t,
                factor,
            ));
            accumulated_width += char_width;
        }
        let mut gradient_text = Vec::new();
        for (i, c) in text.chars().zip(gradient_colors) {
            gradient_text.push(Span::styled(
                i.to_string(),
                Style::default().fg(Color::Rgb(c.0, c.1, c.2)),
            ));
        }
        gradient_text
    }
    /// Sets gradient colors for specific segments of the border.
    ///
    /// # Parameters
    /// - `gradientlist`: A vector of tuples where each tuple contains:
    ///   - A `GradientSegments` enum value specifying which border segment to apply the gradient to.
    ///   - A vector of RGB color tuples (`Vec<(u8, u8, u8)>`) representing the gradient colors.
    ///   - A `f32` value representing the gradient factor (e.g., intensity or blending weight).
    ///
    /// # Notes
    /// - If a gradient should be a solid color, provide the same RGB value twice.
    /// - Gradients must have at least two different colors to transition properly.
    ///
    /// # Example 1: Applying a gradient to the top border
    /// ```
    /// let border = TuiGradientblock::new().set_gradients(vec![
    ///     (GradientSegments::Top, vec![(255, 0, 0), (0, 0, 255)], 0.5),
    /// ]);
    /// ```
    ///
    /// # Example 2: Applying a solid color to the right border
    /// ```
    /// let border = TuiGradientblock::new().set_gradients(vec![
    ///     (GradientSegments::Right, vec![(50, 50, 50), (50, 50, 50)], 1.0),
    /// ]);
    /// ```
    pub fn set_gradients(
        mut self,
        gradientlist: Vec<(GradientSegments, Vec<(u8, u8, u8)>, f32)>,
    ) -> Self {
        for (segment, colors, factor) in gradientlist {
            let gradient_data = Some((colors, factor));

            match segment {
                GradientSegments::Top => {
                    self.border_segments.top_ln.gradient = gradient_data;
                    self.border_segments.top_ln.should_use_gradient = true;
                }
                GradientSegments::Bottom => {
                    self.border_segments.bottom_ln.gradient = gradient_data;
                    self.border_segments.bottom_ln.should_use_gradient = true;
                }
                GradientSegments::Left => {
                    self.border_segments.left_ln.gradient = gradient_data;
                    self.border_segments.left_ln.should_use_gradient = true;
                }
                GradientSegments::Right => {
                    self.border_segments.right_ln.gradient = gradient_data;
                    self.border_segments.right_ln.should_use_gradient = true;
                }
                GradientSegments::TopHorizontalLeftLn => {
                    self.border_segments.top_horizontal_left_ln.gradient = gradient_data;
                    self.border_segments
                        .top_horizontal_left_ln
                        .should_use_gradient = true;
                }
                GradientSegments::TopHorizontalRightLn => {
                    self.border_segments.top_horizontal_right_ln.gradient = gradient_data;
                    self.border_segments
                        .top_horizontal_right_ln
                        .should_use_gradient = true;
                }
                GradientSegments::BottomHorizontalLeftLn => {
                    self.border_segments.bottom_horizontal_left_ln.gradient = gradient_data;
                    self.border_segments
                        .bottom_horizontal_left_ln
                        .should_use_gradient = true;
                }
                GradientSegments::BottomHorizontalRightLn => {
                    self.border_segments.bottom_horizontal_right_ln.gradient = gradient_data;
                    self.border_segments
                        .bottom_horizontal_right_ln
                        .should_use_gradient = true;
                }
                GradientSegments::TopVerticalLeftLn => {
                    self.border_segments.top_vertical_left_ln.gradient = gradient_data;
                    self.border_segments
                        .top_vertical_left_ln
                        .should_use_gradient = true;
                }
                GradientSegments::TopVerticalRightLn => {
                    self.border_segments.top_vertical_right_ln.gradient = gradient_data;
                    self.border_segments
                        .top_vertical_right_ln
                        .should_use_gradient = true;
                }
                GradientSegments::BottomVerticalLeftLn => {
                    self.border_segments.bottom_vertical_left_ln.gradient = gradient_data;
                    self.border_segments
                        .bottom_vertical_left_ln
                        .should_use_gradient = true;
                }
                GradientSegments::BottomVerticalRightLn => {
                    self.border_segments.bottom_vertical_right_ln.gradient = gradient_data;
                    self.border_segments
                        .bottom_vertical_right_ln
                        .should_use_gradient = true;
                }
            }
        }
        self
    }

    /// Sets the border style for the block.
    ///
    /// If this function is not called, the border will be plain by default.
    ///
    /// # Parameters
    /// - `style`: A `BorderStyle` enum value that determines the appearance of the border.
    ///   - `BorderStyle::Plain`: A simple, unstyled border.
    ///   - `BorderStyle::Double`: A double-lined border.
    ///   - `BorderStyle::Thick`: A thick-stroked border.
    ///   - `BorderStyle::Rounded`: A border with rounded corners.
    ///   - `BorderStyle::MiscBorder(MiscBorderTypes)`: A selection of miscellaneous predefined border styles.
    ///   - `BorderStyle::CustomBorderType`: Allows custom border symbols to be set manually.
    ///
    /// # Example 1: Using a standard border style
    /// ```
    /// let border = TuiGradientblock::new().border_style(BorderStyle::Double);
    /// ```
    ///
    /// # Example 2: Using a miscellaneous border style
    /// ```
    /// let border = TuiGradientblock::new().border_style(BorderStyle::MiscBorder(MiscBorderTypes::Misc2));
    /// ```
    ///
    /// # Example 3: Using a custom border type
    /// ```
    /// let border = TuiGradientblock::new()
    ///     .border_style(BorderStyle::CustomBorderType)
    ///     .top_left('╔')
    ///     .top_right('╗')
    ///     .bottom_left('╚')
    ///     .bottom_right('╝');
    /// ```
    /// Sets the border style of the block.
    ///
    /// This function allows setting a predefined border style or a custom one.
    ///
    /// # Parameters
    /// - `style`: A `BorderStyle` enum variant specifying the desired border style.
    ///
    /// # Behavior
    /// - `BorderStyle::CustomBorderType`: Does not set predefined symbols, allowing manual customization.
    /// - `BorderStyle::MiscBorder(MiscBorderTypes)`: Uses a predefined miscellaneous border style.
    /// - `BorderStyle::Plain`, `BorderStyle::Double`, `BorderStyle::Thick`, `BorderStyle::Rounded`:
    ///   Sets the block's borders to one of these predefined styles.
    ///
    /// # Example
    /// ```
    /// let block = TuiGradientblock::new().border_style(BorderStyle::Double);
    /// ```
    pub fn border_style(mut self, style: BorderStyle) -> Self {
        match &style {
            BorderStyle::CustomBorderType => {
                // Does not set predefined symbols, allowing manual customization.
            }
            BorderStyle::MiscBorder(t) => {
                let miscborder = match t {
                    MiscBorderTypes::Misc1 => MISC1,
                    MiscBorderTypes::Misc2 => MISC2,
                    MiscBorderTypes::Misc3 => MISC3,
                    MiscBorderTypes::Misc4 => MISC4,
                };
                self.border_symbols.top_left = Some(miscborder.top_left);
                self.border_symbols.bottom_left = Some(miscborder.bottom_left);
                self.border_symbols.top_right = Some(miscborder.top_right);
                self.border_symbols.bottom_right = Some(miscborder.bottom_right);
                self.border_symbols.top_horizontal = Some(miscborder.top);
                self.border_symbols.bottom_horizontal = Some(miscborder.bottom);
                self.border_symbols.left_vertical = Some(miscborder.left);
                self.border_symbols.right_vertical = Some(miscborder.right);
                self.border_symbols.bottom_center = Some(miscborder.bottom_center);
                self.border_symbols.top_center = Some(miscborder.top_center);
                self.border_symbols.right_center = Some(miscborder.right_center);
                self.border_symbols.left_center = Some(miscborder.left_center);
            }
            regborder => {
                let reg = match regborder {
                    BorderStyle::Plain => PLAIN,
                    BorderStyle::Double => DOUBLE,
                    BorderStyle::Thick => THICK,
                    BorderStyle::Rounded => ROUNDED,
                    _ => PLAIN, // Fallback to plain border.
                };
                self.border_symbols.top_left = Some(reg.top_left.parse().unwrap());
                self.border_symbols.bottom_left = Some(reg.bottom_left.parse().unwrap());
                self.border_symbols.top_right = Some(reg.top_right.parse().unwrap());
                self.border_symbols.bottom_right = Some(reg.bottom_right.parse().unwrap());
                self.border_symbols.top_horizontal = Some(reg.horizontal_top.parse().unwrap());
                self.border_symbols.bottom_horizontal =
                    Some(reg.horizontal_bottom.parse().unwrap());
                self.border_symbols.left_vertical = Some(reg.vertical_left.parse().unwrap());
                self.border_symbols.right_vertical = Some(reg.vertical_right.parse().unwrap());
                self.border_symbols.bottom_center = Some(reg.horizontal_bottom.parse().unwrap());
                self.border_symbols.top_center = Some(reg.horizontal_top.parse().unwrap());
                self.border_symbols.right_center = Some(reg.vertical_right.parse().unwrap());
                self.border_symbols.left_center = Some(reg.vertical_left.parse().unwrap());
            }
        };
        self.bordertype = style;
        self
    }

    /// Sets the titles that appear at the bottom of the border.
    ///
    /// # Parameters
    /// - `titles`: A vector of tuples where each tuple contains:
    ///   - A `String` representing the title text.
    ///   - A `TitleAlignment` indicating how the title should be aligned (e.g., left, center, right).
    ///   - An optional tuple containing a vector of RGB colors and a gradient factor (f32).
    ///
    /// # Example
    /// ```
    /// let border = Border::new().bottom_titles(vec![
    ///     ("Footer", TitleAlignment::Center, Some((vec![(255, 0, 0), (190, 3, 252)], 0.5))),
    /// ]);
    /// ```
    pub fn bottom_titles(
        mut self,
        titles: Vec<(String, TitleAlignment, Option<(Vec<(u8, u8, u8)>, f32)>)>,
    ) -> Self {
        for (title, align, colors) in titles {
            self.bottom_titles.push((title, align, colors));
        }
        self
    }

    /// Sets the titles that appear at the top of the border.
    ///
    /// # Parameters
    /// - `titles`: A vector of tuples where each tuple contains:
    ///   - A `String` representing the title text.
    ///   - A `TitleAlignment` indicating how the title should be aligned (e.g., left, center, right).
    ///   - An optional tuple containing a vector of RGB colors and a gradient factor (f32).
    ///
    /// # Example 1: Without Gradient
    /// ```
    /// let border = TuiGradientblock::new().top_titles(vec![
    ///     ("Header", TitleAlignment::Left, None),
    /// ]);
    /// ```
    ///
    /// # Example 2: With Gradient
    /// In this example, we use two different colors for the gradient (Red to Blue).
    /// ```
    /// let border = TuiGradientblock::new().top_titles(vec![
    ///     ("Header", TitleAlignment::Center, Some((vec![(255, 0, 0), (0, 0, 255)], 0.5))),
    /// ]);
    /// ```
    pub fn top_titles(
        mut self,
        titles: Vec<(String, TitleAlignment, Option<(Vec<(u8, u8, u8)>, f32)>)>,
    ) -> Self {
        for (title, align, colors) in titles {
            self.top_titles.push((title, align, colors));
        }
        self
    }

    /// Sets the symbol for the top-right corner of the border.
    ///
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used in the top-right corner.
    ///
    /// # Example
    /// ```
    /// let border = TuiGradientblock::new().top_right('#');
    /// ```
    pub fn top_right(mut self, symb: char) -> Self {
        self.border_symbols.top_right = Some(symb);
        self
    }

    /// Sets the symbol for the top-left corner of the border.
    ///
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used in the top-left corner.
    ///
    /// # Example
    /// ```
    /// let border = TuiGradientblock::new().top_left('*');
    /// ```
    pub fn top_left(mut self, symb: char) -> Self {
        self.border_symbols.top_left = Some(symb);
        self
    }

    /// Sets the symbol for the bottom-right corner of the border.
    ///
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used in the bottom-right corner.
    ///
    /// # Example
    /// ```
    /// let border = TuiGradientblock::new().bottom_right('%');
    /// ```
    pub fn bottom_right(mut self, symb: char) -> Self {
        self.border_symbols.bottom_right = Some(symb);
        self
    }

    /// Sets the symbol for the bottom-left corner of the border.
    ///
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used in the bottom-left corner.
    ///
    /// # Example
    /// ```
    /// let border = TuiGradientblock::new().bottom_left('@');
    /// ```
    pub fn bottom_left(mut self, symb: char) -> Self {
        self.border_symbols.bottom_left = Some(symb);
        self
    }

    /// Sets the symbol for the bottom horizontal segment.
    ///
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used for the bottom horizontal border.
    ///
    /// # Example
    /// ```
    /// let border = TuiGradientblockr::new().bottom_horizontal_symbol('-');
    /// ```
    pub fn bottom_horizontal_symbol(mut self, symb: char) -> Self {
        self.border_symbols.bottom_horizontal = Some(symb);
        self
    }

    /// Sets the symbol for the top horizontal border segment.
    ///
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used for the top horizontal border.
    ///
    /// # Example
    /// ```
    /// let border = Border::new().top_horizontal_symbol('=');
    /// ```
    pub fn top_horizontal_symbol(mut self, symb: char) -> Self {
        self.border_symbols.top_horizontal = Some(symb);
        self
    }

    /// Sets the symbol for the right vertical border segment.
    ///
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used for the right vertical border.
    ///
    /// # Example
    /// ```
    /// let border = TuiGradientblock::new().right_vertical_symbol('|');
    /// ```
    pub fn right_vertical_symbol(mut self, symb: char) -> Self {
        self.border_symbols.right_vertical = Some(symb);
        self
    }
    /// Sets the left vertical border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = TuiGradientblock::new().left_vertical_symbol('|');
    /// ```
    pub fn left_vertical_symbol(mut self, symb: char) -> Self {
        self.border_symbols.left_vertical = Some(symb);
        self
    }

    /// Sets the top center border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = TuiGradientblock::new().top_center_symbol('─');
    /// ```
    pub fn top_center_symbol(mut self, symb: char) -> Self {
        self.border_symbols.top_center = Some(symb);
        self
    }

    /// Sets the bottom center border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = TuiGradientblock::new().bottom_center_symbol('═');
    /// ```
    pub fn bottom_center_symbol(mut self, symb: char) -> Self {
        self.border_symbols.bottom_center = Some(symb);
        self
    }

    /// Sets the left center vertical border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = TuiGradientblock::new().left_center_symbol('+');
    /// ```
    pub fn left_center_symbol(mut self, symb: char) -> Self {
        self.border_symbols.left_center = Some(symb);
        self
    }

    /// Sets the right center vertical border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = TuiGradientblock::new().right_center_symbol('+');
    /// ```
    pub fn right_center_symbol(mut self, symb: char) -> Self {
        self.border_symbols.right_center = Some(symb);
        self
    }

    /// Sets the top right horizontal border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = TuiGradientblock::new().top_horizontal_right_symbol('┐');
    /// ```
    pub fn top_horizontal_right_symbol(mut self, symb: char) -> Self {
        self.border_symbols.top_horizontal_right = Some(symb);
        self
    }
    /// Sets the symbol used for the repeated section of the bottom horizontal border (right side).
    ///
    /// # Example
    /// ```
    /// let block = TuiGradientblock::new().bottom_horizontal_right_symbol('*');
    /// ```
    pub fn bottom_horizontal_right_symbol(mut self, symb: char) -> Self {
        self.border_symbols.bottom_horizontal_right = Some(symb);
        self
    }

    /// Sets the symbol for the top horizontal left connector.
    ///
    /// # Example
    /// ```
    /// let block = TuiGradientblock::new().top_horizontal_left_symbol('=');
    /// ```
    pub fn top_horizontal_left_symbol(mut self, symb: char) -> Self {
        self.border_symbols.top_horizontal_left = Some(symb);
        self
    }

    /// Sets the symbol for the bottom horizontal left connector.
    ///
    /// # Example
    /// ```
    /// let block = TuiGradientblock::new().bottom_horizontal_left_symbol('=');
    /// ```
    pub fn bottom_horizontal_left_symbol(mut self, symb: char) -> Self {
        self.border_symbols.bottom_horizontal_left = Some(symb);
        self
    }

    /// Sets the symbol for the top vertical right connector.
    ///
    /// # Example
    /// ```
    /// let block = TuiGradientblock::new().top_vertical_right_symbol('|');
    /// ```
    pub fn top_vertical_right_symbol(mut self, symb: char) -> Self {
        self.border_symbols.top_vertical_right = Some(symb);
        self
    }

    /// Sets the symbol for the bottom vertical right connector.
    ///
    /// # Example
    /// ```
    /// let block = TuiGradientblock::new().bottom_vertical_right_symbol('|');
    /// ```
    pub fn bottom_vertical_right_symbol(mut self, symb: char) -> Self {
        self.border_symbols.bottom_vertical_right = Some(symb);
        self
    }

    /// Sets the symbol for the top vertical left connector.
    ///
    /// # Example
    /// ```
    /// let block = TuiGradientblock::new().top_vertical_left_symbol('|');
    /// ```
    pub fn top_vertical_left_symbol(mut self, symb: char) -> Self {
        self.border_symbols.top_vertical_left = Some(symb);
        self
    }

    /// Sets the symbol for the bottom vertical left connector.
    ///
    /// # Example
    /// ```
    /// let block = TuiGradientblock::new().bottom_vertical_left_symbol('|');
    /// ```
    pub fn bottom_vertical_left_symbol(mut self, symb: char) -> Self {
        self.border_symbols.bottom_vertical_left = Some(symb);
        self
    }

    /// Sets the fill string for the block.
    ///
    /// This string is used to fill the inner area of the block.
    ///
    /// # Example
    /// ```
    /// let block = TuiGradientblock::new().fill_string(String::from("Hello"));
    /// ```
    pub fn fill_string(mut self, string: String) -> Self {
        self.fill.fill_string = Some(string);
        self
    }

    /// Sets the fill gradient for the block.
    ///
    /// The gradient is defined as a list of RGB colors and a factor to control the blending effect.
    ///
    /// # Example
    /// ```
    /// let colors = vec![(255, 0, 0), (0, 255, 0), (0, 0, 255)];
    /// let block = TuiGradientblock::new().fill_gradient(colors, 0.5);
    /// ```
    pub fn fill_gradient(mut self, colors: Vec<(u8, u8, u8)>, factor: f32) -> Self {
        self.fill.gradient = Some((colors, factor));
        self
    }

    /// Sets the border line segments based on the area and border symbols.
    ///
    /// This method configures the border segments (top, bottom, left, right) and any possible splits
    /// within the block. It calculates and sets the text for each border line segment using the provided
    /// border symbols and the block's area. The function supports setting up horizontal and vertical
    /// lines, as well as handling special cases where the border is split into smaller sections.
    ///
    /// **Important:**
    /// - This function should be called **last** after all other block properties are set, as it depends
    ///   on the final values of the area and border symbols.
    ///
    /// # Behavior
    /// - The function calculates the appropriate border segments, including top, bottom, left, and right borders.
    /// - It uses the provided `border_symbols` to determine the characters used for the borders.
    /// - If the `split_segments` attribute contains any of `TOP`, `BOTTOM`, `LEFT`, or `RIGHT`, the respective
    ///   segments are split into smaller parts, and the relevant segments are marked for rendering.
    /// - In the case of the `ALL` split, all border lines are broken into smaller segments and set to be rendered.
    /// - If no split is needed (`NONE`), the function disables the rendering of split border lines.
    ///
    /// # Parameters
    /// - This method takes no parameters and modifies the internal state of the struct it's called on.
    ///
    /// # Returns
    /// - A modified instance of the struct (self), with the border segments set according to the configurations.
    pub fn set_lines(mut self) -> Self {
        let border_symbols = &self.border_symbols;
        let area = &self.area;
        let top_horizontal = border_symbols
            .top_horizontal
            .unwrap_or(PLAIN.horizontal_top.parse().unwrap());
        let left_vertical = border_symbols
            .left_vertical
            .unwrap_or(PLAIN.vertical_left.parse().unwrap());
        let bottom_horizontal = border_symbols
            .bottom_horizontal
            .unwrap_or(PLAIN.horizontal_top.parse().unwrap());
        let right_vertical = border_symbols
            .right_vertical
            .unwrap_or(PLAIN.vertical_right.parse().unwrap());
        let top_center_char = border_symbols.top_center.unwrap_or(top_horizontal);
        let bottom_center_char = border_symbols.bottom_center.unwrap_or(bottom_horizontal);
        let left_center_char = border_symbols.left_center.unwrap_or(left_vertical);
        let right_center_char = border_symbols.right_center.unwrap_or(right_vertical);
        let top_right = border_symbols
            .top_right
            .unwrap_or(PLAIN.top_right.parse().unwrap());
        let top_left: char = border_symbols
            .top_left
            .unwrap_or(PLAIN.top_left.parse().unwrap());
        let bottom_right = border_symbols
            .bottom_right
            .unwrap_or(PLAIN.bottom_right.parse().unwrap());
        let bottom_left = border_symbols
            .bottom_left
            .unwrap_or(PLAIN.bottom_left.parse().unwrap());
        let top_horizontal_right = border_symbols
            .top_horizontal_right
            .unwrap_or(top_horizontal);
        let bottom_horizontal_right = border_symbols
            .bottom_horizontal_right
            .unwrap_or(bottom_horizontal);
        let top_horizontal_left = border_symbols.top_horizontal_left.unwrap_or(top_horizontal);
        let bottom_horizontal_left = border_symbols
            .bottom_horizontal_left
            .unwrap_or(bottom_horizontal);
        let top_vertical_right = border_symbols.top_vertical_right.unwrap_or(right_vertical);
        let bottom_vertical_right = border_symbols
            .bottom_vertical_right
            .unwrap_or(right_vertical);
        let top_vertical_left = border_symbols.top_vertical_left.unwrap_or(left_vertical);
        let bottom_vertical_left = border_symbols.bottom_vertical_left.unwrap_or(left_vertical);
        let top_horizontal_right_ln = format!(
            "{}{}{}",
            top_center_char,
            top_horizontal_right
                .to_string()
                .repeat((area.width as usize - 1) / 2 - 1),
            top_right,
        );
        let top_horizontal_left_ln = format!(
            "{}{}{}",
            top_left,
            top_horizontal_left
                .to_string()
                .repeat((area.width as usize - 1) / 2 - 1),
            top_center_char
        );
        let bottom_horizontal_left_ln = format!(
            "{}{}{}",
            bottom_left,
            bottom_horizontal_left
                .to_string()
                .repeat((area.width as usize - 1) / 2 - 1),
            bottom_center_char,
        );
        let bottom_horizontal_right_ln = format!(
            "{}{}{}",
            bottom_center_char,
            bottom_horizontal_right
                .to_string()
                .repeat((area.width as usize - 1) / 2 - 1),
            bottom_right,
        );
        let top_vertical_right_ln = format!(
            "{}{}{}",
            top_right,
            top_vertical_right
                .to_string()
                .repeat(((area.height as usize + 1) / 2).saturating_sub(2)),
            right_center_char
        );
        let top_vertical_left_ln = format!(
            "{}{}{}",
            top_left,
            top_vertical_left
                .to_string()
                .repeat(((area.height as usize + 1) / 2).saturating_sub(2)),
            left_center_char,
        );
        let bottom_vertical_left_ln = format!(
            "{}{}{}",
            left_center_char,
            bottom_vertical_left
                .to_string()
                .repeat((area.height as usize / 2).saturating_sub(1)),
            bottom_left,
        );
        let bottom_vertical_right_ln = format!(
            "{}{}{}",
            right_center_char,
            bottom_vertical_right
                .to_string()
                .repeat((area.height as usize / 2).saturating_sub(1)),
            bottom_right,
        );
        let top_ln = format!(
            "{}{}{}{}{}",
            top_left,
            top_horizontal_left
                .to_string()
                .repeat(((area.width as usize + 1) / 2).saturating_sub(2)),
            top_center_char,
            top_horizontal_right
                .to_string()
                .repeat((area.width as usize / 2).saturating_sub(1)),
            top_right,
        );
        let bottom_ln = format!(
            "{}{}{}{}{}",
            bottom_left,
            bottom_horizontal_left
                .to_string()
                .repeat(((area.width as usize + 1) / 2).saturating_sub(1)),
            bottom_center_char,
            bottom_horizontal_right
                .to_string()
                .repeat((area.width as usize / 2).saturating_sub(1)),
            bottom_right,
        );
        let right_ln = format!(
            "{}{}{}{}{}",
            top_right,
            top_vertical_right
                .to_string()
                .repeat(((area.height as usize + 1) / 2).saturating_sub(2)),
            right_center_char,
            bottom_vertical_right
                .to_string()
                .repeat((area.height as usize / 2).saturating_sub(1)),
            bottom_right,
        );
        let left_ln = format!(
            "{}{}{}{}{}",
            top_left,
            top_vertical_left
                .to_string()
                .repeat(((area.height as usize + 1) / 2).saturating_sub(2)),
            left_center_char,
            bottom_vertical_left
                .to_string()
                .repeat((area.height as usize / 2).saturating_sub(1)),
            bottom_left
        );
        self.border_segments.top_ln.should_be_rendered = true;
        self.border_segments.top_ln.segment_text = top_ln;
        self.border_segments.bottom_ln.should_be_rendered = true;
        self.border_segments.bottom_ln.segment_text = bottom_ln;
        self.border_segments.left_ln.should_be_rendered = true;
        self.border_segments.left_ln.segment_text = left_ln;
        self.border_segments.right_ln.should_be_rendered = true;
        self.border_segments.right_ln.segment_text = right_ln;
        if self.split_segments.contains(SplitBorderSegments::TOP) {
            self.border_segments.top_ln.should_be_rendered = false;
            self.border_segments
                .top_horizontal_left_ln
                .should_be_rendered = true;
            self.border_segments.top_horizontal_left_ln.segment_text = top_horizontal_left_ln;
            self.border_segments
                .top_horizontal_right_ln
                .should_be_rendered = true;
            self.border_segments.top_horizontal_right_ln.segment_text = top_horizontal_right_ln;
        }

        if self.split_segments.contains(SplitBorderSegments::BOTTOM) {
            self.border_segments.bottom_ln.should_be_rendered = false;
            self.border_segments.bottom_horizontal_left_ln.segment_text = bottom_horizontal_left_ln;
            self.border_segments
                .bottom_horizontal_left_ln
                .should_be_rendered = true;

            self.border_segments.bottom_horizontal_right_ln.segment_text =
                bottom_horizontal_right_ln;
            self.border_segments
                .bottom_horizontal_right_ln
                .should_be_rendered = true;
        }
        if self.split_segments.contains(SplitBorderSegments::LEFT) {
            self.border_segments.left_ln.should_be_rendered = false;
            self.border_segments.top_vertical_left_ln.should_be_rendered = true;
            self.border_segments.top_vertical_left_ln.segment_text = top_vertical_left_ln;
            self.border_segments
                .bottom_vertical_left_ln
                .should_be_rendered = true;
            self.border_segments.bottom_vertical_left_ln.segment_text = bottom_vertical_left_ln;
        }
        if self.split_segments.contains(SplitBorderSegments::RIGHT) {
            self.border_segments.right_ln.should_be_rendered = false;
            self.border_segments
                .top_vertical_right_ln
                .should_be_rendered = true;
            self.border_segments.top_vertical_right_ln.segment_text = top_vertical_right_ln;
            self.border_segments
                .bottom_vertical_right_ln
                .should_be_rendered = true;
            self.border_segments.bottom_vertical_right_ln.segment_text = bottom_vertical_right_ln;
        }
        if self.split_segments == SplitBorderSegments::ALL {
            self.border_segments.top_ln.should_be_rendered = false;
            self.border_segments.bottom_ln.should_be_rendered = false;
            self.border_segments.left_ln.should_be_rendered = false;
            self.border_segments.right_ln.should_be_rendered = false;
            self.border_segments
                .top_horizontal_left_ln
                .should_be_rendered = true;
            self.border_segments
                .top_horizontal_right_ln
                .should_be_rendered = true;
            self.border_segments
                .bottom_horizontal_left_ln
                .should_be_rendered = true;
            self.border_segments
                .bottom_horizontal_right_ln
                .should_be_rendered = true;
            self.border_segments.top_vertical_left_ln.should_be_rendered = true;
            self.border_segments
                .top_vertical_right_ln
                .should_be_rendered = true;
            self.border_segments
                .bottom_vertical_left_ln
                .should_be_rendered = true;
            self.border_segments
                .bottom_vertical_right_ln
                .should_be_rendered = true;
        }

        if self.split_segments == SplitBorderSegments::NONE {
            self.border_segments
                .top_horizontal_left_ln
                .should_be_rendered = false;
            self.border_segments
                .top_horizontal_right_ln
                .should_be_rendered = false;
            self.border_segments
                .bottom_horizontal_left_ln
                .should_be_rendered = false;
            self.border_segments
                .bottom_horizontal_right_ln
                .should_be_rendered = false;
        }
        self
    }
    /// Renders the border segments of the block.
    ///
    /// This private function iterates through all defined border segments and renders only those
    /// that are marked to be rendered (`should_be_rendered == true`).
    ///
    /// It uses a list of tuples where each tuple contains a border segment and its corresponding
    /// render function. If a segment is marked as `should_be_rendered`, the associated render function
    /// is invoked to render that segment.
    ///
    /// # Parameters
    /// - `buf`: A mutable reference to the [`Buffer`] where the border segments will be drawn.
    ///
    /// # Behavior
    /// - The function checks each border segment and invokes the corresponding render function if the
    ///   segment is set to be rendered.
    ///
    /// # Note
    /// This function is called internally by the program when rendering the custom block, and isn't meant
    /// to be directly called by users.
    fn render_block(&self, buf: &mut Buffer) {
        let segments: &[(&BorderSegment, fn(&Self, &mut Buffer))] = &[
            (&self.border_segments.top_ln, Self::render_top_ln),
            (&self.border_segments.bottom_ln, Self::render_bottom_ln),
            (&self.border_segments.left_ln, Self::render_left_ln),
            (&self.border_segments.right_ln, Self::render_right_ln),
            (
                &self.border_segments.top_horizontal_left_ln,
                Self::render_top_horizontal_left_ln,
            ),
            (
                &self.border_segments.top_horizontal_right_ln,
                Self::render_top_horizontal_right_ln,
            ),
            (
                &self.border_segments.bottom_vertical_left_ln,
                Self::render_bottom_vertical_left_ln,
            ),
            (
                &self.border_segments.bottom_vertical_right_ln,
                Self::render_bottom_vertical_right_ln,
            ),
            (
                &self.border_segments.bottom_horizontal_left_ln,
                Self::render_bottom_horizontal_left_ln,
            ),
            (
                &self.border_segments.bottom_horizontal_right_ln,
                Self::render_bottom_horizontal_right_ln,
            ),
            (
                &self.border_segments.top_vertical_left_ln,
                Self::render_top_vertical_left_ln,
            ),
            (
                &self.border_segments.top_vertical_right_ln,
                Self::render_top_vertical_right_ln,
            ),
        ];

        for (segment, render_fn) in segments {
            if segment.should_be_rendered {
                render_fn(self, buf);
            }
        }
    }

    /// Renders the top horizontal line of the border with an optional gradient.
    ///
    /// This function renders the top border line. If the `top_ln` segment should use a gradient,
    /// it applies the gradient to the segment text. Otherwise, it renders the text as-is.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +-----+
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +-----+
    /// ```
    fn render_top_ln(&self, buf: &mut Buffer) {
        let top_ln = self.border_segments.top_ln.clone();
        let used_top_ln = match top_ln.should_use_gradient {
            true => &Line::from(Self::create_gradient_text(
                &top_ln.segment_text,
                top_ln.gradient.clone().unwrap().0,
                top_ln.gradient.unwrap().1,
            )),
            false => &Line::from(top_ln.segment_text),
        };
        buf.set_line(top_ln.x, top_ln.y, used_top_ln, self.area.width);
    }

    /// Renders the left vertical line of the border with an optional gradient.
    ///
    /// This function renders the left border line. If the `left_ln` segment should use a gradient,
    /// it applies the gradient to the segment text. Otherwise, it renders the text as-is.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +-----+
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +-----+
    /// ```
    fn render_left_ln(&self, buf: &mut Buffer) {
        let left_ln = self.border_segments.left_ln.clone();
        let used_left_ln: &Vec<Span> = match left_ln.should_use_gradient {
            true => &Self::create_gradient_text(
                &left_ln.segment_text,
                left_ln.gradient.clone().unwrap().0,
                left_ln.gradient.unwrap().1,
            ),
            false => &left_ln
                .segment_text
                .chars()
                .map(|i| Span::raw(i.to_string()))
                .collect(),
        };
        for (i, ln) in used_left_ln.iter().enumerate() {
            buf.set_span(left_ln.x, left_ln.y + i as u16, ln, 1);
        }
    }

    /// Renders the bottom horizontal line of the border with an optional gradient.
    ///
    /// This function renders the bottom border line. If the `bottom_ln` segment should use a gradient,
    /// it applies the gradient to the segment text. Otherwise, it renders the text as-is.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +-----+
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +-----+
    /// ```
    fn render_bottom_ln(&self, buf: &mut Buffer) {
        let bottom_ln = self.border_segments.bottom_ln.clone();
        let used_bottom_ln = match bottom_ln.should_use_gradient {
            true => &Line::from(Self::create_gradient_text(
                &bottom_ln.segment_text,
                bottom_ln.gradient.clone().unwrap().0,
                bottom_ln.gradient.unwrap().1,
            )),
            false => &Line::from(bottom_ln.segment_text),
        };
        buf.set_line(bottom_ln.x, bottom_ln.y, used_bottom_ln, self.area.width);
    }

    /// Renders the right vertical line of the border with an optional gradient.
    ///
    /// This function renders the right border line. If the `right_ln` segment should use a gradient,
    /// it applies the gradient to the segment text. Otherwise, it renders the text as-is.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +-----+
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +-----+
    /// ```
    fn render_right_ln(&self, buf: &mut Buffer) {
        let mut right_ln = self.border_segments.right_ln.clone();

        let used_right_ln: Vec<Span> = if right_ln.should_use_gradient {
            Self::create_gradient_text(
                &right_ln.clone().segment_text,
                right_ln.gradient.clone().unwrap().0,
                right_ln.gradient.as_mut().unwrap().1,
            )
        } else {
            right_ln
                .clone()
                .segment_text
                .chars()
                .map(|i| Span::raw(i.to_string()))
                .collect()
        };
        for (i, span) in used_right_ln.iter().enumerate() {
            buf.set_span(right_ln.x, right_ln.y + i as u16, span, 1);
        }
    }

    /// Renders the left side of the top horizontal line of the border with an optional gradient.
    ///
    /// This function renders the left part of the top horizontal line. If the `top_horizontal_left_ln` segment
    /// should use a gradient, it applies the gradient to the segment text. Otherwise, it renders the text as-is.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// ****~~~
    /// ~     ~
    /// ~     ~
    /// ~     ~
    /// ~     ~
    /// ~     ~
    /// ~~~~~~~
    /// ```
    fn render_top_horizontal_left_ln(&self, buf: &mut Buffer) {
        let top_horizontal_left_ln = self.border_segments.top_horizontal_left_ln.clone();
        let used_top_horizontal_left_ln = match top_horizontal_left_ln.should_use_gradient {
            true => &Line::from(Self::create_gradient_text(
                &top_horizontal_left_ln.segment_text,
                top_horizontal_left_ln.gradient.clone().unwrap().0,
                top_horizontal_left_ln.gradient.unwrap().1,
            )),
            false => &Line::from(top_horizontal_left_ln.segment_text),
        };
        buf.set_line(
            top_horizontal_left_ln.x,
            top_horizontal_left_ln.y,
            used_top_horizontal_left_ln,
            self.area.width,
        );
    }

    /// Renders the right side of the top horizontal line of the border with an optional gradient.
    ///
    /// This function renders the right part of the top horizontal line. If the `top_horizontal_right_ln` segment
    /// should use a gradient, it applies the gradient to the segment text. Otherwise, it renders the text as-is.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +---  +
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +-----+
    /// ```
    fn render_top_horizontal_right_ln(&self, buf: &mut Buffer) {
        let top_horizontal_right_ln = self.border_segments.top_horizontal_right_ln.clone();
        let used_top_horizontal_right_ln = match top_horizontal_right_ln.should_use_gradient {
            true => &Line::from(Self::create_gradient_text(
                &top_horizontal_right_ln.segment_text,
                top_horizontal_right_ln.gradient.clone().unwrap().0,
                top_horizontal_right_ln.gradient.unwrap().1,
            )),
            false => &Line::from(top_horizontal_right_ln.segment_text),
        };
        buf.set_line(
            top_horizontal_right_ln.x,
            top_horizontal_right_ln.y,
            used_top_horizontal_right_ln,
            self.area.width,
        );
    }

    /// Renders the left side of the bottom horizontal line of the border with an optional gradient.
    ///
    /// This function renders the left part of the bottom horizontal line. If the segment should use a gradient,
    /// it applies the gradient to the segment text. Otherwise, it renders the text as-is.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +-----+
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +  ---+
    /// ```
    fn render_bottom_horizontal_left_ln(&self, buf: &mut Buffer) {
        let bottom_horizontal_left_ln = self.border_segments.bottom_horizontal_left_ln.clone();
        let used_bottom_horizontal_left_ln = match bottom_horizontal_left_ln.should_use_gradient {
            true => &Line::from(Self::create_gradient_text(
                &bottom_horizontal_left_ln.segment_text,
                bottom_horizontal_left_ln.gradient.clone().unwrap().0,
                bottom_horizontal_left_ln.gradient.unwrap().1,
            )),
            false => &Line::from(bottom_horizontal_left_ln.segment_text),
        };
        buf.set_line(
            bottom_horizontal_left_ln.x,
            bottom_horizontal_left_ln.y,
            used_bottom_horizontal_left_ln,
            self.area.width,
        );
    }

    /// Renders the right side of the bottom horizontal line of the border with an optional gradient.
    ///
    /// This function renders the right part of the bottom horizontal line. If the segment should use a gradient,
    /// it applies the gradient to the segment text. Otherwise, it renders the text as-is.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +-----+
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +---  +
    /// ```
    fn render_bottom_horizontal_right_ln(&self, buf: &mut Buffer) {
        let bottom_horizontal_right_ln = self.border_segments.bottom_horizontal_right_ln.clone();
        let used_bottom_horizontal_right_ln = match bottom_horizontal_right_ln.should_use_gradient {
            true => &Line::from(Self::create_gradient_text(
                &bottom_horizontal_right_ln.segment_text,
                bottom_horizontal_right_ln.gradient.clone().unwrap().0,
                bottom_horizontal_right_ln.gradient.unwrap().1,
            )),
            false => &Line::from(bottom_horizontal_right_ln.segment_text),
        };
        buf.set_line(
            bottom_horizontal_right_ln.x,
            bottom_horizontal_right_ln.y,
            used_bottom_horizontal_right_ln,
            self.area.width,
        );
    }

    /// Renders the top side of the left vertical line of the border with an optional gradient.
    ///
    /// This function renders the left part of the top vertical line. If the segment should use a gradient,
    /// it applies the gradient to the segment text. Otherwise, it renders the text as-is.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +-----+
    ///       |
    ///       |
    /// +     +
    /// |     |
    /// |     |
    /// +-----+
    /// ```
    fn render_top_vertical_left_ln(&self, buf: &mut Buffer) {
        let mut top_vertical_left_ln = self.border_segments.top_vertical_left_ln.clone();

        let used_top_vertical_left_ln: Vec<Span> = if top_vertical_left_ln.should_use_gradient {
            Self::create_gradient_text(
                &top_vertical_left_ln.clone().segment_text,
                top_vertical_left_ln.gradient.clone().unwrap().0,
                top_vertical_left_ln.gradient.as_mut().unwrap().1,
            )
        } else {
            top_vertical_left_ln
                .clone()
                .segment_text
                .chars()
                .map(|i| Span::raw(i.to_string()))
                .collect()
        };
        for (i, span) in used_top_vertical_left_ln.iter().enumerate() {
            buf.set_span(
                top_vertical_left_ln.x,
                top_vertical_left_ln.y + i as u16,
                span,
                1,
            );
        }
    }

    /// Renders the top side of the right vertical line of the border with an optional gradient.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// This function renders the top-right vertical line of the border. If the segment associated with
    /// this line should use a gradient, it applies the gradient to the segment text using the `create_gradient_text`
    /// function. Otherwise, the function renders the segment text as-is.
    ///
    /// # Example:
    /// ```rust
    /// let widget = TuiGradientblock::new();
    /// let mut buffer = Buffer::new();
    /// widget.render_top_vertical_right_ln(&mut buffer);
    /// ```
    ///
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +-----+
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +-----+
    /// ```
    fn render_top_vertical_right_ln(&self, buf: &mut Buffer) {
        let mut top_vertical_right_ln = self.border_segments.top_vertical_right_ln.clone();

        let used_top_vertical_right_ln: Vec<Span> = if top_vertical_right_ln.should_use_gradient {
            Self::create_gradient_text(
                &top_vertical_right_ln.clone().segment_text,
                top_vertical_right_ln.gradient.clone().unwrap().0,
                top_vertical_right_ln.gradient.as_mut().unwrap().1,
            )
        } else {
            top_vertical_right_ln
                .clone()
                .segment_text
                .chars()
                .map(|i| Span::raw(i.to_string()))
                .collect()
        };
        for (i, span) in used_top_vertical_right_ln.iter().enumerate() {
            buf.set_span(
                top_vertical_right_ln.x,
                top_vertical_right_ln.y + i as u16,
                span,
                1,
            );
        }
    }

    /// Renders the bottom vertical right line of the border with an optional gradient.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// This function renders the bottom-right vertical line of the border. If the segment associated with
    /// this line should use a gradient, it applies the gradient to the segment text using the `create_gradient_text`
    /// function. Otherwise, the function renders the segment text as-is.
    ///
    /// # Example:
    /// ```rust
    /// let widget = TuiGradientblock::new();
    /// let mut buffer = Buffer::new();
    /// widget.render_bottom_vertical_right_ln(&mut buffer);
    /// ```
    ///
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +-----+
    /// |     |
    /// |     |
    /// +     +
    /// |
    /// |
    /// +-----+
    /// ```
    fn render_bottom_vertical_right_ln(&self, buf: &mut Buffer) {
        let mut bottom_vertical_right_ln = self.border_segments.bottom_vertical_right_ln.clone();
        let used_bottom_vertical_right_ln: Vec<Span> =
            if bottom_vertical_right_ln.should_use_gradient {
                Self::create_gradient_text(
                    &bottom_vertical_right_ln.clone().segment_text,
                    bottom_vertical_right_ln.gradient.clone().unwrap().0,
                    bottom_vertical_right_ln.gradient.as_mut().unwrap().1,
                )
            } else {
                bottom_vertical_right_ln
                    .clone()
                    .segment_text
                    .chars()
                    .map(|i| Span::raw(i.to_string()))
                    .collect()
            };
        for (i, span) in used_bottom_vertical_right_ln.iter().enumerate() {
            buf.set_span(
                bottom_vertical_right_ln.x,
                bottom_vertical_right_ln.y + i as u16,
                span,
                1,
            );
        }
    }

    /// Renders the bottom vertical left segment of the border with an optional gradient.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// This function renders the bottom-left vertical segment of the border. If the segment associated with
    /// this line should use a gradient, it applies the gradient to the segment text using the `create_gradient_text`
    /// function. Otherwise, the function renders the segment text as-is.
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +-----+
    /// |     |
    /// |     |
    /// +     +
    /// |
    /// |
    /// +-----+
    /// ```
    fn render_bottom_vertical_left_ln(&self, buf: &mut Buffer) {
        let mut bottom_vertical_left_ln = self.border_segments.bottom_vertical_left_ln.clone();
        let used_bottom_vertical_left_ln: Vec<Span> = if bottom_vertical_left_ln.should_use_gradient
        {
            Self::create_gradient_text(
                &bottom_vertical_left_ln.clone().segment_text,
                bottom_vertical_left_ln.gradient.clone().unwrap().0,
                bottom_vertical_left_ln.gradient.as_mut().unwrap().1,
            )
        } else {
            bottom_vertical_left_ln
                .clone()
                .segment_text
                .chars()
                .map(|i| Span::raw(i.to_string()))
                .collect()
        };
        for (i, span) in used_bottom_vertical_left_ln.iter().enumerate() {
            buf.set_span(
                bottom_vertical_left_ln.x,
                bottom_vertical_left_ln.y + i as u16,
                span,
                1,
            );
        }
    }

    /// Renders the bottom titles for the `TuiGradientblock` widget, with optional gradient support.
    ///
    /// # Parameters:
    /// - `area`: A `Rc<Rect>` that defines the area where the bottom titles will be rendered.
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// This function renders each title in the `bottom_titles` collection with a specified alignment:
    /// - **Left-aligned**: The title is rendered at the left edge of the bottom row of the area.
    /// - **Right-aligned**: The title is rendered at the right edge of the bottom row of the area.
    /// - **Centered**: The title is centered within the bottom row of the area.
    ///
    /// If a gradient is specified for a title, it is applied by calling `create_gradient_text`
    /// with the gradient’s start and end colors. The resulting text is then rendered with the specified alignment.
    fn render_bottom_titles(&self, area: Rc<Rect>, buf: &mut Buffer) {
        for title in &self.bottom_titles {
            let ln = match &title.2 {
                Some(s) => &Line::from(Self::create_gradient_text(&title.0, s.0.clone(), s.1)),
                None => &Line::from(title.0.clone()),
            };
            match title.1 {
                TitleAlignment::LeftAligned => {
                    buf.set_line(area.left() + 1, area.bottom() - 1, ln, title.0.len() as u16);
                }
                TitleAlignment::RightAligned => {
                    buf.set_line(
                        (area.right() - 1).saturating_sub(title.0.len() as u16),
                        area.bottom() - 1,
                        ln,
                        title.0.len() as u16,
                    );
                }
                TitleAlignment::Centered => {
                    buf.set_line(
                        (area.right() / 2).saturating_sub(title.0.len() as u16 / 2),
                        area.bottom() - 1,
                        ln,
                        title.0.len() as u16,
                    );
                }
            }
        }
    }

    /// Renders the top titles for the `TuiGradientblock` widget, with optional gradient support.
    ///
    /// # Parameters:
    /// - `area`: A `Rc<Rect>` that defines the area where the top titles will be rendered.
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// This function renders each title in the `top_titles` collection with a specified alignment:
    /// - **Left-aligned**: The title is rendered at the left edge of the area.
    /// - **Right-aligned**: The title is rendered at the right edge of the area.
    /// - **Centered**: The title is centered within the area.
    ///
    /// If a gradient is specified for a title, it is applied by calling `create_gradient_text`
    /// with the gradient’s start and end colors. The resulting text is then rendered with the specified alignment.
    ///
    /// # Example:
    /// ```rust
    /// let widget = TuiGradientblock::new();
    /// let area = Rc::new(Rect::new(0, 0, 20, 5));
    /// let mut buffer = Buffer::new();
    /// widget.render_top_titles(area, &mut buffer);
    /// ```
    fn render_top_titles(&self, area: Rc<Rect>, buf: &mut Buffer) {
        for title in &self.top_titles {
            let ln = match &title.2 {
                Some(s) => &Line::from(Self::create_gradient_text(&title.0, s.0.clone(), s.1)),
                None => &Line::from(title.0.clone()),
            };
            match title.1 {
                TitleAlignment::LeftAligned => {
                    buf.set_line(area.left() + 1, area.top(), ln, title.0.len() as u16);
                }
                TitleAlignment::RightAligned => {
                    buf.set_line(
                        (area.right() - 1).saturating_sub(title.0.len() as u16),
                        area.top(),
                        ln,
                        title.0.len() as u16,
                    );
                }
                TitleAlignment::Centered => {
                    buf.set_line(
                        (area.right() / 2).saturating_sub(title.0.len() as u16 / 2),
                        area.top(),
                        ln,
                        title.0.len() as u16,
                    );
                }
            }
        }
    }

    /// Renders the fill for the `TuiGradientblock` widget, including optional gradient rendering.
    ///
    /// # Parameters:
    /// - `area`: A `Rc<Rect>` that defines the area in which to render the fill.
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// This function performs the following:
    /// - If a `fill_string` is provided (via `self.fill.fill_string`), it generates a repeated string
    ///   pattern to fill the given `area`.
    /// - If no gradient is specified (`self.fill.gradient` is `None`), it renders the repeated string
    ///   in the specified area using a simple block with borders.
    /// - If a gradient is provided, the function applies the gradient to the fill and renders the
    ///   resulting text using a centered block with borders.
    ///
    /// The function uses `Paragraph::new` to render the text and applies wrapping and border options.
    /// It also calls `create_gradient_text` to generate the text with the applied gradient if a gradient
    /// is specified.
    ///
    /// # Example:
    /// ```rust
    /// let widget = TuiGradientblock::new();
    /// let area = Rc::new(Rect::new(0, 0, 10, 5));
    /// let mut buffer = Buffer::new();
    /// widget.render_fill(area, &mut buffer);
    /// ```
    fn render_fill(&self, area: Rc<Rect>, buf: &mut Buffer) {
        let mut string = self.fill.fill_string.clone().unwrap();
        string.push(' ');
        if self.fill.gradient.is_none() {
            Paragraph::new(string.repeat(area.width as usize * string.len()).to_owned())
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL))
                .render(*area, buf);
        } else {
            // Get a reference to the gradient and its color values
            let gradient = self.fill.gradient.as_ref().unwrap();
            let gradient_text = Self::create_gradient_text(
                &string
                    .repeat(area.width as usize * string.len())
                    .to_string(),
                gradient.0.clone(),
                gradient.1,
            );

            let text = Line::from(gradient_text);
            Paragraph::new(text)
                .centered()
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL))
                .render(*area, buf);
        }
    }

    /// Renders the `TuiGradientblock` widget, including optional fill and custom block rendering,
    /// along with top and bottom titles.
    ///
    /// # Parameters:
    /// - `area`: A reference to a `Rect` that specifies the area to render the widget in.
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// This function:
    /// - Checks if there is a fill string and calls `render_fill` if present.
    /// - Renders the custom block using `render_custom_block`.
    /// - Renders the top titles using `render_top_titles`.
    /// - Renders the bottom titles using `render_bottom_titles`.
    pub fn main(&self, area: &Rect, buf: &mut Buffer) {
        let area_rc = Rc::new(*area);
        if self.fill.fill_string.is_some() {
            Self::render_fill(self, Rc::clone(&area_rc), buf);
        }
        self.render_block(buf);
        Self::render_top_titles(self, Rc::clone(&area_rc), buf);
        Self::render_bottom_titles(self, Rc::clone(&area_rc), buf);
    }
}
impl Widget for TuiGradientblock {
    /// Renders the `TuiGradientblock` widget using the `main` function.
    ///
    /// This is part of the `Widget` trait implementation. The `render` function takes an
    /// `area` and a mutable reference to the `Buffer`, and delegates rendering to the `main` function.
    ///
    /// # Parameters:
    /// - `area`: A `Rect` that defines the area for rendering the widget.
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// # Example:
    /// ```
    /// let widget = TuiGradientblock::new();
    /// let area = Rect::new(0, 0, 10, 10);
    /// let mut buffer = Buffer::new();
    /// widget.render(area, &mut buffer);
    /// ```
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.main(&area, buf);
    }
}
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
    /// +=====+
    /// |     |
    /// +     +
    /// |     |
    /// ╘═════╛
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
/// A set of symbols that define the appearance of a border.  
/// Most symbols are optional, allowing for customization of corners, edges, and intersections.
struct BorderSymbols {
    /// The character used for the top-left corner of the border (e.g., '┌' for a box).  
    /// If not set, a default may be used.
    top_left: Option<char>,

    /// The character used for the top-right corner of the border (e.g., '┐' for a box).
    top_right: Option<char>,

    /// The character used for the bottom-left corner of the border (e.g., '└' for a box).
    bottom_left: Option<char>,

    /// The character used for the bottom-right corner of the border (e.g., '┘' for a box).
    bottom_right: Option<char>,

    /// The character placed at the center of the top border.  
    /// Defaults to `top_horizontal` if not specified.
    top_center: Option<char>,

    /// The character placed at the center of the bottom border.  
    /// Defaults to `bottom_horizontal` if not specified.
    bottom_center: Option<char>,

    /// The character placed at the center of the left border.  
    /// Defaults to `left_vertical` if not specified.
    left_center: Option<char>,

    /// The character placed at the center of the right border.  
    /// Defaults to `right_vertical` if not specified.
    right_center: Option<char>,

    /// The character used for the horizontal segments of the top border (e.g., '─' for a solid line).  
    /// Defaults to '─' if not set.
    top_horizontal: Option<char>,

    /// The character used for the vertical segments of the left border (e.g., '│' for a solid line).  
    /// Defaults to '│' if not set.
    left_vertical: Option<char>,

    /// The character used for the horizontal segments of the bottom border (e.g., '─' for a solid line).  
    /// Defaults to '─' if not set.
    bottom_horizontal: Option<char>,

    /// The character used for the vertical segments of the right border (e.g., '│' for a solid line).  
    /// Defaults to '│' if not set.
    right_vertical: Option<char>,

    /// The character repeated for the right side of the top border.  
    /// Defaults to `top_horizontal` if not set.
    top_horizontal_right: Option<char>,

    /// The character repeated for the right side of the bottom border.  
    /// Defaults to `bottom_horizontal` if not set.
    bottom_horizontal_right: Option<char>,

    /// The character repeated for the left side of the top border.  
    /// Defaults to `top_horizontal` if not set.
    top_horizontal_left: Option<char>,

    /// The character repeated for the left side of the bottom border.  
    /// Defaults to `bottom_horizontal` if not set.
    bottom_horizontal_left: Option<char>,

    /// The character repeated for the top section of the right border.  
    /// Defaults to `right_vertical` if not set.
    top_vertical_right: Option<char>,

    /// The character repeated for the bottom section of the right border.  
    /// Defaults to `right_vertical` if not set.
    bottom_vertical_right: Option<char>,

    /// The character repeated for the top section of the left border.  
    /// Defaults to `left_vertical` if not set.
    top_vertical_left: Option<char>,

    /// The character repeated for the bottom section of the left border.  
    /// Defaults to `left_vertical` if not set.
    bottom_vertical_left: Option<char>,
}

#[derive(Debug, Clone)]
/// controls the optional fill with an optional gradient
struct Fill {
    fill_string: Option<String>,
    gradient: Option<(Vec<(u8, u8, u8)>, f32)>,
}
#[derive(Debug, Clone)]
struct BorderSegment {
    /// The text representation of this border segment.  
    /// - For a top border, this might be `"┌──────┐"`.  
    /// - For a right border, this might be `"┐││││┘"` (each character is rendered on a separate line from top to bottom, as newlines cannot be rendered directly).
    segment_text: String,

    /// An optional gradient applied to this border segment.  
    /// The gradient consists of a vector of RGB color values and a scaling factor.
    gradient: Option<(Vec<(u8, u8, u8)>, f32)>,

    /// Determines whether this segment should be rendered with a gradient.  
    /// If `false`, the gradient (if present) will be ignored.
    should_use_gradient: bool,

    /// Determines whether the segment should be rendered at all.  
    /// If `false`, this segment will not be displayed.
    should_be_rendered: bool,

    /// The X-coordinate of the segment’s position.
    x: u16,

    /// The Y-coordinate of the segment’s position.
    y: u16,
}
impl BorderSegment {
    /// Creates a new, empty border segment at the specified position.  
    ///
    /// The segment starts with no text, no gradient, and is not set to be rendered by default.
    ///
    /// # Parameters
    /// - `x`: The X-coordinate of the segment’s position.
    /// - `y`: The Y-coordinate of the segment’s position.
    ///
    /// # Returns
    /// A `BorderSegment` instance with default values.
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            segment_text: String::new(),
            gradient: None,
            should_use_gradient: false,
            should_be_rendered: false,
            x,
            y,
        }
    }
}

#[derive(Debug, Clone)]

/// A collection of border segments representing different parts of a bordered structure.  
///
/// This struct holds individual `BorderSegment` instances for each section of the border,  
/// ensuring flexibility in rendering complex layouts.
struct BorderSegments {
    /// The left portion of the top horizontal border.
    top_horizontal_left_ln: BorderSegment,
    /// The right portion of the top horizontal border.
    top_horizontal_right_ln: BorderSegment,
    /// The left portion of the bottom horizontal border.
    bottom_horizontal_left_ln: BorderSegment,
    /// The right portion of the bottom horizontal border.
    bottom_horizontal_right_ln: BorderSegment,
    /// The upper portion of the left vertical border.
    top_vertical_left_ln: BorderSegment,
    /// The upper portion of the right vertical border.
    top_vertical_right_ln: BorderSegment,
    /// The lower portion of the left vertical border.
    bottom_vertical_left_ln: BorderSegment,
    /// The lower portion of the right vertical border.
    bottom_vertical_right_ln: BorderSegment,
    /// The full top border segment.
    top_ln: BorderSegment,
    /// The full bottom border segment.
    bottom_ln: BorderSegment,
    /// The full left border segment.
    left_ln: BorderSegment,
    /// The full right border segment.
    right_ln: BorderSegment,
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
            top_horizontal_left_ln: BorderSegment::new(area.left(), area.top()),
            top_horizontal_right_ln: BorderSegment::new(area.right() / 2, area.top()),
            bottom_horizontal_left_ln: BorderSegment::new(area.left(), area.bottom() - 1),
            bottom_horizontal_right_ln: BorderSegment::new(area.right() / 2, area.bottom() - 1),
            top_vertical_left_ln: BorderSegment::new(area.left(), area.top()),
            top_vertical_right_ln: BorderSegment::new(area.right() - 1, area.top()),
            bottom_vertical_left_ln: BorderSegment::new(
                area.left(),
                ((area.height as usize + 1) / 2) as u16,
            ),
            bottom_vertical_right_ln: BorderSegment::new(
                area.right() - 1,
                ((area.height as usize + 1) / 2) as u16,
            ),
            top_ln: BorderSegment::new(area.left(), area.top()),
            bottom_ln: BorderSegment::new(area.left(), area.bottom() - 1),
            left_ln: BorderSegment::new(area.left(), area.top()),
            right_ln: BorderSegment::new(area.right() - 1, area.top()),
        }
    }
}

bitflags! {
    /// Represents individual border segments that can be split or modified.
    #[derive(PartialEq, Clone)]
    pub struct SplitBorderSegments: u8 {
        /// No border segments are split.
        const NONE   = 0b0000;
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
#[derive(Clone)]
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
#[derive(Clone)]
/// Defines the alignment options for a title within a bordered area.
pub enum TitleAlignment {
    /// Centers the title within the border.
    Centered,
    /// Aligns the title to the left within the border.
    LeftAligned,
    /// Aligns the title to the right within the border.
    RightAligned,
}
