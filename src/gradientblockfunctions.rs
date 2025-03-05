use ratatui::symbols::border::*;

use crate::{
    BorderStyle, MISC1, MISC2, MISC3, MISC4, MiscBorderTypes,
    SplitBorderSegments, create_segment, get_parsed_symbol,
    get_symbol,
    gradient_block::GradientBlock,
    types::{enums::GradientSegments, structs::Gradient},
};

impl GradientBlock {
    pub fn gradients(
        mut self,
        rgblist: Vec<(GradientSegments, Gradient)>,
    ) -> GradientBlock {
        for (segment, gradient) in rgblist {
            let gradient_data = gradient;
            match segment {
                GradientSegments::Left => {
                    self.border_segments.left_ln.gradient =
                        gradient_data;
                    self.border_segments
                        .left_ln
                        .should_use_gradient = true;
                }
                GradientSegments::Right => {
                    self.border_segments.right_ln.gradient =
                        gradient_data;
                    self.border_segments
                        .right_ln
                        .should_use_gradient = true;
                }
                GradientSegments::TopHorizontalLeftLn => {
                    self.border_segments
                        .top_horizontal_left_ln
                        .gradient = gradient_data;
                    self.border_segments
                        .top_horizontal_left_ln
                        .should_use_gradient = true;
                }
                GradientSegments::TopHorizontalRightLn => {
                    self.border_segments
                        .top_horizontal_right_ln
                        .gradient = gradient_data;
                    self.border_segments
                        .top_horizontal_right_ln
                        .should_use_gradient = true;
                }
                GradientSegments::BottomHorizontalLeftLn => {
                    self.border_segments
                        .bottom_horizontal_left_ln
                        .gradient = gradient_data;
                    self.border_segments
                        .bottom_horizontal_left_ln
                        .should_use_gradient = true;
                }
                GradientSegments::BottomHorizontalRightLn => {
                    self.border_segments
                        .bottom_horizontal_right_ln
                        .gradient = gradient_data;
                    self.border_segments
                        .bottom_horizontal_right_ln
                        .should_use_gradient = true;
                }
                GradientSegments::TopVerticalLeftLn => {
                    self.border_segments
                        .top_vertical_left_ln
                        .gradient = gradient_data;
                    self.border_segments
                        .top_vertical_left_ln
                        .should_use_gradient = true;
                }
                GradientSegments::TopVerticalRightLn => {
                    self.border_segments
                        .top_vertical_right_ln
                        .gradient = gradient_data;
                    self.border_segments
                        .top_vertical_right_ln
                        .should_use_gradient = true;
                }
                GradientSegments::BottomVerticalLeftLn => {
                    self.border_segments
                        .bottom_vertical_left_ln
                        .gradient = gradient_data;
                    self.border_segments
                        .bottom_vertical_left_ln
                        .should_use_gradient = true;
                }
                GradientSegments::BottomVerticalRightLn => {
                    self.border_segments
                        .bottom_vertical_right_ln
                        .gradient = gradient_data;
                    self.border_segments
                        .bottom_vertical_right_ln
                        .should_use_gradient = true;
                }
                GradientSegments::Top => {
                    self.border_segments.top_ln.gradient =
                        gradient_data;
                    self.border_segments.top_ln.should_use_gradient =
                        true;
                }
                GradientSegments::Bottom => {
                    self.border_segments.bottom_ln.gradient =
                        gradient_data;
                    self.border_segments
                        .bottom_ln
                        .should_use_gradient = true;
                }
            }
        }
        self
    }
    pub fn set_lns(mut self) -> GradientBlock {
        macro_rules! set_segment {
            ($segment:ident, $text:expr, $render:expr) => {
                self.border_segments.$segment.should_be_rendered =
                    $render;
                self.border_segments.$segment.segment_text =
                    $text.clone();
            };
        }
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

        if self.split_segments.contains(SplitBorderSegments::TOP) {
            let top_horizontal_left_ln = create_segment!(
                top_left,
                top_horizontal_left,
                top_hor_left_rep,
                top_center_char
            );
            let top_horizontal_right_ln = create_segment!(
                top_center_char,
                top_horizontal_right,
                top_hor_right_rep,
                top_right
            );

            set_segment!(top_ln, String::new(), false);
            set_segment!(
                top_horizontal_left_ln,
                top_horizontal_left_ln,
                true
            );
            set_segment!(
                top_horizontal_right_ln,
                top_horizontal_right_ln,
                true
            );
        }

        if self.split_segments.contains(SplitBorderSegments::BOTTOM) {
            let bottom_horizontal_left_ln = create_segment!(
                bottom_left,
                bottom_horizontal_left,
                top_hor_left_rep,
                bottom_center_char
            );
            let bottom_horizontal_right_ln = create_segment!(
                bottom_center_char,
                bottom_horizontal_right,
                top_hor_right_rep,
                bottom_right
            );

            set_segment!(bottom_ln, String::new(), false);
            set_segment!(
                bottom_horizontal_left_ln,
                bottom_horizontal_left_ln,
                true
            );
            set_segment!(
                bottom_horizontal_right_ln,
                bottom_horizontal_right_ln,
                true
            );
        }

        if self.split_segments.contains(SplitBorderSegments::LEFT) {
            let top_vertical_left_ln = create_segment!(
                top_left,
                top_vertical_left,
                top_vert_left_rep,
                left_center_char
            );
            let bottom_vertical_left_ln = create_segment!(
                left_center_char,
                bottom_vertical_left,
                bottom_vert_left_rep,
                bottom_left
            );

            set_segment!(left_ln, String::new(), false);
            set_segment!(
                top_vertical_left_ln,
                top_vertical_left_ln,
                true
            );
            set_segment!(
                bottom_vertical_left_ln,
                bottom_vertical_left_ln,
                true
            );
        }

        if self.split_segments.contains(SplitBorderSegments::RIGHT) {
            let top_vertical_right_ln = create_segment!(
                top_right,
                top_vertical_right,
                top_vert_left_rep,
                right_center_char
            );
            let bottom_vertical_right_ln = create_segment!(
                right_center_char,
                bottom_vertical_right,
                bottom_vert_left_rep,
                bottom_right
            );
            set_segment!(right_ln, String::new(), false);
            set_segment!(
                top_vertical_right_ln,
                top_vertical_right_ln,
                true
            );
            set_segment!(
                bottom_vertical_right_ln,
                bottom_vertical_right_ln,
                true
            );
        }
        if self.split_segments == SplitBorderSegments::ALL {
            set_segment!(top_ln, String::new(), false);
            set_segment!(bottom_ln, String::new(), false);
            set_segment!(left_ln, String::new(), false);
            set_segment!(right_ln, String::new(), false);
        }

        if self.split_segments.is_empty() {
            let top_ln = create_segment!(
                top_left,
                top_horizontal_left,
                bottom_hor_left_rep,
                top_center_char,
                top_horizontal_right,
                bottom_hor_right_rep,
                top_right
            );
            let bottom_ln = create_segment!(
                bottom_left,
                bottom_horizontal_left,
                bottom_hor_left_rep,
                bottom_center_char,
                bottom_horizontal_right,
                bottom_hor_right_rep,
                bottom_right
            );
            let left_ln = create_segment!(
                top_left,
                top_vertical_left,
                top_vert_left_rep,
                left_center_char,
                bottom_vertical_left,
                bottom_vert_left_rep,
                bottom_left
            );
            let right_ln = create_segment!(
                top_right,
                top_vertical_right,
                top_vert_left_rep,
                right_center_char,
                bottom_vertical_right,
                bottom_vert_left_rep,
                bottom_right
            );

            set_segment!(
                top_horizontal_left_ln,
                String::new(),
                false
            );
            set_segment!(
                top_horizontal_right_ln,
                String::new(),
                false
            );
            set_segment!(
                bottom_horizontal_left_ln,
                String::new(),
                false
            );
            set_segment!(
                bottom_horizontal_right_ln,
                String::new(),
                false
            );

            set_segment!(top_ln, top_ln, true);
            set_segment!(bottom_ln, bottom_ln, true);
            set_segment!(left_ln, left_ln, true);
            set_segment!(right_ln, right_ln, true);
        }
        self
    }
    pub fn set_border_style(mut self, style: BorderStyle) -> Self {
        match &style {
            BorderStyle::CustomBorderType => {}
            BorderStyle::MiscBorder(t) => {
                let miscborder = match t {
                    MiscBorderTypes::Misc1 => MISC1,
                    MiscBorderTypes::Misc2 => MISC2,
                    MiscBorderTypes::Misc3 => MISC3,
                    MiscBorderTypes::Misc4 => MISC4,
                };
                self.border_symbols.top_left =
                    Some(miscborder.top_left);
                self.border_symbols.bottom_left =
                    Some(miscborder.bottom_left);
                self.border_symbols.top_right =
                    Some(miscborder.top_right);
                self.border_symbols.bottom_right =
                    Some(miscborder.bottom_right);
                self.border_symbols.top_horizontal =
                    Some(miscborder.top);
                self.border_symbols.bottom_horizontal =
                    Some(miscborder.bottom);
                self.border_symbols.left_vertical =
                    Some(miscborder.left);
                self.border_symbols.right_vertical =
                    Some(miscborder.right);
                self.border_symbols.bottom_center =
                    Some(miscborder.bottom_center);
                self.border_symbols.top_center =
                    Some(miscborder.top_center);
                self.border_symbols.right_center =
                    Some(miscborder.right_center);
                self.border_symbols.left_center =
                    Some(miscborder.left_center);
            }
            regborder => {
                let reg = match regborder {
                    BorderStyle::Plain => PLAIN,
                    BorderStyle::Double => DOUBLE,
                    BorderStyle::Thick => THICK,
                    BorderStyle::Rounded => ROUNDED,
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
        self.bordertype = style;
        self
    }
}
