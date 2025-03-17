#[macro_export]
macro_rules! get_symbol {
    ($field:expr, $default:expr) => {
        $field.unwrap_or($default)
    };
}
#[macro_export]
macro_rules! get_parsed_symbol {
    ($symbol:expr) => {
        $symbol.parse().unwrap()
    };
}
#[macro_export]
/// Macro for generating border segments
macro_rules! create_segment {
    ($start:expr, $middle1:expr, $repeat1:expr, $middle2:expr, $middle3:expr, $repeat2:expr, $end:expr, $gradient:expr) => {{
        let mut s = String::with_capacity(
            4 + $repeat1 + $repeat2,
        );
        s.push($start);
        for _ in 0..$repeat1 {
            s.push($middle1);
        }
        s.push($middle2);
        for _ in 0..$repeat2 {
            s.push($middle3);
        }
        s.push($end);
        Line::from(Self::create_gradient_text(
            &s, &$gradient,
        ))
    }};
}

#[macro_export]
macro_rules! generate_gradient_text {
    ($ln:expr) => {
        if let Some(boxed) = &$ln.gradient {
            let seg_text = Rc::new(&$ln.segment_text);

        $crate::gradient_block::GradientBlock::create_gradient_text(
            *&$ln.segment_text,
            boxed,
        )} else {
            Vec::new()
        }
    };
    ($text:expr, $gradient:expr) => {
        $crate::gradient_block::GradientBlock::create_gradient_text(
            $text, &$gradient,
        )
    };
}
#[macro_export]
macro_rules! check_gradient {
    ($ln:expr) => {
        match $ln.should_use_gradient {
            true => generate_gradient_text!($ln),
            false => {
                $ln.segment_text.spans.clone()
            }
        }
    };
}
#[macro_export]
macro_rules! set_line {
    ($ln:expr, $used_ln: expr, $buf:expr) => {
        $buf.set_line(
            $ln.x,
            $ln.y,
            $used_ln,
            $ln.segment_text.width() as u16,
        );
    };
}
#[macro_export]
macro_rules! set_vertical_line {
    ($ln:expr, $used_ln:expr, $buf:expr) => {
        for (i, span) in
            $used_ln.iter().enumerate()
        {
            $buf.set_span(
                $ln.x,
                $ln.y + i as u16,
                span,
                1,
            );
        }
    };
}

#[macro_export]
macro_rules! handle_line_logic {
    ($ln:expr, $buf:expr) => {
        let used_ln = check_gradient!($ln);
        if $ln.is_vertical {
            set_vertical_line!(
                $ln, used_ln, $buf
            );
        } else {
            set_line!(
                $ln,
                &Line::from(used_ln),
                $buf
            );
        }
    };
}
#[macro_export]
macro_rules! get_aligned_position {
    ($area:expr, $alignment:expr, $text_len:expr) => {
        match $alignment {
            Some(prelude::Alignment::Left) => {
                $area.left()
            }
            Some(prelude::Alignment::Right) => {
                ($area.right())
                    .saturating_sub($text_len)
            }
            Some(prelude::Alignment::Center) => {
                ($area.left() + ($area.width / 2))
                    .saturating_sub($text_len / 2)
            }
            None => $area.left(),
        }
    };
}
#[macro_export]
macro_rules! handle_title_logic {
    ($titles:expr, $area:expr, $buf:expr) => {
        for (title, pos) in $titles {
            let x = get_aligned_position!(
                $area,
                title.alignment,
                title.spans.len() as u16 + 1
            );
            let y = match pos {
                Position::Top => $area.top(),

                Position::Bottom => {
                    $area.bottom()
                }
            };

            $buf.set_line(
                x,
                y,
                &title,
                title.spans.len() as u16 + 1,
            );
        }
    };
}
#[macro_export]
macro_rules! handle_fill {
    ($fill:expr, $area:expr, $buf:expr) => {
        Paragraph::new($fill)
            .wrap(widgets::Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL),
            )
            .render($area, $buf);
    };
}
#[macro_export]
macro_rules! render_example_blocks {
    ($list:expr, $f:expr, $arealist:expr,  $style:expr, $titlelist:expr) => {
        for (((theme, area), title)) in $list
            .iter()
            .zip($arealist.iter())
            .zip($titlelist.into_iter())
        {
            $f.render_widget(
                GradientBlock::new(area)
                    .title(title)
                    .border_style($style)
                    .set_right_ln_gradient(
                        theme.right,
                    )
                    .left_ln_gradient(theme.left)
                    .top_ln_gradient(theme.top)
                    .bottom_ln_gradient(
                        theme.bottom,
                    )
                    .build(),
                *area,
            );
        }
    };
}
#[macro_export]
macro_rules! get_transformed_int {
    ($item:expr, $f:expr, $times:expr) => {
        (($item as f32
            * ($f as f32).powi($times as i32))
        .floor() as u8)
    };
}
#[macro_export]
macro_rules! get_transformed_rgb {
    ($list:expr, $f:expr, $times:expr) => {
        (
            get_transformed_int!(
                $list.0, $f, $times
            ),
            get_transformed_int!(
                $list.1, $f, $times
            ),
            get_transformed_int!(
                $list.2, $f, $times
            ),
        )
    };
}

