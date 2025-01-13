use tui::text::{Span, Spans};
use tui::widgets::Paragraph;

use crate::editor::CURSOR_X_OFFSET;
use crate::text_buffer::TedBuffer;

/// Get the lines that should be rendered based upon the current height
/// of the editor. If the current X position of the cursor is greater
/// than the width of the editor, then 'slide' the viewport to the right,
/// only rendering the contents that can fit on the window!  
pub fn render_text_buffer(
    buffer: &TedBuffer,
    ed_width: usize,
    ed_height: usize,
) -> Paragraph {
    let start_line = match buffer.lines_count() <= ed_height {
        true => 0,
        false => buffer.lines_count() - ed_height,
    };

    // 'slide' the window based upon the current cursor X pos 
    let mut view = buffer.lines()[start_line..].to_vec();
    if buffer.view_cursor_x() >= ed_width {
        let diff = buffer.view_cursor_x() - ed_width;
        for line in &mut view {
            if diff < line.len() {
                *line = line[diff..line.len()].to_string();
            } else {
                *line = String::new()
            }
        }
    }

    let lines: Vec<Spans> = view.iter()
        .enumerate()
        .map(|(idx, lbuff)| {
            let line_num_as_string = (idx + start_line + 1).to_string();
            let space_count = CURSOR_X_OFFSET - line_num_as_string.len();
            let spaces = " ".repeat(space_count);

            Spans::from(Span::from(format!("{}{}{}", line_num_as_string, spaces, lbuff)))
        })
        .collect();
    Paragraph::new(lines)
}

