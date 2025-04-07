#[macro_export]
macro_rules! get_aligned_position {
    ($area:expr, $alignment:expr, $text_len:expr, $padding_left:expr, $padding_right:expr) => {
        match $alignment {
            Some(prelude::Alignment::Left) => {
                $area.left().saturating_sub($padding_left + 1)
            }
            Some(prelude::Alignment::Right) => ($area.right())
                .saturating_sub($text_len + 1)
                .saturating_sub($padding_right),
            Some(prelude::Alignment::Center) => ($area.left() + $area.width / 2)
                .saturating_sub($text_len / 2)
                .saturating_sub($padding_right)
                .saturating_add($padding_left),
            None => $area.left().saturating_add($padding_left + 1),
        }
    };
}
#[macro_export]
macro_rules! handle_fill {
    ($fill:expr, $area:expr, $buf:expr) => {
        Paragraph::new($fill)
            .wrap(widgets::Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL))
            .render($area, $buf);
    };
}
#[macro_export]
macro_rules! handle_args {
    () => {{
        use ratatui::symbols::border::{THICK, ROUNDED, DOUBLE, PLAIN};
        let style: ratatui::symbols::border::Set;
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 || args.len() > 2 {
            eprintln!(
                "Usage: cargo run --example <example_name> <parameter>"
            );
            eprintln!(
                "Valid parameters are plain, double, thick, and rounded"
            );
            std::process::exit(1);
        }

        let parameter = &args[1];
        match parameter.to_lowercase().as_str() {
            "plain" => {
                style = PLAIN;
            }
            "double" => {
                style = DOUBLE;
            }
            "thick" => {
                style = THICK;
            }
            "rounded" | "round" => {
                style = ROUNDED;
            }
            _ => {
                eprintln!(
                    "Please enter a valid parameter. Valid parameters are plain, double, thick, and rounded"
                );
                std::process::exit(1);
            }
        }
        style
    }};
}
#[macro_export]
macro_rules! to_ratatui_color {
    ($c:expr) => {
        ratatui::style::Color::Rgb(
            ($c.r * 255.0) as u8,
            ($c.g * 255.0) as u8,
            ($c.b * 255.0) as u8,
        )
    };
}
#[macro_export]
macro_rules! gen_titles {
    ($color:expr) => {{
        use $crate::{
            structs::title::TitleSet, style::Style, text::Line,
            to_ratatui_color as to_r_c,
            widgets::block::Position as P,
        };
        let s = Style::new().fg(to_r_c!($color));
        TitleSet {
            double_corners_left: (
                Line::from("Two Corners Left").centered().style(s),
                P::Top,
            ),
            bottom_left: (
                Line::from("Bottom Left").style(s).centered(),
                P::Top,
            ),
            bottom_right: (
                Line::from("Bottom Right").centered().style(s),
                P::Top,
            ),
            top_left: (
                Line::from("Top Left").centered().style(s),
                P::Top,
            ),
            top_right: (
                Line::from("Top Right").centered().style(s),
                P::Top,
            ),
            horizontal: (
                Line::from("Horizontal").centered().style(s),
                P::Top,
            ),
            double_corners_right: (
                Line::from("Two Corners Right")
                    .centered()
                    .style(s),
                P::Top,
            ),
            vertical: (
                Line::from("Vertical").centered().style(s),
                P::Top,
            ),
            left: (Line::from("Left").centered().style(s), P::Top),
            right: (Line::from("Right").centered().style(s), P::Top),
            up: (Line::from("Up").centered().style(s), P::Top),
            down: (Line::from("Down").centered().style(s), P::Top),
            misc1: (Line::from("Misc 1").centered().style(s), P::Top),
            misc2: (Line::from("Misc 2").centered().style(s), P::Top),
        }
    }};
}

#[macro_export]
macro_rules! generate_theme_use {
    () => {
        pub use colorgrad::{Color, GradientBuilder};
        pub use tui_rule::generate_gradient_text;
        pub use $crate::{
            gen_titles, gradient_block,
            structs::{
                gradient::{
                    GradientTheme as GT, GradientVariation as GV,
                },
                title::TitleSet,
            },
            types::G,
        };
    };
}
#[macro_export]
macro_rules! generate_from_json {
    ($path:expr, $returntype:ty) => {{
        let f = std::fs::File::open($path)?;
        let gradient: $returntype =
            serde_json::from_reader(std::io::BufReader::new(f))?;
        Ok(gradient)
    }};
}
#[macro_export]
macro_rules! generate_to_json {
    ($val:expr) => {
        serde_json::to_string_pretty(&$val).unwrap()
    };
}
