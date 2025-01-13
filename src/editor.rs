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
    text_buffer::TedBuffer,
    editor_state::TedState,
    renderer,
};

pub(crate) const CURSOR_X_OFFSET: usize = 5;

pub struct Editor {
    buffer: TedBuffer,
    editor_state: TedState,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    column_cache: Option<usize>,    // if you press up/down continuosly, snap to the column you start from
}

impl Editor {

    pub fn new() -> crossterm::Result<Self> {
        let buffer = TedBuffer::new();
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Editor { 
            buffer, 
            editor_state: TedState::Insert,
            terminal, 
            column_cache: None 
        })
    }

    pub fn run(&mut self) -> crossterm::Result<()> {

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
                // let editor_chunk = chunks[1];
                let width = chunks[1].width as usize - CURSOR_X_OFFSET;
                let height = chunks[1].height as usize;

                let render_lines = renderer::render_text_buffer(&self.buffer, width, height);
                // let editor = renderer::render_text(&render_lines);
                frame.render_widget(render_lines, chunks[1]);
    
                // Render status bar
                let status = Paragraph::new(" Press 'ALT + Backspace' to quit")
                    .block(Block::default().borders(Borders::ALL));
                frame.render_widget(status, chunks[2]);
    
                // set the cursor position
                frame.set_cursor(
                    (CURSOR_X_OFFSET + self.buffer.cursor_x()) as u16, // x
                    u16::min(self.buffer.cursor_y() as u16, height as u16 - 1) + chunks[0].height // y
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
                self.editor_state = TedState::Save
            },

            // if any character has been pressed with any modifiers
            (KeyCode::Char(ch), key_mod) => {
                self.editor_state = TedState::Insert;
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

}