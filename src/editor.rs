use std::io::Stdout;
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};
use tui::{
    backend::CrosstermBackend, 
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph}, 
    Terminal
};

use crate::{
    model::{
        editor_state::TedState, 
        text_buffer::TedBuffer
    },
    view
};

pub(crate) const CURSOR_X_OFFSET: usize = 5;
pub(crate) const TITLE_BOX_HEIGHT: u16 = 3;

pub struct Editor {
    file_name: String,
    buffer: TedBuffer,
    editor_state: TedState,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Editor {

    pub fn new(file_name: String) -> crossterm::Result<Self> {
        let buffer = TedBuffer::new();
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Editor {
            file_name,
            buffer, 
            editor_state: TedState::Insert,
            terminal,
        })
    }

    pub fn run(&mut self) -> crossterm::Result<()> {

        let mut editor_view = view::view_components::BufferRenderer::new();

        loop {
            if self.handle_input()? == 0x01 {
                break
            }

            self.terminal.draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(TITLE_BOX_HEIGHT),
                            Constraint::Min(1),  // text section can be a minimum of 1 line
                            Constraint::Length(TITLE_BOX_HEIGHT)
                        ].as_ref(),
                    )
                    .split(frame.size());
    
                // Render title
                let header = view::view_components::render_title(&self.file_name);
                frame.render_widget(header, chunks[0]);

                // Render editor
                let width = chunks[1].width as usize - CURSOR_X_OFFSET;
                let height = chunks[1].height as usize;

                editor_view.update_view_window(width, height, &self.buffer);
                let render_lines = editor_view.render_text_buffer(&self.buffer, height);
                frame.render_widget(render_lines, chunks[1]);
    
                // Render status bar
                let status = Paragraph::new(" Press 'ALT + Backspace' to quit")
                    .block(Block::default().borders(Borders::ALL));
                frame.render_widget(status, chunks[2]);

                // update the cursor position
                let (cx, cy) = editor_view.view_cursor_position_on_buffer(&self.buffer);
                frame.set_cursor(cx, cy);
            })?;
        }
        Ok(())
    }

    
    fn handle_input(&mut self) -> Result<u8, std::io::Error> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    return self.key_handler(&key_event);
                }
            }
        }
        Ok(0x00)
    }

    fn key_handler(&mut self, event: &KeyEvent) -> Result<u8, std::io::Error> {
        let buffer = &mut self.buffer;

        match (event.code, event.modifiers) {
            // ALT + backspace, exit the programs
            (KeyCode::Backspace, KeyModifiers::ALT) => return Ok(0x01),

            // CTRL + 's' || CTRL + 'S'
            (KeyCode::Char(c), KeyModifiers::CONTROL) if c == 's' || c == 'S' => {
                self.editor_state = TedState::Save
            },

            // if any character has been pressed with any modifiers
            (KeyCode::Char(ch), key_mod) => {
                self.editor_state = TedState::Insert;
                match key_mod {
                    KeyModifiers::NONE => buffer.insert_char(ch),
                    KeyModifiers::SHIFT => {
                        if let Some(val) = ch.to_uppercase().next() {
                            buffer.insert_char(val);
                        }
                    },
                    _ => () // invalid modifier
                }
            },
            
            (KeyCode::Enter, KeyModifiers::NONE) => buffer.insert_empty_line(),
            (KeyCode::Backspace, KeyModifiers::NONE) => buffer.rm_char_backspace(),
            (KeyCode::Delete, KeyModifiers::NONE) => buffer.rm_char_delete(),
            (KeyCode::Up, KeyModifiers::NONE) => buffer.move_cursor_up(),
            (KeyCode::Down, KeyModifiers::NONE) => buffer.move_cursor_down(),
            (KeyCode::Left, KeyModifiers::NONE) => buffer.move_cursor_left(),
            (KeyCode::Right, KeyModifiers::NONE) => buffer.move_cursor_right(),
            _ => {}
        };  

        // update cache if the key pressed is not up or down, and it isn't already None
        if event.code != KeyCode::Up && event.code != KeyCode::Down && self.buffer.column_cache().is_some() {
            self.buffer.reset_column_cache()
        }

        Ok(0x00)
    }

}