use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

/// A rippling body of water.
#[derive(Debug)]
pub struct Pond {
    /// The aspect ratio of a terminal character/cell. Height divided by width.
    ///
    /// Defaults to `2.0`.
    pub cell_aspect: f64,
}

impl Pond {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Pond {
    fn default() -> Self {
        Self { cell_aspect: 2.0 }
    }
}

impl Widget for Pond {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let center_x = area.x + area.width / 2;
        let center_y = area.y + area.height / 2;
        buf[(center_x, center_y)].set_char('R');
    }
}
