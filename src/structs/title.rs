use ratatui::widgets::block::Title;
pub struct TitleSet<'a> {
    pub top: Title<'a>,
    pub bottom: Title<'a>,
    pub left: Title<'a>,
    pub right: Title<'a>,
    pub top_left: Title<'a>,
    pub top_right: Title<'a>,
    pub bottom_left: Title<'a>,
    pub bottom_right: Title<'a>,
    pub double_right: Title<'a>,
    pub double_left: Title<'a>,
    pub vertical: Title<'a>,
    pub horizontal: Title<'a>,
    pub base1: Title<'a>,
    pub base2: Title<'a>,
}
