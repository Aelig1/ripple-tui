use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::{app::App, pond::Pond};

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        Pond::new().render(area, buf);
    }
}
