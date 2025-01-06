mod editor;
mod text_buffer;
mod editor_state;

use editor::Editor;

use crossterm::{
    execute, 
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        DisableLineWrap, EnableLineWrap, 
        EnterAlternateScreen, LeaveAlternateScreen
    }
};

fn main() -> crossterm::Result<()> {
    let mut stdout = std::io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, DisableLineWrap)?;
    
    // render and run the editor
    let editor = Editor::new(); 
    match editor {
        Ok(mut e) => e.run()?,
        Err(_) => (),
    };

    execute!(&stdout, LeaveAlternateScreen, EnableLineWrap)?;
    disable_raw_mode()?;
    Ok(())
}
