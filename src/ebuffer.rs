
pub struct EBuffer {
    lines: Vec<String>,
    cx: usize,
    cy: usize,
}

impl EBuffer {

    /// Creates a clean text buffer
    pub fn new() -> Self {
        Self { 
            lines: vec![String::new()],
            cx: 0,
            cy: 0,
        }
    }

    // pub fn lines(&self) -> &Vec<String> {
    //     &self.lines
    // }
    // pub fn line_count(&self) -> usize {
    //     self.lines.len()
    // }

    pub fn cursor_x(&self) -> usize {
        self.cx
    }

    pub fn cursor_y(&self) -> usize {
        self.cy
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

    pub fn move_cursor_up(&mut self, column_cache: &mut Option<usize>) {
        if self.cy > 0 {
            self.cy -= 1;
            let new_line_buffer = &self.lines[self.cy];

            self.cx = self.get_and_update_cached_x(column_cache, &new_line_buffer);
        }
    }

    pub fn move_cursor_down(&mut self, column_cache: &mut Option<usize>) {
        if self.cy < self.lines.len() - 1 {
            self.cy += 1;
            let new_line_buffer = &self.lines[self.cy];
            
            self.cx = self.get_and_update_cached_x(column_cache, &new_line_buffer);
        }
    }

    /// Computes the cursors next X position based upon the cached cursor X value stored 
    /// in the editor buffer (EBuffer). This is helpful when you repeatedly moving either 
    /// 'up' or 'down'. The result is that the cursor tries to get as close as possible to 
    /// the column it started on. If you are moving to a shorter line if will go to the 
    /// end of the line. Otherwise it will move to the orginal column, but on the newline!
    fn get_and_update_cached_x(&self, column_cache: &mut Option<usize>, next_row: &String) -> usize {
        if column_cache.is_none() {
            *column_cache = Some(self.cx)
        }

        usize::min(column_cache.unwrap(), next_row.len())
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

    pub fn display_lines(&self, num_lines_to_display: usize) -> (&[String], usize) {
        match self.lines.len() > num_lines_to_display {
            true => {
                let start = self.lines.len() - num_lines_to_display;
                (&self.lines[start..], start)
            },
            false => (&self.lines, 0)
        }
    }

}
