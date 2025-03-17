use crate::{
    enums, gradient_block,
    structs::flags,
    text::Line,
    types::G,
    widgets::{self, block::title::Position},
};
impl gradient_block::GradientBlock<'a> {
    /// Sets gradient of the right segment of the border.
    pub fn set_right_ln_gradient(
        mut self,
        gradient: G,
    ) -> Self {
        self.border_segments.right_ln.gradient =
            Some(gradient);
        self.border_segments
            .right_ln
            .should_use_gradient = true;
        self
    }
    /// Sets gradient of the left segment of the border.
    pub fn set_left_ln_gradient(
        mut self,
        gradient: G,
    ) -> Self {
        self.border_segments.left_ln.gradient =
            Some(gradient);
        self.border_segments
            .left_ln
            .should_use_gradient = true;
        self
    }
    /// Sets gradient of the top segment of the border.
    pub fn set_top_ln_gradient(
        mut self,
        gradient: G,
    ) -> Self {
        self.border_segments.top_ln.gradient =
            Some(gradient);
        self.border_segments
            .top_ln
            .should_use_gradient = true;
        self
    }
    /// Sets gradient of the bottom segment of the border.
    pub fn set_bottom_ln_gradient(
        mut self,
        gradient: G,
    ) -> Self {
        self.border_segments.bottom_ln.gradient =
            Some(gradient);
        self.border_segments
            .bottom_ln
            .should_use_gradient = true;
        self
    }
    pub fn borders(
        mut self,
        borders: widgets::Borders,
    ) -> Self {
        use widgets::Borders as B;
        macro_rules! set_seg_state {
            ($seg:ident, $state:expr) => {
                self.border_segments
                    .$seg
                    .should_be_rendered = $state;
            };
        }
        macro_rules! set_corner {
            ($corner:ident) => {
                self.border_symbols.$corner = ' '
            };
        }
        if !borders.contains(B::RIGHT) {
            set_seg_state!(right_ln, false);
            set_corner!(top_right);
            set_corner!(bottom_right);
        }
        if !borders.contains(B::LEFT) {
            set_seg_state!(left_ln, false);
            set_corner!(top_left);
            set_corner!(bottom_left);
        }
        if !borders.contains(B::TOP) {
            set_seg_state!(top_ln, false);
            set_corner!(top_left);
            set_corner!(top_right);
        }
        if !borders.contains(B::BOTTOM) {
            set_seg_state!(bottom_ln, false);
            set_corner!(bottom_left);
            set_corner!(bottom_right);
        }
        if borders == B::None {
            set_seg_state!(bottom_ln, false);
            set_seg_state!(left_ln, false);
            set_seg_state!(right_ln, false);
            set_seg_state!(top_ln, false);
        }

        self.corners(flags::Corners::NONE)
    }

    pub fn corners(
        mut self,
        corners: flags::Corners,
    ) -> Self {
        use flags::Corners as C;
        macro_rules! set_corner {
            ($corner:ident) => {
                self.border_symbols.$corner = ' '
            };
        }
        if !corners.contains(C::TOP_RIGHT) {
            set_corner!(top_right)
        }
        if !corners.contains(C::TOP_LEFT) {
            set_corner!(top_left);
        }
        if !corners.contains(C::BOTTOM_LEFT) {
            set_corner!(bottom_Left);
        }
        if !corners.contains(C::BOTTOM_RIGHT) {
            set_corner!(bottom_right);
        }
        if corners == C::NONE {
            set_corner!(bottom_right);
            set_corner!(bottom_left);
            set_corner!(top_left);
            set_corner!(top_right);
        }
        self
    }
    pub fn center_symbols(
        mut self,
        symbols: flags::CenterSymbols,
    ) -> Self {
        use flags::CenterSymbols as S;
        macro_rules! set_symb {
            ($symb:ident) => {
                self.border_symbols.$symb = ' '
            };
        }
        if !symbols.contains(S::LEFT_CENTER) {
            set_symb!(left_center);
        }
        if !symbols.contains(S::RIGHT_CENTER) {
            set_symb!(right_center);
        }
        if !symbols.contains(S::BOTTOM_CENTER) {
            set_symb!(bottom_center);
        }
        if !symbols.contains(S::TOP_CENTER) {
            set_symb!(top_center);
        }
        if symbols == S::NONE {
            set_symb!(top_center);
            set_symb!(right_center);
            set_symb!(left_center);
            set_symb!(bottom_center);
        }
        self
    }
    pub fn title_top(
        mut self,
        title: Into<Line<'a>>,
    ) -> Self {
        self.titles
            .push((title.into(), Position::Top));
        self
    }
    pub fn title_bottom(
        mut self,
        title: Into<Line<'a>>,
    ) -> Self {
        self.titles.push((
            title.into(),
            Position::Bottom,
        ));
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
    /// ```let
    /// let block = Gradientblock::new().border_style(BorderStyle::Double);
    /// ```
    pub fn border_style(
        self,
        style: enums::BorderStyle,
    ) -> Self {
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
    pub fn titles(
        mut self,
        titles: &[(Title, Position)],
    ) -> Self {
        self.titles = (titles.to_vec());
        self
    }
    pub fn title(
        mut self,
        title: Line<'_>,
        pos: Position,
    ) -> Self {
        self.titles.push((title.clone, pos));
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
    pub const fn top_right(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.top_right =
            Some(symb);
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
    pub const fn top_left(
        mut self,
        symb: char,
    ) -> Self {
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
    pub const fn bottom_right(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.bottom_right =
            Some(symb);
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
    pub const fn bottom_left(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.bottom_left =
            Some(symb);
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
        self.border_symbols.bottom_horizontal =
            Some(symb);
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
    pub const fn top_horizontal_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.top_horizontal =
            Some(symb);
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
    pub const fn right_vertical_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.right_vertical =
            Some(symb);
        self
    }
    /// Sets the left vertical border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = Gradientblock::new().left_vertical_symbol('|');
    /// ```
    pub const fn left_vertical_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.left_vertical =
            Some(symb);
        self
    }

    /// Sets the top center border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = Gradientblock::new().top_center_symbol('─');
    /// ```
    pub const fn top_center_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.top_center =
            Some(symb);
        self
    }

    /// Sets the bottom center border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = Gradientblock::new().bottom_center_symbol('═');
    /// ```
    pub const fn bottom_center_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.bottom_center =
            Some(symb);
        self
    }

    /// Sets the left center vertical border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = Gradientblock::new().left_center_symbol('+');
    /// ```
    pub const fn left_center_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.left_center =
            Some(symb);
        self
    }

    /// Sets the right center vertical border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = Gradientblock::new().right_center_symbol('+');
    /// ```
    pub const fn right_center_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols.right_center =
            Some(symb);
        self
    }

    /// Sets the top right horizontal border symbol.
    ///
    /// # Example
    /// ```
    /// let widget = Gradientblock::new().top_horizontal_right_symbol('┐');
    /// ```
    pub fn top_horizontal_right_symbol(
        mut self,
        symb: char,
    ) -> Self {
        self.border_symbols
            .top_horizontal_right = Some(symb);
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
        self.border_symbols
            .bottom_horizontal_right = Some(symb);
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
        self.border_symbols.top_horizontal_left =
            Some(symb);
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
        self.border_symbols
            .bottom_horizontal_left = Some(symb);
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
        self.border_symbols.top_vertical_right =
            Some(symb);
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
        self.border_symbols
            .bottom_vertical_right = Some(symb);
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
        self.border_symbols.top_vertical_left =
            Some(symb);
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
        self.border_symbols
            .bottom_vertical_left = Some(symb);
        self
    }
    pub const fn border_symbols(
        mut self,
        symbols: structs::BorderSymbolSet,
    ) -> Self {
        self.border_symbols = symbols;
        self
    }
    pub fn fill(
        mut self,
        fill: structs::Fill,
    ) -> Self {
        self.fill = fill;
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
    pub fn fill_string(
        mut self,
        string: String,
    ) -> Self {
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
    pub fn fill_gradient(
        mut self,
        gradient: structs::Gradient,
    ) -> Self {
        self.fill.gradient = Some(gradient);
        self
    }
}
