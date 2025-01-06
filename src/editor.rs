use std::io::Stdout;
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};
use tui::{
    backend::CrosstermBackend, 
    layout::{Constraint, Direction, Layout}, 
    text::{Span, Spans}, 
    widgets::{Block, Borders, Paragraph}, 
    Terminal
};

use crate::text_buffer::TBuffer;
use crate::editor_state::EState;

pub struct Editor {
    buffer: TBuffer,
    editor_state: EState,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    column_cache: Option<usize>,    // if you press up/down continuosly, snap to the column you start from
}

impl Editor {

    pub fn new() -> crossterm::Result<Self> {
        let buffer = TBuffer::new();
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Editor { 
            buffer, 
            editor_state: EState::Insert,
            terminal, 
            column_cache: None 
        })
    }

    pub fn run(&mut self) -> crossterm::Result<()>{
        const CURSOR_X_OFFSET: u16 = 5;

        loop {
            if !self.handle_input()? {
                break
            }
            
            self.terminal.draw(|frame| {
                let status_section_height: u16 = 3;                
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(status_section_height),
                            Constraint::Min(1),  // text section can be a minimum of 1 line
                            Constraint::Length(status_section_height)
                        ].as_ref(),
                    )
                    .split(frame.size());
    
                // Render title
                let status = Paragraph::new(" - Editor - ")
                    .block(Block::default().borders(Borders::ALL));
                frame.render_widget(status, chunks[0]);

                // Render editor
                let editor_chunk = chunks[1];
                let width = editor_chunk.width;
                let height = editor_chunk.height;

                let render_lines = self.buffer.display_lines(height as usize);
                let editor = Editor::render_current_text_contents(render_lines, width as usize);
                frame.render_widget(editor, chunks[1]);
    
                // Render status bar
                let status = Paragraph::new(" Press 'ALT + Backspace' to quit")
                    .block(Block::default().borders(Borders::ALL));
                frame.render_widget(status, chunks[2]);
    
                // set the cursor position
                frame.set_cursor(
                    CURSOR_X_OFFSET + self.buffer.cursor_x() as u16,
                    u16::min(self.buffer.cursor_y() as u16, height - 1) + chunks[0].height
                );
            })?;
        }
        Ok(())
    }

    
    fn handle_input(&mut self) -> Result<bool, std::io::Error> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    return self.key_handler(&key_event);
                }
            }
        }
        Ok(true)
    }

    fn key_handler(&mut self, event: &KeyEvent) -> Result<bool, std::io::Error> {
        let buffer = &mut self.buffer;

        match (event.code, event.modifiers) {
            // ALT + backspace, exit the programs
            (KeyCode::Backspace, KeyModifiers::ALT) => return Ok(false),

            // CTRL + 's' 
            (KeyCode::Char(c), KeyModifiers::CONTROL) if c.to_ascii_lowercase() == 's' => {
                self.editor_state = EState::Save
            },

            // if any character has been pressed with any modifiers
            (KeyCode::Char(ch), key_mod) => {
                self.editor_state = EState::Insert;
                match key_mod {
                    KeyModifiers::NONE => buffer.insert_char(ch),
                    KeyModifiers::SHIFT => buffer.insert_char(ch.to_ascii_uppercase()),
                    _ => () // invalid modifier
                }
            },
            
            (KeyCode::Enter, KeyModifiers::NONE) => buffer.insert_empty_line(),
            (KeyCode::Backspace, KeyModifiers::NONE) => buffer.rm_char_backspace(),
            (KeyCode::Delete, KeyModifiers::NONE) => buffer.rm_char_delete(),
            (KeyCode::Up, KeyModifiers::NONE) => buffer.move_cursor_up(&mut self.column_cache),
            (KeyCode::Down, KeyModifiers::NONE) => buffer.move_cursor_down(&mut self.column_cache),
            (KeyCode::Left, KeyModifiers::NONE) => buffer.move_cursor_left(),
            (KeyCode::Right, KeyModifiers::NONE) => buffer.move_cursor_right(),
            _ => {}
        };  

        // update cache if the key pressed is not up or down, and it isn't already None
        if event.code != KeyCode::Up && event.code != KeyCode::Down && self.column_cache.is_some() {
            self.column_cache = None
        }

        Ok(true)
    }

    /// Render the lines that should be visible in the text editor.
    /// 
    /// TODO, if the current line is off the page, either wrap the tex onto a
    /// new line of shift all lines to the left in the viewport!
    fn render_current_text_contents<'a>(render_lines: (&'a [String], usize), line_width: usize) -> Paragraph<'a> {
        let idx_offset = render_lines.1;
        let lines: Vec<Spans> = render_lines.0
            .iter()
            .enumerate()
            .map(|(idx, lbuff)| {
                let idx_as_str = (idx + idx_offset + 1).to_string();
                let space_count = 5 - idx_as_str.len();
                let spaces  = " ".repeat(space_count);


                
                Spans::from(Span::from(format!("{}{}{}", idx_as_str, spaces, lbuff)))
            })
            .collect();

        Paragraph::new(lines)
    }
}