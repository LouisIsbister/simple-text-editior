use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

use crate::editor::{CURSOR_X_OFFSET, TITLE_BOX_HEIGHT};
use crate::model::text_buffer::TedBuffer;

pub struct BufferRenderer {
    start_x: usize,
    start_y: usize,
}

impl BufferRenderer {
    pub fn new() -> Self {
        Self {
            start_x: 0,
            start_y: 0,
        }
    }

    pub fn update_view_window(
        &mut self,
        ed_width: usize, // current width of the editor
        ed_height: usize, // current heigth of the editor
        buffer: &TedBuffer,
    ) {
        // update the start and nd y line corrdinates
        if buffer.cursor_y() >= self.start_y + ed_height {
            self.start_y = buffer.view_cursor_y() - ed_height
        } else if buffer.cursor_y() < self.start_y {
            self.start_y = buffer.cursor_y();
        }

        if buffer.cursor_x() >= self.start_x + ed_width {
            self.start_x = buffer.view_cursor_x() - ed_width
        } else if buffer.cursor_x() < self.start_x {
            self.start_x -= 1;
        }
    }

    /// Get the lines that should be rendered based upon the current height
    /// of the editor. If the current X position of the cursor is greater
    /// than the width of the editor, then 'slide' the viewport to the right,
    /// only rendering the contents that can fit on the window!  
    pub fn render_text_buffer(
        &mut self,
        buffer: &TedBuffer,
        ed_height: usize,
    ) -> Paragraph {
        // 'slide' the window based upon the current cursor X pos
        let end_y = usize::min(buffer.lines_count(), self.start_y + ed_height);
        let mut view = buffer.lines()[self.start_y..end_y].to_vec();

        if self.start_x > 0 {
            // let diff = buffer.view_cursor_x() - ed_width;
            for line in &mut view {
                if self.start_x < line.len() {
                    *line = line[self.start_x..line.len()].to_string();
                } else {
                    *line = String::new()
                }
            }
        }

        let lines: Vec<Spans> = view
            .iter()
            .enumerate()
            .map(|(idx, lbuff)| {
                let line_num_as_string = (idx + self.start_y + 1).to_string();
                let space_count = CURSOR_X_OFFSET - line_num_as_string.len();
                let spaces = " ".repeat(space_count);

                Spans::from(Span::from(format!(
                    "{}{}{}",
                    line_num_as_string, spaces, lbuff
                )))
            })
            .collect();
        Paragraph::new(lines)
    }

    pub fn view_cursor_position_on_buffer(&self, buffer: &TedBuffer) -> (u16, u16) {
        let cx = buffer.cursor_x() - self.start_x;
        let cy = buffer.cursor_y() - self.start_y;
        
        ((CURSOR_X_OFFSET + cx) as u16, TITLE_BOX_HEIGHT + cy as u16)
    }

}

pub fn render_title(fname: &String) -> Paragraph<'static> {
    // Render title
    let title = format!(" Editing '{}' ~ [So productivveee]", fname);
    Paragraph::new(title).block(Block::default().borders(Borders::ALL))
}
