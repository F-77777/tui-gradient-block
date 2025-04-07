use crate::{
    enums,
    gradient_block::{self, SS},
    structs::flags,
    text::Line,
    types::G,
    widgets::{self, block::title::Position},
};
use tui_rule::{create_raw_spans, generate_gradient_text};
impl<'a> gradient_block::GradientBlock<'a> {
    pub fn with_gradient(
        mut self,
        gradient: crate::structs::gradient::GradientVariation,
    ) -> Self {
        self = self
            .right_gradient(gradient.right)
            .left_gradient(gradient.left)
            .top_gradient(gradient.top)
            .bottom_gradient(gradient.bottom);
        self
    }
    /// sets the right segment
    pub fn right(mut self, seg: tui_rule::Rule) -> Self {
        self.border_segments.right.seg = seg;
        self
    }
    /// sets the left segment
    pub fn left(mut self, seg: tui_rule::Rule) -> Self {
        self.border_segments.left.seg = seg;
        self
    }
    /// sets the top segment
    pub fn top(mut self, seg: tui_rule::Rule) -> Self {
        self.border_segments.top.seg = seg;
        self
    }
    /// sets bottom segment
    pub fn bottom(mut self, seg: tui_rule::Rule) -> Self {
        self.border_segments.bottom.seg = seg;
        self
    }
    /// Sets gradient of the right segment of the border.
    pub fn right_gradient(mut self, gradient: G) -> Self {
        self.border_segments.right.seg.gradient = Some(gradient);
        self
    }
    /// Sets gradient of the left segment of the border.
    pub fn left_gradient(mut self, gradient: G) -> Self {
        self.border_segments.left.seg.gradient = Some(gradient);
        self
    }
    /// Sets gradient of the top segment of the border.
    pub fn top_gradient(mut self, gradient: G) -> Self {
        self.border_segments.top.seg.gradient = Some(gradient);
        self
    }
    /// Sets gradient of the bottom segment of the border.
    pub fn bottom_gradient(mut self, gradient: G) -> Self {
        self.border_segments.bottom.seg.gradient = Some(gradient);
        self
    }
    pub fn margin(mut self, horizontal: u16, vertical: u16) -> Self {
        let marg = ratatui::prelude::layout::Margin::new(
            horizontal, vertical,
        );
        let segs = &mut self.border_segments;
        segs.bottom.seg.area_margin = marg;
        segs.top.seg.area_margin = marg;
        segs.right.seg.area_margin = marg;
        segs.left.seg.area_margin = marg;
        self
    }
    pub fn horizontal_margin(mut self, margin: u16) -> Self {
        let segs = &mut self.border_segments;
        segs.bottom.seg.area_margin.horizontal = margin;
        segs.top.seg.area_margin.horizontal = margin;
        segs.right.seg.area_margin.horizontal = margin;
        segs.left.seg.area_margin.horizontal = margin;
        self
    }
    pub fn vertical_margin(mut self, margin: u16) -> Self {
        let segs = &mut self.border_segments;
        segs.bottom.seg.area_margin.vertical = margin;
        segs.top.seg.area_margin.vertical = margin;
        segs.right.seg.area_margin.vertical = margin;
        segs.left.seg.area_margin.vertical = margin;
        self
    }
    pub fn right_padding(mut self, padding: u16) -> Self {
        let segs = &mut self.border_segments;
        segs.top.seg.padding.right = padding;
        segs.bottom.seg.padding.right = padding;
        segs.right.seg.padding.right = padding;
        self
    }
    pub fn left_padding(mut self, padding: u16) -> Self {
        let segs = &mut self.border_segments;
        segs.top.seg.padding.left = padding;
        segs.bottom.seg.padding.left = padding;
        segs.left.seg.padding.left = padding;
        self
    }

    pub fn top_padding(mut self, padding: u16) -> Self {
        let segs = &mut self.border_segments;
        segs.top.seg.padding.top = padding;
        segs.left.seg.padding.top = padding;
        segs.right.seg.padding.top = padding;
        self
    }

    pub fn bottom_padding(mut self, padding: u16) -> Self {
        let segs = &mut self.border_segments;
        segs.bottom.seg.padding.bottom = padding;
        segs.left.seg.padding.bottom = padding;
        segs.right.seg.padding.bottom = padding;
        self
    }

