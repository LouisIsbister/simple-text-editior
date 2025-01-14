
pub struct TedBuffer {
    lines: Vec<String>,
    cx: usize,
    cy: usize,
    column_cache: Option<usize>  // if you press up/down continuosly, snap to the column you start from
}

impl TedBuffer {

    /// Creates a clean text buffer
    pub fn new() -> Self {
        Self { 
            lines: vec![String::new()],
            cx: 0,
            cy: 0,
            column_cache: None
        }
    }

    pub fn cursor_x(&self) -> usize {
        self.cx
    }

    pub fn cursor_y(&self) -> usize {
        self.cy
    }

    pub fn column_cache(&self) -> &Option<usize> {
        &self.column_cache
    }

    /// Cursor X and Y methods indexed from 1 as opposed 
    /// to being indexed from 0 (as they already are!)
    pub fn view_cursor_x(&self) -> usize {
        self.cx + 1
    }

    pub fn view_cursor_y(&self) -> usize {
        self.cy + 1
    }

    /// Functions to obtain line information
    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn lines_count(&self) -> usize {
        self.lines.len()
    }

    pub fn reset_column_cache(&mut self) {
        self.column_cache = None
    }

    /// Inserts a newline into the current cursor position. If the cursor 
    /// is not at the end of the current line, it will split the line buffer,
    /// keeping the first half on the current line and inserting the second
    /// half on a new line.
    pub fn insert_empty_line(&mut self) {
        if self.cy < self.lines.len() - 1 {
            let current_line = self.lines.remove(self.cy);
            let (before_cursor, after_cursor) = current_line.split_at(self.cx);
            self.lines.insert(self.cy, before_cursor.to_string());
            self.lines.insert(self.cy + 1, after_cursor.to_string());
        } else {
            self.lines.push(String::new());
        }
        self.cx = 0;
        self.cy += 1;
    }

    /// inserts the pressed character at the current cursor positon.
    pub fn insert_char(&mut self, ch: char) { 
        if self.cy < self.lines.len() {
            self.lines[self.cy].insert(self.cx, ch);
            self.cx += 1
        }
    }

    /// Backspace character:
    /// Deletes the next character to the left. If the current X position 
    /// is at 0 the the current lines contents should be concatenated
    /// onto the end of the line above! However, if Y is also 0, then 
    /// nothing can be updated. Otherwise, if X is not 0 then simply
    /// remove the current charcter from the current line.
    pub fn rm_char_backspace(&mut self) {
        if self.cx == 0 {
            if self.cy == 0 {
                return
            }
            let cur_line_buffer = self.lines.remove(self.cy);
            let prev_line = &mut self.lines[self.cy - 1];

            self.cy -= 1;
            self.cx = prev_line.len();

            prev_line.push_str(&cur_line_buffer);
        } else {
            self.lines[self.cy].remove(self.cx - 1);
            self.cx -= 1
        }
    }

    /// Delete character:
    /// Deletes the current character and shifts the rest of the line 
    /// to the left. If X is the endd of the current line, the is appends
    /// the contents of the next line onto the end of the current line
    /// and removes the next line.  
    pub fn rm_char_delete(&mut self) {
        if self.cx == self.lines[self.cy].len() {
            if self.cy >= self.lines.len() - 1 {
                return
            }

            let next_line_buffer = self.lines.remove(self.cy + 1);
            self.lines[self.cy].push_str(&next_line_buffer);
        } else {
            self.lines[self.cy].remove(self.cx);
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cy > 0 { 
            self.cy -= 1;
            self.cx = self.get_and_update_cached_x();
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cy < self.lines.len() - 1 {
            self.cy += 1;
            self.cx = self.get_and_update_cached_x();
        }
    }

    /// Computes the cursors next X position based upon the cached cursor X value stored 
    /// in the editor buffer (EBuffer). This is helpful when you repeatedly moving either 
    /// 'up' or 'down'. The result is that the cursor tries to get as close as possible to 
    /// the column it started on. If you are moving to a shorter line if will go to the 
    /// end of the line. Otherwise it will move to the orginal column, but on the newline!
    fn get_and_update_cached_x(&mut self) -> usize {
        if self.column_cache.is_none() {
            self.column_cache = Some(self.cx)
        }

        let next_row_len = self.lines[self.cy].len();
        usize::min(self.column_cache.unwrap(), next_row_len)
    }

    pub fn move_cursor_left(&mut self) {
        if self.cx > 0 {
            self.cx -= 1;
        } else if self.cx == 0 && self.cy > 0 { 
            // wrap the cursor to the end of the prev line
            self.cy -= 1;
            self.cx = self.lines[self.cy].len()
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cx < self.lines[self.cy].len() {
            self.cx += 1;
        } else if self.cx == self.lines[self.cy].len() && self.cy < self.lines.len() - 1 {
            // wrap the cursor to the start of the next line
            self.cy += 1;
            self.cx = 0
        }
    }

}
