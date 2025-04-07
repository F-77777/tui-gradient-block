use colorgrad::{Color, GradientBuilder};
use crossterm::event::{self, *};
use std::io;
use tui_gradient_block::{
    types::G,
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
fn solid(col: (u8, u8, u8)) -> G {
    Box::new(
        GradientBuilder::new()
            .colors(&[Color::from_rgba8(col.0, col.1, col.2, 1)])
            .build::<colorgrad::LinearGradient>()
            .unwrap(),
    )
}
fn run(
    terminal: &mut ratatui::DefaultTerminal,
    set: SegmentSet,
) -> io::Result<()> {
    use tui_gradient_block::gradient_block::GradientBlock;
    let block = GradientBlock::new()
        .with_set(set)
        .left_gradient(solid((48, 174, 209)))
        .bottom_gradient(solid((48, 174, 209)))
        .top_gradient(Box::new(
            GradientBuilder::new()
                .colors(&[
                    Color::from_rgba8(48, 174, 209, 1),
                    Color::from_rgba8(225, 22, 247, 1),
                ])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ))
        .right_gradient(Box::new(
            GradientBuilder::new()
                .colors(&[
                    Color::from_rgba8(225, 22, 247, 1),
                    Color::from_rgba8(48, 174, 209, 1),
                ])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
        ));

    loop {
        terminal.draw(|f| f.render_widget(&block, f.area()))?;
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