    pub fn borders(
        mut self,
        borders: widgets::Borders,
        corners: bool,
    ) -> Self {
        use widgets::Borders as B;
        macro_rules! set_seg_state {
            ($seg:ident, $state:expr) => {
                self.border_segments.$seg.should_be_rendered = $state;
            };
        }
        macro_rules! set_corner {
            ($seg:ident, $val:ident) => {
                self.border_segments.$seg.seg.symbol_set.$val = ' '
            };
        }
        if !borders.contains(B::RIGHT) {
            set_seg_state!(right, false);
            if corners {
                set_corner!(top, end);
                set_corner!(bottom, end);
            }
        }
        if !borders.contains(B::LEFT) {
            set_seg_state!(left, false);
            if corners {
                set_corner!(top, start);
                set_corner!(bottom, start);
            }
        }
        if !borders.contains(B::TOP) {
            set_seg_state!(top, false);
            if corners {
                set_corner!(left, start);
                set_corner!(right, start);
            }
        }
        if !borders.contains(B::BOTTOM) {
            set_seg_state!(bottom, false);
            if corners {
                set_corner!(right, end);
                set_corner!(left, end);
            }
        }
        if borders == B::NONE {
            set_seg_state!(bottom, false);
            set_seg_state!(left, false);
            set_seg_state!(right, false);
            set_seg_state!(top, false);
        }
        self
    }

