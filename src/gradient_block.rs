use crate::{
    predefined::consts::ERROR_MESSAGE,
    types::{
        enums::{BorderStyle, GradientSegments, TitleDirection},
        structs::{
            BorderSegment, BorderSegments, BorderSymbols, Fill,
            Gradient, SplitBorderSegments, Title,
        },
    },
    *,
};
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
/// let gradient_block = Gradientblock {
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

#[derive(Clone)]
pub struct GradientBlock {
    pub bordertype: BorderStyle,
    pub fill: Fill,
    pub top_titles: Vec<Title>,
    pub bottom_titles: Vec<Title>,
    pub border_symbols: BorderSymbols,
    pub border_segments: BorderSegments,
    pub split_segments: SplitBorderSegments,
    pub area: Rect,
}

impl GradientBlock {
    pub fn new(
        area: &Rect,
        split_segments: SplitBorderSegments,
    ) -> Self {
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
        let t_pow_factor = t.powf(factor);
        let one_minus_t = 1.0 - t_pow_factor;
        (
            (one_minus_t * start.0 as f32
                + t_pow_factor * end.0 as f32) as u8,
            (one_minus_t * start.1 as f32
                + t_pow_factor * end.1 as f32) as u8,
            (one_minus_t * start.2 as f32
                + t_pow_factor * end.2 as f32) as u8,
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
    /// let gradient_text = create_gradient_text(&text, &colors, &factor);
    /// ```
    /// In the above example, the `gradient_text` will be a vector of `Span`s with the text "Hello, World!"
    /// styled with a gradient transitioning from red to green to blue.
    /// # Note
    /// - The `interpolate_color` function is used internally to calculate the intermediate colors based on the
    ///   position of the character relative to the total width of the text.
    pub fn create_gradient_text(
        text: &str,
        colors: &[(u8, u8, u8)],
        factor: &f32,
    ) -> Vec<Span<'static>> {
        if colors.len() < 2 {
            ratatui::restore();
            panic!("{}", ERROR_MESSAGE);
        }
        let total_width: usize =
            text.chars().map(|c| c.width().unwrap_or(1)).sum();
        let num_colors = colors.len();
        let mut gradient_text = Vec::with_capacity(text.len());
        let mut accumulated_width = 0.0;
        for c in text.chars() {
            let char_width = c.width().unwrap_or(1) as f32;
            let relative_pos = accumulated_width / total_width as f32;
            let pos = relative_pos * (num_colors - 1) as f32;
            let section_index = pos.floor() as usize;
            let t = pos - section_index as f32;
            let next_color_index =
                (section_index + 1).min(num_colors - 1);
            let color = Self::interpolate_color(
                colors[section_index],
                colors[next_color_index],
                t,
                *factor,
            );
            gradient_text.push(Span::styled(
                c.to_string(),
                Style::default()
                    .fg(Color::Rgb(color.0, color.1, color.2)),
            ));
            accumulated_width += char_width;
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
    /// # Notes
    /// - If a gradient should be a solid color, provide the same RGB value twice.
    /// - Gradients must have at least two different colors to transition properly.
    /// # Example 1: Applying a gradient to the top border
    /// ```
    /// let border = Gradientblock::new().set_gradients(vec![
    ///     (GradientSegments::Top, vec![(255, 0, 0), (0, 0, 255)], 0.5),
    /// ]);
    /// ```
    /// # Example 2: Applying a solid color to the right border
    /// ```
    /// let border = Gradientblock::new().set_gradients(vec![
    ///     (GradientSegments::Right, vec![(50, 50, 50), (50, 50, 50)], 1.0),
    /// ]);
    /// ```
    pub fn set_gradients(
        self,
        gradientlist: Vec<(GradientSegments, Gradient)>,
    ) -> Self {
        self.gradients(gradientlist)
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
    /// let border = Gradientblock::new().border_style(BorderStyle::Double);
    /// ```
    ///
    /// # Example 2: Using a miscellaneous border style
    /// ```
    /// let border = Gradientblock::new().border_style(BorderStyle::MiscBorder(MiscBorderTypes::Misc2));
    /// ```
    ///
    /// # Example 3: Using a custom border type
    /// ```
    /// let border = Gradientblock::new()
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
    /// let block = Gradientblock::new().border_style(BorderStyle::Double);
    /// ```
    pub fn border_style(self, style: BorderStyle) -> Self {
        Self::set_border_style(self, style)
    }

    /// Sets the titles that appear at the bottom of the border.
    ///
    /// # Parameters
    /// - `titles`: A vector of tuples where each tuple contains:
    ///   - A `String` representing the title text.
    ///   - A `Alignment` indicating how the title should be aligned (e.g., left, center, right).
    ///   - An optional tuple containing a vector of RGB colors and a gradient factor (f32).
    ///
    /// # Example
    /// ```
    /// let border = Border::new().bottom_titles(vec![
    ///     ("Footer", Alignment::Center, Some((vec![(255, 0, 0), (190, 3, 252)], 0.5))),
    /// ]);
    /// ```
    pub fn bottom_titles(mut self, titles: Vec<Title>) -> Self {
        for title in titles {
            self.bottom_titles.push(title);
        }
        self
    }

    /// Sets the titles that appear at the top of the border.
    ///
    /// # Parameters
    /// - `titles`: A vector of tuples where each tuple contains:
    ///   - A `String` representing the title text.
    ///   - A `Alignment` indicating how the title should be aligned (e.g., left, center, right).
    ///   - An optional tuple containing a vector of RGB colors and a gradient factor (f32).
    ///
    /// # Example 1: Without Gradient
    /// ```
    /// let border = Gradientblock::new().top_titles(vec![
    ///     ("Header", Alignment::Left, None),
    /// ]);
    /// ```
    ///
    /// # Example 2: With Gradient
    /// In this example, we use two different colors for the gradient (Red to Blue).
    /// ```
    /// let border = Gradientblock::new().top_titles(vec![
    ///     ("Header", Alignment::Center, Some((vec![(255, 0, 0), (0, 0, 255)], 0.5))),
    /// ]);
    /// ```
    pub fn top_titles(mut self, titles: Vec<Title>) -> Self {
        for title in titles {
            self.top_titles.push(title);
        }
        self
    }

    /// Sets the symbol for the top-right corner of the border.
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used in the top-right corner.
    ///
    /// # Example
    /// ```
    /// let border = Gradientblock::new().top_right('#');
    /// ```
    pub const fn top_right(mut self, symb: char) -> Self {
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
    /// let border = Gradientblock::new().top_left('*');
    /// ```
    pub const fn top_left(mut self, symb: char) -> Self {
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
    /// let border = Gradientblock::new().bottom_right('%');
    /// ```
    pub const fn bottom_right(mut self, symb: char) -> Self {
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
    /// let border = Gradientblock::new().bottom_left('@');
    /// ```
    pub const fn bottom_left(mut self, symb: char) -> Self {
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
    /// let border = Gradientblockr::new().bottom_horizontal_symbol('-');
    /// ```
    pub const fn bottom_horizontal_symbol(
        mut self,
        symb: char,
    ) -> Self {
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
    pub const fn top_horizontal_symbol(mut self, symb: char) -> Self {
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
    /// let border = Gradientblock::new().right_vertical_symbol('|');
    /// ```
    pub const fn right_vertical_symbol(mut self, symb: char) -> Self {
        self.border_symbols.right_vertical = Some(symb);
        self
    }
    /// Sets the left vertical border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = Gradientblock::new().left_vertical_symbol('|');
    /// ```
    pub const fn left_vertical_symbol(mut self, symb: char) -> Self {
        self.border_symbols.left_vertical = Some(symb);
        self
    }

    /// Sets the top center border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = Gradientblock::new().top_center_symbol('─');
    /// ```
    pub const fn top_center_symbol(mut self, symb: char) -> Self {
        self.border_symbols.top_center = Some(symb);
        self
    }

    /// Sets the bottom center border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = Gradientblock::new().bottom_center_symbol('═');
    /// ```
    pub const fn bottom_center_symbol(mut self, symb: char) -> Self {
        self.border_symbols.bottom_center = Some(symb);
        self
    }

    /// Sets the left center vertical border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = Gradientblock::new().left_center_symbol('+');
    /// ```
    pub const fn left_center_symbol(mut self, symb: char) -> Self {
        self.border_symbols.left_center = Some(symb);
        self
    }

    /// Sets the right center vertical border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = Gradientblock::new().right_center_symbol('+');
    /// ```
    pub const fn right_center_symbol(mut self, symb: char) -> Self {
        self.border_symbols.right_center = Some(symb);
        self
    }

    /// Sets the top right horizontal border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = Gradientblock::new().top_horizontal_right_symbol('┐');
    /// ```
    pub fn top_horizontal_right_symbol(mut self, symb: char) -> Self {
        self.border_symbols.top_horizontal_right = Some(symb);
        self
    }
    /// Sets the symbol used for the repeated section of the bottom horizontal border (right side).
    ///
    /// # Example
    /// ```
    /// let block = Gradientblock::new().bottom_horizontal_right_symbol('*');
    /// ```
    pub const fn bottom_horizontal_right_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.bottom_horizontal_right = Some(symb);
        self
    }

    /// Sets the symbol for the top horizontal left connector.
    ///
    /// # Example
    /// ```
    /// let block = Gradientblock::new().top_horizontal_left_symbol('=');
    /// ```
    pub const fn top_horizontal_left_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.top_horizontal_left = Some(symb);
        self
    }

    /// Sets the symbol for the bottom horizontal left connector.
    ///
    /// # Example
    /// ```
    /// let block = Gradientblock::new().bottom_horizontal_left_symbol('=');
    /// ```
    pub const fn bottom_horizontal_left_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.bottom_horizontal_left = Some(symb);
        self
    }

    /// Sets the symbol for the top vertical right connector.
    ///
    /// # Example
    /// ```
    /// let block = Gradientblock::new().top_vertical_right_symbol('|');
    /// ```
    pub const fn top_vertical_right_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.top_vertical_right = Some(symb);
        self
    }

    /// Sets the symbol for the bottom vertical right connector.
    ///
    /// # Example
    /// ```
    /// let block = Gradientblock::new().bottom_vertical_right_symbol('|');
    /// ```
    pub const fn bottom_vertical_right_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.bottom_vertical_right = Some(symb);
        self
    }

    /// Sets the symbol for the top vertical left connector.
    ///
    /// # Example
    /// ```
    /// let block = Gradientblock::new().top_vertical_left_symbol('|');
    /// ```
    pub const fn top_vertical_left_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.top_vertical_left = Some(symb);
        self
    }

    /// Sets the symbol for the bottom vertical left connector.
    ///
    /// # Example
    /// ```
    /// let block = Gradientblock::new().bottom_vertical_left_symbol('|');
    /// ```
    pub const fn bottom_vertical_left_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.bottom_vertical_left = Some(symb);
        self
    }

    /// Sets the fill string for the block.
    ///
    /// This string is used to fill the inner area of the block.
    ///
    /// # Example
    /// ```
    /// let block = Gradientblock::new().fill_string(String::from("Hello"));
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
    /// let block = Gradientblock::new().fill_gradient(Gradient {
    /// gradient_list: colors,
    /// factor: 0.5,
    /// });
    /// ```
    pub fn fill_gradient(mut self, gradient: Gradient) -> Self {
        self.fill.gradient = Some(gradient);
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
    pub fn set_lines(self) -> Self {
        Self::set_lns(self)
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
            (&self.border_segments.left_ln, Self::render_left_ln),
            (&self.border_segments.right_ln, Self::render_right_ln),
            (&self.border_segments.top_ln, Self::render_top_ln),
            (&self.border_segments.bottom_ln, Self::render_bottom_ln),
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
    /// +     +
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +-----+
    /// ```
    fn render_top_ln(&self, buf: &mut Buffer) {
        handle_line_logic!(self.border_segments.top_ln, buf);
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
    ///       |
    ///       |
    ///       +
    ///       |
    ///       |
    /// +-----+
    /// ```
    fn render_left_ln(&self, buf: &mut Buffer) {
        handle_line_logic!(self.border_segments.left_ln, buf);
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
    /// +--+--+
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +     +
    /// ```
    fn render_bottom_ln(&self, buf: &mut Buffer) {
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
    fn render_right_ln(&self, buf: &mut Buffer) {
        handle_line_logic!(&self.border_segments.right_ln, buf);
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
    /// +  +--+
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +--+--+
    /// ```
    fn render_top_horizontal_left_ln(&self, buf: &mut Buffer) {
        handle_line_logic!(
            &self.border_segments.top_horizontal_left_ln,
            buf
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
        handle_line_logic!(
            &self.border_segments.top_horizontal_right_ln,
            buf
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
        handle_line_logic!(
            &self.border_segments.bottom_horizontal_left_ln,
            buf
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
        handle_line_logic!(
            self.border_segments.bottom_horizontal_right_ln,
            buf
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
        let ln = &self.border_segments.top_vertical_left_ln;
        let spanvec = check_gradient!(ln);
        set_vertical_line!(ln, spanvec, buf);
    }

    /// Renders the top side of the right vertical line of the border with an optional gradient.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// This function renders the top-right vertical line of the border. If the segment associated with
    /// this line should use a gradient, it applies the gradient to the segment text using the `create_gradient_text`
    /// function. Otherwise, the function renders the segment text as-is.
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +-----+
    /// |     
    /// |     
    /// +     +
    /// |     |
    /// |     |
    /// +-----+
    /// ```
    fn render_top_vertical_right_ln(&self, buf: &mut Buffer) {
        let ln = &self.border_segments.top_vertical_right_ln;
        let spanvec = check_gradient!(ln);
        set_vertical_line!(ln, spanvec, buf);
    }

    /// Renders the bottom vertical right line of the border with an optional gradient.
    ///
    /// # Parameters:
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// This function renders the bottom-right vertical line of the border. If the segment associated with
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
    fn render_bottom_vertical_right_ln(&self, buf: &mut Buffer) {
        let ln = &self.border_segments.bottom_vertical_right_ln;
        let spanvec = check_gradient!(ln);
        set_vertical_line!(ln, spanvec, buf);
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
    ///       |
    ///       |
    /// +-----+
    /// ```
    fn render_bottom_vertical_left_ln(&self, buf: &mut Buffer) {
        handle_line_logic!(
            &self.border_segments.bottom_vertical_left_ln,
            buf
        );
    }

    /// Renders the bottom titles for the `Gradientblock` widget, with optional gradient support.
    ///
    /// # Parameters:
    /// - `area`: A `Rc<Rect>` that defines the area where the bottom titles will be rendered.
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    ///
    /// This function renders each title in the `bottom_titles` collection with a specified alignment:
    /// - **Left-**: The title is rendered at the left edge of the bottom row of the area.
    /// - **Right**: The title is rendered at the right edge of the bottom row of the area.
    /// - **Centered**: The title is centered within the bottom row of the area.
    ///
    /// If a gradient is specified for a title, it is applied by calling `create_gradient_text`
    /// with the gradient’s start and end colors. The resulting text is then rendered with the specified alignment.
    fn render_bottom_titles(&self, area: Rc<Rect>, buf: &mut Buffer) {
        handle_title_logic!(
            &self.bottom_titles,
            TitleDirection::Bottom,
            *area,
            buf
        );
    }

    /// Renders the top titles for the `Gradientblock` widget, with optional gradient support.
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
    fn render_top_titles(&self, area: Rc<Rect>, buf: &mut Buffer) {
        handle_title_logic!(
            &self.top_titles,
            TitleDirection::Top,
            *area,
            buf
        );
    }

    /// Renders the fill for the `Gradientblock` widget, including optional gradient rendering.
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
    fn render_fill(&self, area: Rc<Rect>, buf: &mut Buffer) {
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
impl Widget for GradientBlock {
    /// Renders the `Gradientblock` widget using the `main` function.
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
    /// let widget = Gradientblock::new();
    /// let area = Rect::new(0, 0, 10, 10);
    /// let mut buffer = Buffer::new();
    /// widget.render(area, &mut buffer);
    /// ```
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.main(&area, buf);
    }
}
