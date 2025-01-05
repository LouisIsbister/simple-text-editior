mod editor_buffer;

use editor_buffer::Buffer;

use crossterm::{
    event::{
        self, Event, 
        KeyCode, KeyEvent, KeyEventKind, KeyModifiers
    }, execute, terminal::{
        disable_raw_mode,
        enable_raw_mode,
        DisableLineWrap, EnableLineWrap, 
        EnterAlternateScreen, LeaveAlternateScreen
    }
};

use tui::{
    backend::CrosstermBackend, 
    layout::{
        Constraint, 
        Direction, 
        Layout
    }, 
    widgets::{
        Block, 
        Borders, 
        Paragraph
    }, 
    Terminal
};

fn main() -> crossterm::Result<()> {
    let stdout = std::io::stdout();
    enable_raw_mode()?;
    execute!(&stdout, EnterAlternateScreen, DisableLineWrap)?;
    
    
    let backend = CrosstermBackend::new(&stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut buffer = Buffer::new();

    loop {
        if !handle_input(&mut buffer)? {
            break
        }
        
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [Constraint::Min(3), Constraint::Length(3)]
                        .as_ref(),
                )
                .split(frame.size());

            // Render editor
            let editor = render_editor(&buffer);
            frame.render_widget(editor, chunks[0]);

            // Render status bar
            let status = Paragraph::new("Press 'ALT + Backspace' to quit")
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(status, chunks[1]);

            // set the cursor position
            frame.set_cursor(6 + buffer.cursor_x() as u16, 1 + buffer.cursor_y() as u16);
        })?;       
    }

    execute!(&stdout, LeaveAlternateScreen, EnableLineWrap)?;
    disable_raw_mode()?;
    Ok(())
}

fn handle_input(buffer: &mut Buffer) -> Result<bool, std::io::Error> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                return key_handler(buffer, &key_event);
            }
        }
    }
    Ok(true)
}

fn key_handler(buffer: &mut Buffer, event: &KeyEvent) -> Result<bool, std::io::Error> {
    match (event.code, event.modifiers) {
        // return Err(Error::new(io::ErrorKind::Interrupted, "Quit editor!"));
        (KeyCode::Backspace, KeyModifiers::ALT) => return Ok(false),
        
        (KeyCode::Char(ch), key_mod) => {
            match key_mod {
                KeyModifiers::NONE => buffer.insert_char(ch),
                KeyModifiers::SHIFT => buffer.insert_char(ch.to_ascii_uppercase()),
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
    Ok(true)
}

fn render_editor(buffer: &Buffer) -> Paragraph {
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
