use crossterm::event::{self, *};
use std::io;
use tui_gradient_block::{
    handle_args, structs::border_symbols::SegmentSet,
};
use tui_rule::create_raw_spans;
fn main() -> io::Result<()> {
    let arg = handle_args!();
    let style = SegmentSet::from_ratatui_set(arg);
    let mut terminal = ratatui::init();
    let app_result = run(&mut terminal, style);
    ratatui::restore();
    app_result
}
fn run(
    terminal: &mut ratatui::DefaultTerminal,
    set: SegmentSet,
) -> io::Result<()> {
    use ratatui::{text::Line, layout::{Constraint, Direction, Layout}};
    use tui_gradient_block::{
        gradient_block::GradientBlock,
        theme_presets::cool::t_zombie_dreams,
    };
    let theme = t_zombie_dreams::full();
    let blocks = vec![
        GradientBlock::new()
            .with_gradient(theme.double_corners_left)
            .with_set(set.clone()),
        GradientBlock::new().title_top(Line::from(tui_rule::generate_gradient_text!("Zombie Dreams", theme.right.top)).centered())
            .with_gradient(theme.misc1)
            .with_set(set.clone()),
        GradientBlock::new()
            .with_gradient(theme.double_corners_right)
            .with_set(set.clone()),
    ];

    loop {
        terminal.draw(|f| {
            let base = Layout::new(
                Direction::Horizontal,
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ],
            )
            .vertical_margin(1)
            .horizontal_margin(1)
            .spacing(3)
            .split(f.area());
            for (block, area) in blocks.iter().zip(base.iter()) {
                f.render_widget(block, *area);
            }
        })?;
        let event = event::read()?;

        if let Event::Key(key_event) = event {
            if key_event.kind == KeyEventKind::Press {
                if let KeyCode::Char('q') = key_event.code {
                    break Ok(());
                }
            }
        }
    }
}
