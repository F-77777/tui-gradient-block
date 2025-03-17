pub use crate::{
    border::*,
    border_styles::*,
    buffer, check_gradient, enums,
    generate_gradient_text, get_aligned_position,
    get_parsed_symbol, get_symbol, handle_fill,
    handle_line_logic, handle_title_logic,
    prelude::{self, Widget},
    set_line, set_vertical_line,
    structs::{
        self, border_segment, border_symbols,
        gradient,
    },
    text::{self, Line},
    types::G,
    widgets::{
        self,
        block::{self, title::Position},
        Block, Borders, Paragraph,
    },
};
use std::rc::Rc;
/// A struct that represents a customizable block with gradient text, borders, and other visual elements.
///
/// This struct allows you to create and manage blocks that have a gradient color effect for text,
/// customizable borders, and areas with specific alignments and fill styles.
pub struct GradientBlock<'a> {
    pub border_type: enums::BorderStyle,
    pub fill: Line<'a>,
    pub titles: Vec<(Line<'a>, Position)>,
    pub border_symbols:
        border_symbols::BorderSymbolsSet,
    pub border_segments:
        border_segment::BorderSegments,
    pub area: prelude::Rect,
}

impl<'a> GradientBlock<'a> {
    pub fn new(area: &prelude::Rect) -> Self {
        Self {
            border_type: enums::BorderStyle::Plain,
            fill: Line::raw(""),
            titles: Vec::new(),
            border_symbols: border_symbols::BorderSymbolsSet::new(),
            border_segments: border_segment::BorderSegments::new(
                area,
            ),
            area: *area,
        }
    }

