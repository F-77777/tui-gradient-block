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
/// Macro for generating line segments
macro_rules! create_segment {
    ($start:expr, $middle:expr, $repeat:expr, $end:expr) => {{
        let mut s = String::with_capacity(2 + $repeat);
        s.push($start);
        for _ in 0..$repeat {
            s.push($middle);
        }
        s.push($end);
        s
    }};
    ($start:expr, $middle1:expr, $repeat1:expr, $middle2:expr, $middle3:expr, $repeat2:expr, $end:expr) => {{
        let mut s = String::with_capacity(4 + $repeat1 + $repeat2);
        s.push($start);
        for _ in 0..$repeat1 {
            s.push($middle1);
        }
        s.push($middle2);
        for _ in 0..$repeat2 {
            s.push($middle3);
        }
        s.push($end);
        s
    }};
}
#[macro_export]
macro_rules! generate_gradient_text {
    ($ln:expr) => {
        GradientBlock::create_gradient_text(
            $ln.segment_text.as_str(),
            &$ln.gradient.gradient_list,
            &$ln.gradient.factor,
        )
    };
    ($text:expr, $gradient:expr, $factor:expr) => {
        GradientBlock::create_gradient_text(
            $text, &$gradient, &$factor,
        )
    };
}
#[macro_export]
macro_rules! create_vec_spans {
    ($ln:expr) => {
        $ln.chars().map(|i| Span::raw(i.to_string())).collect()
    };
}
#[macro_export]
macro_rules! check_gradient {
    ($ln:expr) => {
        match $ln.should_use_gradient {
            true => generate_gradient_text!($ln),
            false => create_vec_spans!($ln.segment_text),
        }
    };
    ($title:expr, $_:expr) => {
        match &$title.gradients {
            Some(t) => generate_gradient_text!(
                $title.title_text.as_str(),
                t.gradient_list,
                t.factor
            ),
            None => create_vec_spans!($title.title_text),
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
            $ln.segment_text.len() as u16 * 2,
        );
    };
}
#[macro_export]
macro_rules! set_vertical_line {
    ($ln:expr, $used_ln:expr, $buf:expr) => {
        for (i, span) in $used_ln.iter().enumerate() {
            $buf.set_span($ln.x, $ln.y + i as u16, span, 1);
        }
    };
}

#[macro_export]
macro_rules! handle_line_logic {
    ($ln:expr, $buf:expr) => {
        let used_ln = check_gradient!($ln);
        if $ln.is_vertical {
            set_vertical_line!($ln, used_ln, $buf);
        } else {
            set_line!($ln, &Line::from(used_ln), $buf);
        }
    };
}

