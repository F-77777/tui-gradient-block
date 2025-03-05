pub mod gradient_block;
pub mod gradientblockfunctions;
mod macros;
pub mod predefined;
pub mod types;
pub mod gradient_themes {
    pub mod dark {
        pub mod t_midnight_blurple;
    }
    pub mod warm {
        pub mod t_rusty_ruins;
    }
    pub mod light {
        pub mod t_misty_blue;
    }
}
pub use crate::{
    predefined::{
        border_styles::{MISC1, MISC2, MISC3, MISC4},
        consts::ERROR_MESSAGE,
    },
    types::{
        enums::{
            BorderStyle, GradientSegments, MiscBorderTypes,
            TitleDirection,
        },
        structs::{
            BorderSegment, BorderSegments, BorderSymbols, Fill,
            Gradient, GradientThemeSet, SplitBorderSegments, Title,
            TitleSet,
        },
    },
};
pub use gradient_themes::{
    dark::t_midnight_blurple::*, light::t_misty_blue::*,
    warm::t_rusty_ruins::*,
};

pub use lazy_static::lazy_static;
pub use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    symbols::border::{DOUBLE, PLAIN, ROUNDED, THICK},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};
pub use std::{env, rc::Rc};
pub use unicode_width::UnicodeWidthChar;
