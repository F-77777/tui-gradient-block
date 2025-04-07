pub use crate::{
    border::*,
    border_styles::*,
    buffer, enums, get_aligned_position, handle_fill,
    prelude::{self, Rect as R, Widget},
    structs::{
        self, border_segment, border_symbols,
        border_symbols::SegmentSet as SS, gradient,
    },
    style::{Color, Style},
    text::{self, Line},
    types::{G, T},
    widgets::{
        self,
        block::{self, title::Position},
        Block, Borders, Paragraph, WidgetRef,
    },
};
use std::rc::Rc;
/// A struct that represents a customizable block with gradient text, borders, and other visual elements.
///
/// This struct allows you to create and manage blocks that have a gradient color effect for text,
/// customizable borders, and areas with specific alignments and fill styles.
pub struct GradientBlock<'a> {
    pub fill: Line<'a>,
    pub titles: Vec<T<'a>>,
    pub bg: Option<Color>,
    pub border_segments: border_segment::BorderSegments,
}

impl Default for GradientBlock<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl GradientBlock<'_> {
    pub fn new() -> Self {
        Self {
            fill: Line::raw(""),
            titles: Vec::new(),
            bg: None,
            border_segments: border_segment::BorderSegments::new(),
        }
    }
    /// Sets the border line segments based on the area and border symbols.
    fn render_block(&self, area: Rc<R>, buf: &mut buffer::Buffer) {
        if self.border_segments.left.should_be_rendered {
            Self::render_left(self, *area, buf);
        }
        if self.border_segments.right.should_be_rendered {
            Self::render_right(self, *area, buf);
        }
        if self.border_segments.top.should_be_rendered {
            Self::render_top(self, *area, buf);
        }
        if self.border_segments.bottom.should_be_rendered {
            Self::render_bottom(self, *area, buf);
        }
    }
    /// Renders the top segment of the border with an optional gradient
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +     +
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +-----+
    /// ```
    fn render_top(&self, area: R, buf: &mut buffer::Buffer) {
        self.border_segments.top.seg.render_ref(area, buf);
    }

    /// Renders the left segment of the border with an optional gradient
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +-----+
    ///       |
    ///       |
    ///       |
    ///       +
    ///       |
    ///       |
    /// +-----+
    /// ```
    fn render_left(&self, area: R, buf: &mut buffer::Buffer) {
        self.border_segments.left.seg.render_ref(area, buf);
    }

    /// Renders the bottom segment of the border with an optional gradient
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +--+--+
    /// |     |
    /// |     |
    /// +     +
    /// |     |
    /// |     |
    /// +     +
    /// ````
    fn render_bottom(&self, area: R, buf: &mut buffer::Buffer) {
        self.border_segments.bottom.seg.render_ref(area, buf);
    }

    /// Renders the right segment of the border with an optional gradient
    /// ## Visual Representation:
    /// Without the function:
    /// ```
    /// +--+--+
    /// |     
    /// |     
    /// +     
    /// |     
    /// |     
    /// +--+--+
    /// ```
    fn render_right(&self, area: R, buf: &mut buffer::Buffer) {
        self.border_segments.right.seg.render_ref(area, buf);
    }

    /// Renders the titles for the widget, with an optional gradient
    fn render_titles(&self, area: Rc<R>, buf: &mut buffer::Buffer) {
        for (title, pos) in &self.titles {
            let padding = match pos {
                Position::Top => self.border_segments.top.seg.padding,
                Position::Bottom => {
                    self.border_segments.bottom.seg.padding
                }
            };
            let marg = self.border_segments.top.seg.area_margin;
            let x = get_aligned_position!(
                *area,
                title.alignment,
                title.width() as u16,
                padding.left,
                padding.right
            )
            .saturating_add(marg.horizontal / 2);
            let y = match pos {
                Position::Top => area
                    .top()
                    .saturating_add(padding.top)
                    .saturating_add(marg.horizontal),

                Position::Bottom => area
                    .bottom()
                    .saturating_sub(padding.bottom)
                    .saturating_sub(marg.vertical),
            };

            buf.set_line(x, y, title, area.width);
        }
    }

    /// Renders the fill for the widget, including optional gradient rendering.
    fn render_fill(&self, area: Rc<R>, buf: &mut buffer::Buffer) {
        Paragraph::new(self.fill.clone())
            .wrap(widgets::Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL))
            .render(*area, buf);
    }

    /// Renders the `Gradientblock` widget, including optional fill and custom block rendering,
    /// along with titles.
    pub fn main(
        &self,
        area: &prelude::Rect,
        buf: &mut buffer::Buffer,
    ) {
        let area_rc = Rc::new(*area);
        if !self.fill.spans.is_empty() {
            self.render_fill(Rc::clone(&area_rc), buf);
        }
        self.render_block(Rc::clone(&area_rc), buf);
        self.render_titles(Rc::clone(&area_rc), buf);
        if let Some(bg) = self.bg {
            buf.set_style(*(Rc::clone(&area_rc)), bg);
        }
    }
}

impl widgets::Widget for GradientBlock<'_> {
    /// Renders the `Gradientblock` widget using the `main` function.
    fn render(self, area: prelude::Rect, buf: &mut buffer::Buffer) {
        self.render_ref(area, buf);
    }
}
impl widgets::WidgetRef for GradientBlock<'_> {
    fn render_ref(&self, area: R, buf: &mut buffer::Buffer) {
        self.main(&area, buf);
    }
}