#[macro_export]
macro_rules! handle_title_logic {
    ($titles:expr, $type:expr, $area:expr, $buf:expr) => {
        for title in $titles {
            let x = get_aligned_position!(
                $area,
                title.alignment,
                title.title_text.len() as u16
            );
            let y = match $type {
                TitleDirection::Top => $area.top(),
                TitleDirection::Bottom => $area.bottom() - 1,
            };
            $buf.set_line(
                x,
                y,
                &Line::from(check_gradient!(title, _)),
                title.title_text.len() as u16,
            );
        }
    };
}
#[macro_export]
macro_rules! get_aligned_position {
    ($area:expr, $alignment:expr, $text_len:expr) => {
        match $alignment {
            Alignment::Left => $area.left() + 1,
            Alignment::Right => {
                ($area.right() - 1).saturating_sub($text_len)
            }
            Alignment::Center => ($area.left() + ($area.width / 2))
                .saturating_sub($text_len / 2),
        }
    };
}
#[macro_export]
macro_rules! handle_fill {
    ($fill:expr, $fillstring:expr, $area:expr, $buf:expr, $rep:expr) => {
        let fillvec = match &$fill.gradient {
            Some(gradient) => {
                generate_gradient_text!(
                    $fillstring,
                    &gradient.gradient_list,
                    &gradient.factor
                )
            }
            None => {
                create_vec_spans!($fillstring)
            }
        };
        Paragraph::new(Line::from(fillvec))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL))
            .render($area, $buf);
    };
}
/// expects type "BorderGradients"
#[macro_export]
macro_rules! generate_gradient_theme {
    ($grad:expr) => {
        vec![
            (
                GradientSegments::Top,
                Gradient {
                    gradient_list: $grad.top,
                    factor: $grad.top_fac,
                },
            ),
            (
                GradientSegments::Right,
                Gradient {
                    gradient_list: $grad.right,
                    factor: $grad.right_fac,
                },
            ),
            (
                GradientSegments::Left,
                Gradient {
                    gradient_list: $grad.left,
                    factor: $grad.left_fac,
                },
            ),
            (
                GradientSegments::Bottom,
                Gradient {
                    gradient_list: $grad.bottom,
                    factor: $grad.bottom_fac,
                },
            ),
        ]
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
                GradientBlock::new(
                    area,
                    SplitBorderSegments::empty(),
                )
                .top_titles(vec![title])
                .border_style($style)
                .set_gradients(theme.to_vec())
                .set_lines(),
                *area,
            );
        }
    };
}
#[macro_export]
macro_rules! get_transformed_int {
    ($item:expr, $f:expr, $times:expr) => {
        (($item as f32 * ($f as f32).powi($times as i32)).floor()
            as u8)
    };
}
#[macro_export]
macro_rules! get_transformed_rgb {
    ($list:expr, $f:expr, $times:expr) => {
        (
            get_transformed_int!($list.0, $f, $times),
            get_transformed_int!($list.1, $f, $times),
            get_transformed_int!($list.2, $f, $times),
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
            let app_result =
                run(&mut terminal, style, $theme_list, $title_list);
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
            gen_main, gen_other_functions, gen_use, generate_run,
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
        TitleSet {
            double_left: Title {
                title_text: "Double Left".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![$color_3, $color_2, $color_2],
                    factor: 1.0,
                }),
            },
            bottom_left: Title {
                title_text: "Bottom Left".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![$color_2, $color_2],
                    factor: 1.0,
                }),
            },
            bottom_right: Title {
                title_text: "Bottom Right".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![$color_2, $color_2],
                    factor: 1.0,
                }),
            },
            double_horizontal: Title {
                title_text: "Double Horizontal".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![
                        $color_2, $color_3, $color_3, $color_3,
                        $color_2,
                    ],
                    factor: 1.0,
                }),
            },
            double_right: Title {
                title_text: "Double Right".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![$color_2, $color_2, $color_3],
                    factor: 1.0,
                }),
            },
            top_left: Title {
                title_text: "Top Left".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![$color_3, $color_2, $color_2],
                    factor: 1.0,
                }),
            },
            top_right: Title {
                title_text: "Top Right".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![$color_2, $color_2, $color_3],
                    factor: 1.0,
                }),
            },
            double_vertical: Title {
                title_text: "Double Vertical".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![$color_2, $color_2],
                    factor: 1.0,
                }),
            },
            left: Title {
                title_text: "Left".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![$color_3, $color_2, $color_2],
                    factor: 1.0,
                }),
            },
            right: Title {
                title_text: "Right".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![$color_2, $color_2, $color_3],
                    factor: 1.0,
                }),
            },
            top: Title {
                title_text: "Top".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![$color_3, $color_3, $color_3],
                    factor: 1.0,
                }),
            },
            bottom: Title {
                title_text: "Bottom".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![$color_2, $color_2],
                    factor: 1.0,
                }),
            },
            base1: Title {
                title_text: "Base 1".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![$color_2, $color_3],
                    factor: 1.0,
                }),
            },

            base2: Title {
                title_text: "Base 2".to_string(),
                alignment: Alignment::Center,
                gradients: Some(Gradient {
                    gradient_list: vec![$color_3, $color_2],
                    factor: 1.0,
                }),
            },
        }
    };
    ($color_1:expr, $color_2:expr) => {
        TitleSet {
            double_left: simple_title!(
                "Double Left",
                vec![$color_2, $color_2]
            ),
            bottom_left: simple_title!(
                "Bottom Left",
                vec![$color_2, $color_2]
            ),
            bottom_right: simple_title!(
                "Bottom Right",
                vec![$color_2, $color_2]
            ),
            double_horizontal: simple_title!(
                "Double Horizontal",
                vec![
                    $color_1, $color_2, $color_2, $color_1, $color_1
                ]
            ),
            double_right: simple_title!(
                "Double Right",
                vec![$color_2, $color_1, $color_1]
            ),
            top_left: simple_title!(
                "Top Left",
                vec![$color_1, $color_1, $color_2]
            ),
            top_right: simple_title!(
                "Top Right",
                vec![$color_1, $color_1, $color_2]
            ),
            double_vertical: simple_title!(
                "Double Vertical",
                vec![$color_2, $color_1]
            ),
            left: simple_title!("Left", vec![$color_1, $color_2]),
            right: simple_title!(
                "Right",
                vec![$color_1, $color_1, $color_2]
            ),
            top: simple_title!("Up", vec![$color_1, $color_2]),
            bottom: simple_title!("Down", vec![$color_1, $color_2]),
            base1: simple_title!("Base 1", vec![$color_2, $color_1]),
            base2: simple_title!("Base 2", vec![$color_1, $color_2]),
        }
    };
}
#[macro_export]
macro_rules! simple_title {
    ($name:expr, $colors:expr) => {
        Title {
            title_text: $name.to_string(),
            alignment: Alignment::Center,
            gradients: Some(Gradient {
                gradient_list: $colors,
                factor: 1.0,
            }),
        }
    };
}
