#![deny(clippy::todo)]

pub mod block_setter_functions;
pub mod border_styles;
pub mod gradient_block;
pub mod gradientblockfunctions;
mod macros;
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
pub mod structs {
    pub mod border_segment;
    pub mod border_symbols;
    pub mod fill;
    pub mod gradient;
    pub mod title;
}
pub mod enums;
pub use gradient_themes::{
    dark::t_midnight_blurple, light::t_misty_blue,
    warm::t_rusty_ruins,
};
pub use ratatui::{
    buffer, layout, prelude, style, symbols::border, text, widgets,
};
pub use std::{env, rc};
