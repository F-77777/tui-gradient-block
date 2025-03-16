use crate::{
    border::{DOUBLE, PLAIN, ROUNDED, THICK},
    border_styles::{MISC1, MISC2, MISC3, MISC4},
    create_segment, enums, get_parsed_symbol, get_symbol,
    gradient_block,
    text::Line,
};
impl gradient_block::GradientBlock {
    pub fn set_lns(mut self) -> gradient_block::GradientBlock {
        let border_symbols = &self.border_symbols;
        let area = &self.area;
        let top_horizontal = get_symbol!(
            border_symbols.top_horizontal,
            get_parsed_symbol!(PLAIN.horizontal_top)
        );
        let left_vertical = get_symbol!(
            border_symbols.left_vertical,
            get_parsed_symbol!(PLAIN.vertical_left)
        );
        let bottom_horizontal = get_symbol!(
            border_symbols.bottom_horizontal,
            get_parsed_symbol!(PLAIN.horizontal_top)
        );
        let right_vertical = get_symbol!(
            border_symbols.right_vertical,
            get_parsed_symbol!(PLAIN.vertical_right)
        );

        let top_center_char =
            get_symbol!(border_symbols.top_center, top_horizontal);
        let bottom_center_char = get_symbol!(
            border_symbols.bottom_center,
            bottom_horizontal
        );
        let left_center_char =
            get_symbol!(border_symbols.left_center, left_vertical);
        let right_center_char =
            get_symbol!(border_symbols.right_center, right_vertical);

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
            get_parsed_symbol!(PLAIN.bottom_right)
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
            border_symbols.bottom_horizontal_right,
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
            ((area.width.saturating_sub(1) as usize) / 2)
                .saturating_sub(1);
        let top_hor_right_rep =
            ((area.width.saturating_sub(1) as usize) / 2)
                .saturating_sub(2);
        let bottom_hor_left_rep =
            ((area.width as usize + 1) / 2).saturating_sub(1);
        let bottom_hor_right_rep =
            (area.width as usize / 2).saturating_sub(2);

        let top_vert_left_rep =
            ((area.height as usize + 1) / 2).saturating_sub(2);
        let bottom_vert_left_rep =
            (area.height as usize / 2).saturating_sub(1);
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
            self.border_segments.bottom_ln.gradient
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
            self.border_segments.right_ln.gradient
        );
        self.border_segments.top_ln.segment_text = top_ln;
        self.border_segments.right_ln.segment_text = right_ln;
        self.border_segments.left_ln.segment_text = left_ln;
        self.border_segments.bottom_ln.segment_text = bottom_ln;
        self
    }
    pub fn set_border_style(
        mut self,
        style: enums::BorderStyle,
    ) -> Self {
        match style {
            enums::BorderStyle::CustomBorderType(t) => {
                self.border_symbols = crate::structs::border_symbols::BorderSymbolsSet {
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