    pub fn corners(mut self, corners: flags::Corners) -> Self {
        use flags::Corners as C;
        macro_rules! set_corner {
            ($seg:ident, $corner:ident) => {
                self.border_segments.$seg.seg.symbol_set.$corner = ' '
            };
        }
        if !corners.contains(C::TOP_RIGHT) {
            set_corner!(right, start);
            set_corner!(top, end);
        }
        if !corners.contains(C::TOP_LEFT) {
            set_corner!(left, start);
            set_corner!(top, start);
        }
        if !corners.contains(C::BOTTOM_LEFT) {
            set_corner!(left, end);
            set_corner!(bottom, start);
        }
        if !corners.contains(C::BOTTOM_RIGHT) {
            set_corner!(bottom, end);
            set_corner!(right, end);
        }
        if corners == C::NONE {
            set_corner!(bottom, end);
            set_corner!(right, end);
            set_corner!(left, end);
            set_corner!(bottom, start);
            set_corner!(left, start);
            set_corner!(top, start);
            set_corner!(right, start);
            set_corner!(top, end);
        }
        self
    }
    pub fn center_symbols(
        mut self,
        symbols: flags::CenterSymbols,
    ) -> Self {
        use flags::CenterSymbols as S;
        macro_rules! set_symb {
            ($seg:ident) => {
                self.border_segments.$seg.seg.symbol_set.center = ' '
            };
        }
        if !symbols.contains(S::LEFT_CENTER) {
            set_symb!(left);
        }
        if !symbols.contains(S::RIGHT_CENTER) {
            set_symb!(right);
        }
        if !symbols.contains(S::BOTTOM_CENTER) {
            set_symb!(bottom);
        }
        if !symbols.contains(S::TOP_CENTER) {
            set_symb!(top);
        }
        if symbols == S::NONE {
            set_symb!(top);
            set_symb!(right);
            set_symb!(left);
            set_symb!(bottom);
        }
        self
    }
    pub fn title_top<I: Into<Line<'a>>>(mut self, title: I) -> Self {
        self.titles.push((title.into(), Position::Top));
        self
    }
    pub fn title_bottom<I: Into<Line<'a>>>(
        mut self,
        title: I,
    ) -> Self {
        self.titles.push((title.into(), Position::Bottom));
        self
    }
    /// Sets the border style for the block.
    ///
    /// If this function is not called, the border will be plain by default.
    ///
    /// # Parameters
    /// - `style`: A `BorderStyle` enum value that determines the appearance of the border.
    ///   - `BorderStyle::New`: Empty to be set manually.
    ///   - `BorderStyle::Custom`: Custom border from `SegmentSet` struct
    ///
    /// # Example 1: Using a standard border style
    /// ```
    /// let border = GradientBlock::new().border_style(BorderStyle::Double);
    /// ```
    ///
    /// # Example 2: Using a miscellaneous border style
    /// ```
    /// let border = GradientBlock::new().with_border_style(BorderStyle::Custom(preset::MISC3));
    /// ```
    ///
    /// # Example 3: Using a custom border type
    /// ```
    /// let border = GradientBlock::new()
    ///     .with_border_style(BorderStyle::New)
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
    /// ```let
    /// let block = GradientBlock::new().border_style(BorderStyle::Double);
    /// ```
    pub fn with_border_style(
        mut self,
        style: enums::BorderStyle,
    ) -> Self {
        match style {
            enums::BorderStyle::CustomSet(t) => {
                self.border_segments =
                    self.border_segments.from_segment_set(t);
            }
            enums::BorderStyle::NewSet => {
                self.border_segments = self
                    .border_segments
                    .from_segment_set(crate::preset::EMPTY);
            }
            enums::BorderStyle::RatatuiSet(t) => {
                self.border_segments = self
                    .border_segments
                    .from_segment_set(SS::from_ratatui_set(t));
            }
        };
        self
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
    pub fn titles(mut self, titles: &'a [(Line, Position)]) -> Self {
        self.titles = titles.to_vec();
        self
    }
    pub fn title(mut self, title: Line<'a>, pos: Position) -> Self {
        self.titles.push((title, pos));
        self
    }
    /// Sets the symbol for the top-right corner of the border.
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used in the top-right corner.
    ///
    /// # Example
    /// ```
    /// let border = GradientBlock::new().top_right('#');
    /// ```
    pub const fn top_right(mut self, symb: char) -> Self {
        self.border_segments.right.seg.symbol_set.start = symb;
        self.border_segments.top.seg.symbol_set.end = symb;
        self
    }

    /// Sets the symbol for the top-left corner of the border.
    ///
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used in the top-left corner.
    ///
    /// # Example
    /// ```
    /// let border = GradientBlock::new().top_left('*');
    /// ```
    pub const fn top_left(mut self, symb: char) -> Self {
        self.border_segments.left.seg.symbol_set.start = symb;
        self.border_segments.top.seg.symbol_set.start = symb;
        self
    }

    /// Sets the symbol for the bottom-right corner of the border.
    ///
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used in the bottom-right corner.
    ///
    /// # Example
    /// ```
    /// let border = GradientBlock::new().bottom_right('%');
    /// ```
    pub const fn bottom_right(mut self, symb: char) -> Self {
        self.border_segments.bottom.seg.symbol_set.end = symb;
        self.border_segments.right.seg.symbol_set.end = symb;
        self
    }

    /// Sets the symbol for the bottom-left corner of the border.
    ///
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used in the bottom-left corner.
    ///
    /// # Example
    /// ```
    /// let border = GradientBlock::new().bottom_left('@');
    /// ```
    pub const fn bottom_left(mut self, symb: char) -> Self {
        self.border_segments.bottom.seg.symbol_set.start = symb;
        self.border_segments.left.seg.symbol_set.end = symb;
        self
    }

    /// Sets the symbol for the bottom horizontal segment.
    ///
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used for the bottom horizontal border.
    ///
    /// # Example
    /// ```
    /// let border = GradientBlockr::new().bottom_horizontal_symbol('-');
    /// ```
    pub const fn bottom_horizontal_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_segments.bottom.seg.symbol_set.rep_1 = symb;
        self.border_segments.bottom.seg.symbol_set.rep_2 = symb;

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
        self.border_segments.top.seg.symbol_set.rep_1 = symb;
        self.border_segments.top.seg.symbol_set.rep_2 = symb;
        self
    }

    /// Sets the symbol for the right vertical border segment.
    ///
    /// # Parameters
    /// - `symb`: A `char` representing the symbol to be used for the right vertical border.
    ///
    /// # Example
    /// ```
    /// let border = GradientBlock::new().right_vertical_symbol('|');
    /// ```
    pub const fn right_vertical_symbol(mut self, symb: char) -> Self {
        self.border_segments.right.seg.symbol_set.rep_1 = symb;
        self.border_segments.right.seg.symbol_set.rep_2 = symb;
        self
    }
    /// Sets the left vertical border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = GradientBlock::new().left_vertical_symbol('|');
    /// ```
    pub const fn left_vertical_symbol(mut self, symb: char) -> Self {
        self.border_segments.left.seg.symbol_set.rep_1 = symb;
        self.border_segments.left.seg.symbol_set.rep_2 = symb;
        self
    }

    /// Sets the top center border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = GradientBlock::new().top_center_symbol('─');
    /// ```
    pub const fn top_center_symbol(mut self, symb: char) -> Self {
        self.border_segments.top.seg.symbol_set.center = symb;
        self
    }

    /// Sets the bottom center border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = GradientBlock::new().bottom_center_symbol('═');
    /// ```
    pub const fn bottom_center_symbol(mut self, symb: char) -> Self {
        self.border_segments.bottom.seg.symbol_set.center = symb;
        self
    }

    /// Sets the left center vertical border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = GradientBlock::new().left_center_symbol('+');
    /// ```
    pub const fn left_center_symbol(mut self, symb: char) -> Self {
        self.border_segments.left.seg.symbol_set.center = symb;
        self
    }

    /// Sets the right center vertical border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = GradientBlock::new().right_center_symbol('+');
    /// ```
    pub const fn right_center_symbol(mut self, symb: char) -> Self {
        self.border_segments.right.seg.symbol_set.center = symb;
        self
    }

    /// Sets the top right horizontal border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = GradientBlock::new().top_horizontal_right_symbol('┐');
    /// ```
    pub fn top_horizontal_right_symbol(mut self, symb: char) -> Self {
        self.border_segments.top.seg.symbol_set.rep_2 = symb;
        self
    }
    /// Sets the symbol used for the repeated section of the bottom horizontal border (right side).
    ///
    /// # Example
    /// ```
    /// let block = GradientBlock::new().bottom_horizontal_right_symbol('*');
    /// ```
    pub const fn bottom_horizontal_right_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_segments.bottom.seg.symbol_set.rep_2 = symb;
        self
    }

    /// Sets the symbol for the top horizontal left connector.
    ///
    /// # Example
    /// ```
    /// let block = GradientBlock::new().top_horizontal_left_symbol('=');
    /// ```
    pub const fn top_horizontal_left_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_segments.top.seg.symbol_set.rep_1 = symb;
        self
    }

    /// Sets the symbol for the bottom horizontal left connector.
    ///
    /// # Example
    /// ```
    /// let block = GradientBlock::new().bottom_horizontal_left_symbol('=');
    /// ```
    pub const fn bottom_horizontal_left_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_segments.bottom.seg.symbol_set.rep_1 = symb;
        self
    }

    /// Sets the symbol for the top vertical right connector.
    ///
    /// # Example
    /// ```
    /// let block = GradientBlock::new().top_vertical_right_symbol('|');
    /// ```
    pub const fn top_vertical_right_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_segments.right.seg.symbol_set.rep_1 = symb;
        self
    }

    /// Sets the symbol for the bottom vertical right connector.
    ///
    /// # Example
    /// ```
    /// let block = GradientBlock::new().bottom_vertical_right_symbol('|');
    /// ```
    pub const fn bottom_vertical_right_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_segments.right.seg.symbol_set.rep_2 = symb;
        self
    }

    /// Sets the symbol for the top vertical left connector.
    ///
    /// # Example
    /// ```
    /// let block = GradientBlock::new().top_vertical_left_symbol('|');
    /// ```
    pub const fn top_vertical_left_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_segments.left.seg.symbol_set.rep_1 = symb;
        self
    }
    pub fn with_set(mut self, set: SS) -> Self {
        self = self
            .with_border_style(enums::BorderStyle::CustomSet(set));
        self
    }

    /// Sets the symbol for the bottom vertical left connector.
    ///
    /// # Example
    /// ```
    /// let block = GradientBlock::new().bottom_vertical_left_symbol('|');
    /// ```
    pub const fn bottom_vertical_left_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_segments.left.seg.symbol_set.rep_2 = symb;
        self
    }
    pub fn fill<L: Into<Line<'a>>>(mut self, fill: L) -> Self {
        self.fill = fill.into();
        self
    }
    /// Sets the fill gradient
    /// # Example
    /// ```
    /// let block = GradientBlock::new().fill_gradient(colorgrad::preset::warm());
    /// ```
    pub fn fill_gradient<GR: colorgrad::Gradient>(
        mut self,
        gradient: GR,
    ) -> Self {
        self.fill =
            Line::from(generate_gradient_text!(self.fill, gradient));
        self
    }
}