#[macro_export]
macro_rules! handle_args {
    () => {{
        let style: BorderStyle;
        let args: Vec<String> = env::args().collect();
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
                style = BorderStyle::Plain;
            }
            "double" => {
                style = BorderStyle::Double;
            }
            "thick" => {
                style = BorderStyle::Thick;
            }
            "rounded" | "round" => {
                style = BorderStyle::Rounded;
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
macro_rules! generate_example_layout {
    ($f:expr) => {{
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .margin(1)
            .spacing(2)
            .split($f.area());
        [layout[0], layout[1], layout[2]]
    }};
}
#[macro_export]
macro_rules! generate_run {
    () => {
        use tui_gradient_block::types::typedefinitions::GradientTheme;
        use tui_gradient_block::*;
        fn run(
            terminal: &mut DefaultTerminal,
            style: BorderStyle,
            theme_list: [GradientTheme; 3],
            title_list: [Title; 3],
        ) -> io::Result<()> {
            loop {
                terminal.draw(|f| {
                    let layout = generate_example_layout!(f);
                    render_example_blocks!(
                        theme_list,
                        f,
                        [layout[0], layout[1], layout[2]],
                        style.clone(),
                        title_list.clone()
                    );
                })?;
                if let Err(e) = handle_events() {
                    eprintln!("Error handling events: {}", e);
                }
            }
        }
    };
}
#[macro_export]
macro_rules! gen_main {
    ($theme_list:expr, $title_list:expr) => {
        fn main() -> io::Result<()> {
            let style: BorderStyle;
            style = handle_args!();
            let _ = color_eyre::install();
            let mut terminal = ratatui::init();
            let app_result = run(
                &mut terminal,
                style,
                $theme_list,
                $title_list,
            );
            ratatui::restore();
            app_result
        }
    };
}
#[macro_export]
macro_rules! gen_use {
    () => {
        use ratatui::{
            self, DefaultTerminal,
            crossterm::{
                cursor,
                event::{
                    self, Event, KeyCode, KeyEvent, KeyEventKind,
                },
                execute,
            },
            prelude::{Constraint, Direction, Layout},
        };
        use std::io;
        pub use tui_gradient_block::{
            gradient_block::GradientBlock,
            gradient_themes::dark::t_midnight_blurple::*,
            render_example_blocks,
            types::{
                enums::BorderStyle,
                structs::{GradientThemeSet, SplitBorderSegments},
            },
        };
    };
}
#[macro_export]
macro_rules! gen_other_functions {
    () => {
        fn handle_events() -> io::Result<()> {
            match event::read()? {
                Event::Key(key_event)
                    if key_event.kind == KeyEventKind::Press =>
                {
                    handle_key_event(key_event)
                }
                _ => Ok(()),
            }
        }

        fn handle_key_event(key_event: KeyEvent) -> io::Result<()> {
            match key_event.code {
                KeyCode::Char('q') => Ok(exit()),
                _ => Ok(()),
            }
        }

        fn exit() {
            ratatui::restore();
            let _ = execute!(io::stdout(), cursor::Show);
            std::process::exit(0);
        }
    };
}
#[macro_export]
macro_rules! gen_example_code {
    ($theme_list:expr, $title_list:expr) => {
        use tui_gradient_block::{
            gen_main, gen_other_functions,
            gen_use, generate_run,
        };
        gen_use!();
        gen_other_functions!();
        generate_run!();
        gen_main!($theme_list, $title_list);
    };
}
#[macro_export]
macro_rules! gen_titles {
    ($color_1:expr, $color_2:expr, $color_3:expr) => {
        structs::TitleSet {
            double_left: T {
                content: Line::from(generate_gradient_text!(
                    "Double Left",
                    structs::Gradient {
                        colors: vec![$color_2],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
            bottom_left: T {
                content: Line::from(generate_gradient_text!(
                    "Bottom Left",
                    structs::Gradient {
                        colors: vec![$color_2],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
            bottom_right: T {
                content: Line::from(generate_gradient_text!(
                    "Bottom Right",
                    structs::Gradient {
                        colors: vec![$color_2],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
            horizontal: T {
                content: Line::from(generate_gradient_text!(
                    "Horizontal",
                    structs::Gradient {
                        colors: vec![
                            $color_1, $color_2, $color_2, $color_1
                        ],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
            double_right: T {
                content: Line::from(generate_gradient_text!(
                    "Double Right",
                    structs::Gradient {
                        colors: vec![$color_2, $color_2, $color_1],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
            top_left: T {
                content: Line::from(generate_gradient_text!(
                    "Top Left",
                    structs::Gradient {
                        colors: vec![$color_2, $color_2],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
            top_right: T {
                content: Line::from(generate_gradient_text!(
                    "Top Right",
                    structs::Gradient {
                        colors: vec![$color_2, $color_2],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
            vertical: T {
                content: Line::from(generate_gradient_text!(
                    "Double Vertical",
                    structs::Gradient {
                        colors: vec![$color_2, $color_2],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
            left: T {
                content: Line::from(generate_gradient_text!(
                    "Left",
                    structs::Gradient {
                        colors: vec![$color_2, $color_2, $color_1],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
            right: T {
                content: Line::from(generate_gradient_text!(
                    "Right",
                    structs::Gradient {
                        colors: vec![$color_2, $color_2, $color_1],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
            top: T {
                content: Line::from(generate_gradient_text!(
                    "Top",
                    structs::Gradient {
                        colors: vec![$color_1, $color_1],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
            bottom: T {
                content: Line::from(generate_gradient_text!(
                    "Bottom",
                    structs::Gradient {
                        colors: vec![$color_2, $color_2],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
            base1: T {
                content: Line::from(generate_gradient_text!(
                    "Base 1",
                    structs::Gradient {
                        colors: vec![$color_1, $color_2],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
            base2: T {
                content: Line::from(generate_gradient_text!(
                    "Base 2",
                    structs::Gradient {
                        colors: vec![$color_2, $color_1],
                        gradient_type:
                            enums::GradientType::CatmullRom,
                        gradient_color_count: None,
                    }
                )),
                alignment: Some(prelude::Alignment::Center),
                position: Some(P::Top),
            },
        }
    };
}

#[macro_export]
macro_rules! simple_title {
    ($name:expr, $colors:expr) => {
        Title {
            content: $name.to_string(),
            alignment: Alignment::Center,
            gradients: Some(Gradient {
                gradient_list: $colors,
                factor: 1.0,
            }),
        }
    };
}
#[macro_export]
macro_rules! generate_theme_use {
    () => {
        pub use crate::{
            gen_titles, generate_gradient_text,
            gradient_block,
        };
        use ratatui::{
            prelude, text::Line,
            widgets::block::title::Position as P,
        };
        use std::sync::Mutex;
    };
}
#[macro_export]
macro_rules! generate_from_json {
    ($path:expr, $returntype:ty) => {{
        let f = std::fs::File::open($path)?;
        let gradient: $returntype =
            serde_json::from_reader(
                std::io::BufReader::new(f),
            )?;
        Ok(gradient)
    }};
}
#[macro_export]
macro_rules! generate_to_json {
    ($val:expr) => {
        serde_json::to_string_pretty(&$val)
            .unwrap()
    };
}
