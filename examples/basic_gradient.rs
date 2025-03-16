use ratatui::{
    self, DefaultTerminal,
    crossterm::{
        cursor,
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        execute,
    },
};
use std::io;
pub use tui_gradient_block::{
    gradient_block::GradientBlock,
    gradient_themes::dark::t_midnight_blurple::*,
    render_example_blocks,
    types::{
        enums::BorderStyle,
        structs::{GradientThemeSet, SplitBorderSegments},
        typedefinitions::GradientTheme,
    },
};
fn run(
    terminal: &mut DefaultTerminal,
    style: BorderStyle,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let gradblock = GradientBlock::new(
                &f.area(),
                SplitBorderSegments::empty(),
            )
            .border_style(style.clone())
            .set_gradients(generate_gradient_theme!(
                (BorderGradients {
                    left: vec![(48, 174, 209), (48, 174, 209)],
                    bottom: vec![(48, 174, 209), (48, 174, 209)],
                    right: vec![(225, 22, 247), (48, 174, 209)],
                    top: vec![(48, 174, 209), (225, 22, 247)],
                    left_fac: 1.0,
                    bottom_fac: 1.0,
                    right_fac: 1.0,
                    top_fac: 1.0,
                })
            ));
            f.render_widget(gradblock, f.area());
        })?;
        if let Err(e) = handle_events() {
            eprintln!("Error handling events: {}", e);
        }
    }
}

fn main() -> io::Result<()> {
    let style: BorderStyle;
    style = handle_args!();
    let _ = color_eyre::install();
    let mut terminal = ratatui::init();
    let app_result = run(&mut terminal, style);
    ratatui::restore();
    app_result
}

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
