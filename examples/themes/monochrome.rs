use crossterm::event::{self, *};
use std::io;
use tui_gradient_block::{
    handle_args, structs::border_symbols::SegmentSet,
};
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
    use ratatui::layout::{Constraint, Direction, Layout};
    use tui_gradient_block::{
        gradient_block::GradientBlock,
        theme_presets::misc::t_monochrome,
    };
    let titles = t_monochrome::titles();
    let theme = t_monochrome::full();
    let blocks_top = vec![
        GradientBlock::new()
            .title(titles.up.0, titles.up.1)
            .with_gradient(theme.up)
            .with_set(set.clone()),
        GradientBlock::new()
            .title(titles.down.0, titles.down.1)
            .with_gradient(theme.down)
            .with_set(set.clone()),
        GradientBlock::new()
            .title(titles.left.0, titles.left.1)
            .with_gradient(theme.left)
            .with_set(set.clone()),
        GradientBlock::new()
            .title(titles.right.0, titles.right.1)
            .with_gradient(theme.right)
            .with_set(set.clone()),
        GradientBlock::new()
            .title(titles.top_left.0, titles.top_left.1)
            .with_gradient(theme.top_left)
            .with_set(set.clone()),
        GradientBlock::new()
            .title(titles.top_right.0, titles.top_right.1)
            .with_gradient(theme.top_right)
            .with_set(set.clone()),
        GradientBlock::new()
            .title(titles.bottom_left.0, titles.bottom_left.1)
            .with_gradient(theme.bottom_left)
            .with_set(set.clone()),
    ];
    let blocks_bottom = vec![
        GradientBlock::new()
            .title(titles.bottom_right.0, titles.bottom_right.1)
            .with_gradient(theme.bottom_right)
            .with_set(set.clone()),
        GradientBlock::new()
            .title(
                titles.double_corners_left.0,
                titles.double_corners_left.1,
            )
            .with_gradient(theme.double_corners_left)
            .with_set(set.clone()),
        GradientBlock::new()
            .title(
                titles.double_corners_right.0,
                titles.double_corners_right.1,
            )
            .with_gradient(theme.double_corners_right)
            .with_set(set.clone()),
        GradientBlock::new()
            .title(titles.vertical.0, titles.vertical.1)
            .with_gradient(theme.vertical)
            .with_set(set.clone()),
        GradientBlock::new()
            .title(titles.horizontal.0, titles.horizontal.1)
            .with_gradient(theme.horizontal)
            .with_set(set.clone()),
        GradientBlock::new()
            .title(titles.misc1.0, titles.misc1.1)
            .with_gradient(theme.misc1)
            .with_set(set.clone()),
        GradientBlock::new()
            .title(titles.misc2.0, titles.misc2.1)
            .with_gradient(theme.misc2)
            .with_set(set.clone()),
    ];

    loop {
        terminal.draw(|f| {
            let base = Layout::new(
                Direction::Vertical,
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ],
            )
            .split(f.area());
            let top = Layout::new(
                Direction::Horizontal,
                [
                    Constraint::Percentage(14),
                    Constraint::Percentage(14),
                    Constraint::Percentage(14),
                    Constraint::Percentage(14),
                    Constraint::Percentage(14),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                ],
            )
            .split(base[0]);
            let bottom = Layout::new(
                Direction::Horizontal,
                [
                    Constraint::Percentage(14),
                    Constraint::Percentage(14),
                    Constraint::Percentage(14),
                    Constraint::Percentage(14),
                    Constraint::Percentage(14),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                ],
            )
            .split(base[1]);
            for (block, area) in blocks_top.iter().zip(top.iter()) {
                f.render_widget(block, *area);
            }
            for (block, area) in
                blocks_bottom.iter().zip(bottom.iter())
            {
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