    pub fn create_gradient_text<
        L: Into<Line<'a>>,
    >(
        text: L,
        gradient: &G,
    ) -> Vec<prelude::Span<'_>> {
        let text = text.into();
        let mut gradient_text =
            Vec::with_capacity(text.width());
        for (c, color) in text.spans().zip(
            gradient.colors(text.width()).iter(),
        ) {
            gradient_text.push(c.fg(
                prelude::Color::Rgb(
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8,
                ),
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
    /// Renders the border segments of the block.
    ///
    /// This private function renders the block's borders
    fn render_block(
        &self,
        buf: &mut buffer::Buffer,
    ) {
        if self.left_ln.should_be_rendered {
            Self::render_left_ln(self, buf);
        }
        if self.right_ln.should_be_rendered {
            Self::render_right_ln(self, buf);
        }
        if self.top_ln.should_be_rendered {
            Self::render_top_ln(self, buf);
        }
        if self.bottom_ln.should_be_rendered {
            Self::render_bottom_ln(self, buf);
        }
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
    fn render_top_ln(
        &self,
        buf: &mut buffer::Buffer,
    ) {
        handle_line_logic!(
            self.border_segments.top_ln,
            buf
        );
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
    ///       |
    ///       +
    ///       |
    ///       |
    /// +-----+
    /// ```
    fn render_left_ln(
        &self,
        buf: &mut buffer::Buffer,
    ) {
        handle_line_logic!(
            self.border_segments.left_ln,
            buf
        );
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
    fn render_bottom_ln(
        &self,
        buf: &mut buffer::Buffer,
    ) {
        handle_line_logic!(
            &self.border_segments.bottom_ln,
            buf
        );
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
    fn render_right_ln(
        &self,
        buf: &mut buffer::Buffer,
    ) {
        handle_line_logic!(
            &self.border_segments.right_ln,
            buf
        );
    }

    /// Renders the itles for the widget, with an optional gradient
    fn render_titles(
        &self,
        area: Rc<prelude::Rect>,
        buf: &mut buffer::Buffer,
    ) {
        handle_title_logic!(
            &self.titles,
            *area,
            buf
        );
    }

    /// Renders the fill for the widget, including optional gradient rendering.
    fn render_fill(
        &self,
        area: Rc<prelude::Rect>,
        buf: &mut buffer::Buffer,
    ) {
        handle_fill!(self.fill, *area, buf);
    }

    /// Renders the `Gradientblock` widget, including optional fill and custom block rendering,
    /// along with titles.
    pub fn main(
        &self,
        area: &prelude::Rect,
        buf: &mut buffer::Buffer,
    ) {
        let area_rc = Rc::new(*area);
        if self.fill.is_some() {
            Self::render_fill(
                self,
                Rc::clone(&area_rc),
                buf,
            );
        }
        self.render_block(buf);
        Self::render_titles(
            self,
            Rc::clone(&area_rc),
            buf,
        );
    }
    pub fn build(mut self) -> Self {
        let border_symbols = &self.border_symbols;
        let area = &self.area;
        let top_horizontal = get_symbol!(
            border_symbols.top_horizontal,
            get_parsed_symbol!(
                PLAIN.horizontal_top
            )
        );
        let left_vertical = get_symbol!(
            border_symbols.left_vertical,
            get_parsed_symbol!(
                PLAIN.vertical_left
            )
        );
        let bottom_horizontal = get_symbol!(
            border_symbols.bottom_horizontal,
            get_parsed_symbol!(
                PLAIN.horizontal_top
            )
        );
        let right_vertical = get_symbol!(
            border_symbols.right_vertical,
            get_parsed_symbol!(
                PLAIN.vertical_right
            )
        );

        let top_center_char = get_symbol!(
            border_symbols.top_center,
            top_horizontal
        );
        let bottom_center_char = get_symbol!(
            border_symbols.bottom_center,
            bottom_horizontal
        );
        let left_center_char = get_symbol!(
            border_symbols.left_center,
            left_vertical
        );
        let right_center_char = get_symbol!(
            border_symbols.right_center,
            right_vertical
        );

        let top_right = get_symbol!(
            border_symbols.top_right,
            get_parsed_symbol!(PLAIN.top_right)
        );
        let top_left = get_symbol!(
            border_symbols.top_left,
            get_parsed_symbol!(PLAIN.top_left)
        );
        let bottom_right = get_symbol!(
            border_symbols.bottom_right,
            get_parsed_symbol!(
                PLAIN.bottom_right
            )
        );
        let bottom_left = get_symbol!(
            border_symbols.bottom_left,
            get_parsed_symbol!(PLAIN.bottom_left)
        );

        let top_horizontal_right = get_symbol!(
            border_symbols.top_horizontal_right,
            top_horizontal
        );
        let bottom_horizontal_right = get_symbol!(
            border_symbols
                .bottom_horizontal_right,
            bottom_horizontal
        );
        let top_horizontal_left = get_symbol!(
            border_symbols.top_horizontal_left,
            top_horizontal
        );
        let bottom_horizontal_left = get_symbol!(
            border_symbols.bottom_horizontal_left,
            bottom_horizontal
        );

        let top_vertical_right = get_symbol!(
            border_symbols.top_vertical_right,
            right_vertical
        );
        let bottom_vertical_right = get_symbol!(
            border_symbols.bottom_vertical_right,
            right_vertical
        );
        let top_vertical_left = get_symbol!(
            border_symbols.top_vertical_left,
            left_vertical
        );
        let bottom_vertical_left = get_symbol!(
            border_symbols.bottom_vertical_left,
            left_vertical
        );

        let top_hor_left_rep =
            ((area.width.saturating_sub(1)
                as usize)
                / 2)
            .saturating_sub(1);
        let top_hor_right_rep =
            ((area.width.saturating_sub(1)
                as usize)
                / 2)
            .saturating_sub(2);
        let bottom_hor_left_rep =
            ((area.width as usize + 1) / 2)
                .saturating_sub(1);
        let bottom_hor_right_rep =
            (area.width as usize / 2)
                .saturating_sub(2);

        let top_vert_left_rep =
            ((area.height as usize + 1) / 2)
                .saturating_sub(2);
        let bottom_vert_left_rep =
            (area.height as usize / 2)
                .saturating_sub(1);
        let top_ln = create_segment!(
            top_left,
            top_horizontal_left,
            top_hor_left_rep,
            top_center_char,
            top_horizontal_right,
            top_hor_right_rep,
            top_right,
            self.border_segments.top_ln.gradient
        );
        let bottom_ln = create_segment!(
            bottom_left,
            bottom_horizontal_left,
            bottom_hor_left_rep,
            bottom_center_char,
            bottom_horizontal_right,
            bottom_hor_right_rep,
            bottom_right,
            self.border_segments
                .bottom_ln
                .gradient
        );
        let left_ln = create_segment!(
            top_left,
            top_vertical_left,
            top_vert_left_rep,
            left_center_char,
            bottom_vertical_left,
            bottom_vert_left_rep,
            bottom_left,
            self.border_segments.left_ln.gradient
        );
        let right_ln = create_segment!(
            top_right,
            top_vertical_right,
            top_vert_left_rep,
            right_center_char,
            bottom_vertical_right,
            bottom_vert_left_rep,
            bottom_right,
            self.border_segments
                .right_ln
                .gradient
        );
        self.border_segments
            .top_ln
            .segment_text = top_ln;
        self.border_segments
            .right_ln
            .segment_text = right_ln;
        self.border_segments
            .left_ln
            .segment_text = left_ln;
        self.border_segments
            .bottom_ln
            .segment_text = bottom_ln;
        self
    }
    pub fn set_border_style(
        mut self,
        style: enums::BorderStyle,
    ) -> Self {
        match style {
            enums::BorderStyle::CustomBorderType(t) => {
                self.border_symbols =
                crate::structs::border_symbols::BorderSymbolsSet {
                    top_left: Some(t.top_left),
                    bottom_left: Some(t.bottom_left),
                    top_right: Some(t.top_right),
                    bottom_right: Some(t.bottom_right),
                    top_horizontal: Some(t.top),
                    bottom_horizontal: Some(t.bottom),
                    left_vertical: Some(t.left),
                    right_vertical: Some(t.right),
                    bottom_center: Some(t.bottom_center),
                    top_center: Some(t.top_center),
                    right_center: Some(t.right_center),
                    left_center: Some(t.left_center),

                    top_horizontal_right: Some(t.top),
                    bottom_horizontal_right: Some(t.bottom),
                    top_horizontal_left: Some(t.top),
                    bottom_horizontal_left: Some(t.bottom),

                    top_vertical_right: Some(t.right),
                    bottom_vertical_right: Some(t.right),
                    top_vertical_left: Some(t.top_left),
                    bottom_vertical_left: Some(t.left),
                };
            }
            enums::BorderStyle::CustomBorderTypeFull(t) => {
                self.border_symbols = t;
            }
            enums::BorderStyle::MiscBorder(t) => {
                let miscborder = match t {
                    enums::MiscBorderTypes::Misc1 => MISC1,
                    enums::MiscBorderTypes::Misc2 => MISC2,
                    enums::MiscBorderTypes::Misc3 => MISC3,
                    enums::MiscBorderTypes::Misc4 => MISC4,
                };
                self.border_symbols = t;
            }
            regborder => {
                let reg: ratatui::symbols::border::Set =
                    match regborder {
                        enums::BorderStyle::Plain => PLAIN,
                        enums::BorderStyle::Double => DOUBLE,
                        enums::BorderStyle::Thick => THICK,
                        enums::BorderStyle::Rounded => ROUNDED,
                        _ => PLAIN,
                    };
                self.border_symbols.top_left =
                    Some(get_parsed_symbol!(reg.top_left));
                self.border_symbols.bottom_left =
                    Some(get_parsed_symbol!(reg.bottom_left));
                self.border_symbols.top_right =
                    Some(get_parsed_symbol!(reg.top_right));
                self.border_symbols.bottom_right =
                    Some(get_parsed_symbol!(reg.bottom_right));
                self.border_symbols.top_horizontal =
                    Some(get_parsed_symbol!(reg.horizontal_top));
                self.border_symbols.bottom_horizontal =
                    Some(get_parsed_symbol!(reg.horizontal_bottom));
                self.border_symbols.left_vertical =
                    Some(get_parsed_symbol!(reg.vertical_left));
                self.border_symbols.right_vertical =
                    Some(get_parsed_symbol!(reg.vertical_right));
                self.border_symbols.bottom_center =
                    Some(get_parsed_symbol!(reg.horizontal_bottom));
                self.border_symbols.top_center =
                    Some(get_parsed_symbol!(reg.horizontal_top));
                self.border_symbols.right_center =
                    Some(get_parsed_symbol!(reg.vertical_right));
                self.border_symbols.left_center =
                    Some(get_parsed_symbol!(reg.vertical_left));
            }
        };
        self.border_type = style;
        self
    }
}

impl widgets::Widget for GradientBlock<'_> {
    /// Renders the `Gradientblock` widget using the `main` function.
    ///
    /// This is part of the `Widget` trait implementation. The `render` function takes an
    /// `area` and a mutable reference to the `Buffer`, and delegates rendering to the `main` function.
    ///
    /// # Parameters:
    /// - `area`: A `Rect` that defines the area for rendering the widget.
    /// - `buf`: A mutable reference to the `Buffer` where the rendered output will be stored.
    fn render(
        self,
        area: prelude::Rect,
        buf: &mut buffer::Buffer,
    ) {
        self.main(&area, buf);
    }
}
