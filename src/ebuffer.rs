
pub struct EBuffer {
    lines: Vec<Line>,
    cx: usize,
    cy: usize,
}

impl EBuffer {

    /// Creates a clean text buffer
    pub fn new() -> Self {
        Self { 
            lines: vec![Line::new()],
            cx: 0,
            cy: 0,
        }
    }

    pub fn lines(&self) -> &Vec<Line> {
        &self.lines
    }

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
            let current_line = self.lines.remove(self.cy).buffer;
            let (before_cursor, after_cursor) = current_line.split_at(self.cx);
            self.lines.insert(self.cy, Line::from(before_cursor));
            self.lines.insert(self.cy + 1, Line::from(after_cursor));
        } else {
            self.lines.push(Line::new());
        }
        self.cx = 0;
        self.cy += 1;
    }

    /// inserts the pressed character at the current cursor positon.
    pub fn insert_char(&mut self, ch: char) { 
        if self.cy < self.lines.len() {
            self.lines[self.cy].buffer.insert(self.cx, ch);
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
            let cur_line_buffer = self.lines.remove(self.cy).buffer;
            let prev_line = &mut self.lines[self.cy - 1];

            self.cy -= 1;
            self.cx = prev_line.len();

            prev_line.append_str(&cur_line_buffer);
        } else {
            self.lines[self.cy].buffer.remove(self.cx - 1);
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

            let next_line_buffer = self.lines.remove(self.cy + 1).buffer;
            self.lines[self.cy].append_str(&next_line_buffer);
        } else {
            self.lines[self.cy].buffer.remove(self.cx);
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cy > 0 {
            self.cy -= 1;
            let new_line_buffer = &self.lines[self.cy].buffer;

            // only update x if the prev line is longer than the newline
            if self.cx > new_line_buffer.len() {
                self.cx = new_line_buffer.len()
            }
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cy < self.lines.len() - 1 {
            self.cy += 1;
            let new_line_buffer = &self.lines[self.cy].buffer;
            // only update x if the prev line is longer than the newline
            if self.cx > new_line_buffer.len() {
                self.cx = new_line_buffer.len()
            }
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cx > 0 {
            self.cx -= 1;
        } else if self.cx == 0 && self.cy > 0 { 
            // wrap the cursor to the end of the prev line
            self.cy -= 1;
            self.cx = self.lines[self.cy].buffer.len()
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cx < self.lines[self.cy].buffer.len() {
            self.cx += 1;
        } else if self.cx == self.lines[self.cy].buffer.len() && self.cy < self.lines.len() - 1 {
            // wrap the cursor to the start of the next line
            self.cy += 1;
            self.cx = 0
        }
    }

}


#[derive(Debug)]
pub struct Line {
    buffer: String
}

impl Line {
    pub fn new() -> Self {
        Self { 
            buffer: String::new() 
        }
    }

    pub fn from(s: &str) -> Self {
        Self {
            buffer: String::from(s)
        }
    }

    pub fn buffer(&self) -> &String {
        &self.buffer
    }


    pub fn append_str(&mut self, str: &String) {
        self.buffer.push_str(str);
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

}