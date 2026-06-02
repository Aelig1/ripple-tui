use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

/// A rippling body of water.
#[derive(Debug)]
pub struct Pond {
    /// The aspect ratio of a terminal character/cell. Height divided by width.
    ///
    /// Defaults to `2.0`.
    pub cell_aspect: f64,

    /// Ripple x coordinate just for testing things out.
    debug_x: u16,
    /// Ripple y coordinate just for testing things out.
    debug_y: u16,
}

impl Pond {
    pub fn new() -> Self {
        Self::default()
    }

    /// Drops a droplet at the given terminal-cell coordinates to start a ripple.
    pub fn droplet(&mut self, x: u16, y: u16) {
        self.debug_x = x;
        self.debug_y = y;
    }
}

impl Default for Pond {
    fn default() -> Self {
        Self {
            cell_aspect: 2.0,
            debug_x: 0,
            debug_y: 0,
        }
    }
}

impl Widget for &Pond {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if is_within_area(area, self.debug_x, self.debug_y) {
            buf[(self.debug_x, self.debug_y)].set_char('R');
        }
    }
}

fn is_within_area(area: Rect, x: u16, y: u16) -> bool {
    x >= area.left() && x < area.right() && y >= area.top() && y < area.bottom()
}
