use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};
use tui::{
    backend::CrosstermBackend, 
    layout::{Constraint, Direction, Layout}, 
    widgets::{Block, Borders, Paragraph}, 
    Terminal
};
use std::io::Stdout;

use crate::ebuffer::EBuffer;

// consts
const CURSOR_X_OFFSET: u16 = 6;
const CURSOR_Y_OFFSET: u16 = 1;

pub struct Editor {
    buffer: EBuffer,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    prev_key: Option<(KeyEvent, usize)>   // previous key and its cursor column 
}

impl Editor {

    pub fn new() -> crossterm::Result<Self> {
        let buffer = EBuffer::new();
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Editor { buffer, terminal, prev_key: None })
    }

    pub fn run(&mut self) -> crossterm::Result<()>{
        loop {
            if !self.handle_input()? {
                break
            }
            
            self.terminal.draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [Constraint::Min(3), Constraint::Length(3)]
                            .as_ref(),
                    )
                    .split(frame.size());
    
                // Render editor
                let editor = Editor::render(&self.buffer);
                frame.render_widget(editor, chunks[0]);
    
                // Render status bar
                let status = Paragraph::new("Press 'ALT + Backspace' to quit")
                    .block(Block::default().borders(Borders::ALL));
                frame.render_widget(status, chunks[1]);
    
                // set the cursor position
                frame.set_cursor(
                    CURSOR_X_OFFSET + self.buffer.cursor_x() as u16,
                    CURSOR_Y_OFFSET + self.buffer.cursor_y() as u16
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
        match (event.code, event.modifiers) {
            // return Err(Error::new(io::ErrorKind::Interrupted, "Quit editor!"));
            (KeyCode::Backspace, KeyModifiers::ALT) => return Ok(false),
            
            (KeyCode::Char(ch), key_mod) => {
                match key_mod {
                    KeyModifiers::NONE => self.buffer.insert_char(ch),
                    KeyModifiers::SHIFT => self.buffer.insert_char(ch.to_ascii_uppercase()),
                    _ => () // invalid modifier
                }
            },
    
            (KeyCode::Enter, KeyModifiers::NONE) => self.buffer.insert_empty_line(),
            (KeyCode::Backspace, KeyModifiers::NONE) => self.buffer.rm_char_backspace(),
            (KeyCode::Delete, KeyModifiers::NONE) => self.buffer.rm_char_delete(),
            (KeyCode::Up, KeyModifiers::NONE) => self.buffer.move_cursor_up(),
            (KeyCode::Down, KeyModifiers::NONE) => self.buffer.move_cursor_down(),
            (KeyCode::Left, KeyModifiers::NONE) => self.buffer.move_cursor_left(),
            (KeyCode::Right, KeyModifiers::NONE) => self.buffer.move_cursor_right(),
            _ => {}
        };
        self.prev_key = Some((*event, self.buffer.cursor_x()));
        Ok(true)
    }

    fn render(buffer: &EBuffer) -> Paragraph {
        let content = buffer.lines()
            .iter()
            .enumerate()
            .map(|(idx, l)| {
                let mut idx_as_str = (idx + 1).to_string();
                let space_count = 5 - idx_as_str.len();
                idx_as_str.push_str(" ".repeat(space_count).as_str());
    
                idx_as_str.push_str(&l.buffer());
                idx_as_str
            })
            .collect::<Vec<String>>()
            .join("\n");
        Paragraph::new(content)
            .block(Block::default().title("Editor").borders(Borders::ALL))
    }
}