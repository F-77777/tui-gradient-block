use crate::types::G;
pub use crate::{
    buffer, check_gradient, enums, generate_gradient_text,
    get_aligned_position, handle_fill, handle_line_logic,
    handle_title_logic,
    prelude::{self, Widget},
    set_line, set_vertical_line, structs, text,
    text::Line,
    widgets::{
        self,
        block::{Position, Title as T},
    },
};
use std::rc::Rc;
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
///   - A `Alignment` indicating the alignment of the title.
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
/// let gradient_block = GradientBlock {
///     bordertype: BorderStyle::Solid,
///     fill: Fill::SolidColor((255, 0, 0)),
///     top_titles: vec![("Top Title".to_owned(), Alignment::Center, None)],
///     bottom_titles: vec![("Bottom Title".to_owned(), Alignment::Right, None)],
///     border_symbols: BorderSymbols::Default,
///     border_segments: BorderSegments::Full,
///     split_segments: SplitBorderSegments::None,
///     area: Rect::new(0, 0, 10, 5),
/// };
/// ```

pub struct GradientBlock {
    pub border_type: enums::BorderStyle,
    pub fill: structs::Fill,
    pub top_titles: Vec<widgets::block::Title<'static>>,
    pub bottom_titles: Vec<widgets::block::Title<'static>>,
    pub border_symbols: structs::BorderSymbols,
    pub border_segments: structs::BorderSegments,
    pub area: prelude::Rect,
}

impl GradientBlock {
    pub fn new(area: &prelude::Rect) -> Self {
        Self {
            border_type: enums::BorderStyle::Plain,
            fill: structs::Fill::new(),
            top_titles: Vec::new(),
            bottom_titles: Vec::new(),
            border_symbols: structs::BorderSymbols::new(),
            border_segments: structs::BorderSegments::new(area),
            area: *area,
        }
    }

