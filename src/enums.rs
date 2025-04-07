#[derive(Clone)]
/// contains the custom border types
/// Defines different border styles that can be applied.
pub enum BorderStyle {
    NewSet,
    CustomSet(crate::structs::border_symbols::SegmentSet),
    RatatuiSet(ratatui::symbols::border::Set),
}
