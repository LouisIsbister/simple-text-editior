mod editor_buffer;

use editor_buffer::Buffer;

use crossterm::{
    execute,
    event::{
        self, Event, 
        KeyCode, KeyEvent, KeyEventKind
    }, terminal::{
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
            let status = Paragraph::new("Press 'q' to quit")
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
    match event.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
            // return Err(Error::new(io::ErrorKind::Interrupted, "Quit editor!"));
            return Ok(false)
        },
        KeyCode::Char(c) => buffer.insert_char(c),
        KeyCode::Enter => buffer.insert_empty_line(),
        KeyCode::Backspace => buffer.rm_char_backspace(),
        KeyCode::Delete => buffer.rm_char_delete(),
        KeyCode::Up => buffer.move_cursor_up(),
        KeyCode::Down => buffer.move_cursor_down(),
        KeyCode::Left => buffer.move_cursor_left(),
        KeyCode::Right => buffer.move_cursor_right(),
        _ => {}
    };
    Ok(true)
}