    /// Creates a gradient effect on a given text by interpolating between a list of colors
    /// based on the position of each character in the string.
    ///
    /// # Parameters
    /// - `text`: A reference to the input `String` for which the gradient effect will be applied.
    /// - `gradient` A reference to the Gradient struct
    ///
    /// # Returns
    /// A `Vec<Span<'static>>` containing `Span` elements, where each `Span` represents a styled portion
    /// of the input text with the corresponding color from the gradient.
    /// # Example:
    /// ```rust
    /// let text = "Hello, World!".to_string();
    /// let colors = vec![(255, 0, 0), (0, 255, 0), (0, 0, 255)];
    /// let factor = 1.0;
    /// let gradient_text = create_gradient_text(&text, &colors, &factor);
    /// ```
    /// In the above example, the `gradient_text` will be a vector of `Span`s with the text "Hello, World!"
    /// styled with a gradient transitioning from red to green to blue.
    /// # Note
    /// - The `interpolate_color` function is used internally to calculate the intermediate colors based on the
    ///   position of the character relative to the total width of the text.
    pub fn create_gradient_text(
        text: &str,
        gradient: &G,
    ) -> Vec<prelude::Span<'static>> {
        let mut gradient_text = Vec::with_capacity(text.len());
        for (c, color) in text.chars().zip(gradient.colors(text.len())) {
            gradient_text.push(prelude::Span::styled(
                String::from(c),
                prelude::Style::default().fg(prelude::Color::Rgb(
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8,
                )),
            ));
        }
        gradient_text
    }
    /// Sets the border line segments based on the area and border symbols.
    /// This method configures the border segments (top, bottom, left, right) and any possible splits
    /// **Important:**
    /// - This function should be called **last** after all other block properties are set, as it depends
    ///   on the final values of the area and border symbols.
    ///
    /// # Behavior
    /// - The function calculates the appropriate border segments, including top, bottom, left, and right borders.
    /// - It uses the provided `border_symbols` to determine the characters used for the borders.
    /// # Returns
    /// - A modified instance of the struct (self), with the border segments set according to the configurations.
    pub fn set_lines(self) -> Self {
        Self::set_lns(self)
    }
    /// Renders the border segments of the block.
    ///
    /// This private function renders the top, left, right and bottom lines
    fn render_block(&self, buf: &mut buffer::Buffer) {
        Self::render_left_ln(&self, buf);
        Self::render_right_ln(&self, buf);
        Self::render_top_ln(&self, buf);
        Self::render_bottom_ln(&self, buf);
    }

    /// Renders the top horizontal line of the border with an optional gradient.
    ///
    /// This function renders the top border line. If the `top_ln` segment should use a gradient,
    /// it applies the gradient to the segment text. Otherwise, it renders the text as-is.
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +     +
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +-----+
    /// ```
    fn render_top_ln(&self, buf: &mut buffer::Buffer) {
        handle_line_logic!(self.border_segments.top_ln, buf);
    }

    /// Renders the left vertical line of the border with an optional gradient.
    ///
    /// This function renders the left border line. If the `left_ln` segment should use a gradient,
    /// it applies the gradient to the segment text. Otherwise, it renders the text as-is.
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +-----+
    ///       |
    ///       |
    ///       +
    ///       |
    ///       |
    /// +-----+
    /// ```
    fn render_left_ln(&self, buf: &mut buffer::Buffer) {
        handle_line_logic!(self.border_segments.left_ln, buf);
    }

    /// Renders the bottom horizontal line of the border with an optional gradient.
    ///
    /// This function renders the bottom border line. If the `bottom_ln` segment should use a gradient,:
    /// it applies the gradient to the segment text. Otherwise, it renders the text as-is.
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +--+--+
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +     +
    /// ````
    fn render_bottom_ln(&self, buf: &mut buffer::Buffer) {
        handle_line_logic!(&self.border_segments.bottom_ln, buf);
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
    /// +--+--+
    /// |     
    /// |     
    /// +     
    /// |     
    /// |     
    /// +--+--+
    /// ```
    fn render_right_ln(&self, buf: &mut buffer::Buffer) {
        handle_line_logic!(&self.border_segments.right_ln, buf);
    }

    /// Renders the itles for the widget, with an optional gradient
    fn render_titles(
        &self,
        area: Rc<prelude::Rect>,
        buf: &mut buffer::Buffer,
    ) {
        handle_title_logic!(&self.top_titles, *area, buf);
    }

    /// Renders the fill for the widget, including optional gradient rendering.
    fn render_fill(
        &self,
        area: Rc<prelude::Rect>,
        buf: &mut buffer::Buffer,
    ) {
        handle_fill!(
            self.fill,
            self.fill.fill_string.as_deref().unwrap_or(""),
            *area,
            buf,
            (area.height * area.width) as usize
        );
    }

    /// Renders the `Gradientblock` widget, including optional fill and custom block rendering,
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
    pub fn main(
        &self,
        area: &prelude::Rect,
        buf: &mut buffer::Buffer,
    ) {
        let area_rc = Rc::new(*area);
        if self.fill.fill_string.is_some() {
            Self::render_fill(self, Rc::clone(&area_rc), buf);
        }
        self.render_block(buf);
        Self::render_top_titles(self, Rc::clone(&area_rc), buf);
        Self::render_bottom_titles(self, Rc::clone(&area_rc), buf);
    }
}
impl widgets::Widget for GradientBlock {
    /// Renders the `Gradientblock` widget using the `main` function.
    ///
    /// This is part of the `Widget` trait implementation. The `render` function takes an
    /// `area` and a mutable reference to the `Buffer`, and delegates rendering to the `main` function.
    ///
    /// # Parameters:
    /// - `area`: A `Rect` that defines the area for rendering the widget.
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    fn render(self, area: prelude::Rect, buf: &mut buffer::Buffer) {
        self.main(&area, buf);
    }
}
