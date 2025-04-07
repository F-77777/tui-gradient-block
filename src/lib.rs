pub mod border_styles;
pub mod enums;
pub mod gradient_block;
pub mod macros;
pub mod preset;
pub mod setter_functions;
pub mod types;
pub mod theme_presets {
    pub mod dark {
        pub mod t_midnight_blurple;
    }
    pub mod warm {
        pub mod t_rusty_ruins;
    }
    pub mod cool {
        pub mod t_minty_green;
        pub mod t_misty_blue;
        pub mod t_zombie_dreams;
    }
    pub mod multi_color {
        pub mod t_colorgrad_warm;
        pub mod t_rainbow;
    }
    pub mod misc {
        pub mod t_monochrome;
    }
}
pub mod structs {
    pub mod border_segment;
    pub mod border_symbols;
    pub mod flags;
    pub mod gradient;
    pub mod title;
}
pub use ratatui::{
    buffer, layout, prelude, style, symbols::border, text, widgets,
};
pub use std::{env, rc};
pub use theme_presets::{
    cool::t_misty_blue, dark::t_midnight_blurple, misc::t_monochrome,
    multi_color::t_rainbow, warm::t_rusty_ruins,
};
