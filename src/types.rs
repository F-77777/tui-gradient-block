pub type G = Box<dyn colorgrad::Gradient>;
pub type E = Box<dyn std::error::Error>;
pub type T<'a> = (
    ratatui::text::Line<'a>,
    ratatui::widgets::block::title::Position,
);
