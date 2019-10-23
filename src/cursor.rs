use tui::{
    widgets::{Widget},
    layout::Rect,
    buffer::Buffer,
    style::Color,
};

pub struct Cursor {
    pub x: u16,
    pub y: u16,
}

impl Widget for Cursor {
    fn draw(&mut self, area: Rect, buffer: &mut Buffer) {
        let x = self.x.saturating_add(area.x);
        let y = self.y.saturating_add(area.y);

        buffer.get_mut(x, y)
        	.set_bg(Color::White);

    }
}
