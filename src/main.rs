mod editor;
mod view;
mod model;

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

    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 2);
    
    // render and run the editor
    let editor = editor::Editor::new(args[1].clone()); 
    match editor {
        Ok(mut e) => e.run()?,
        Err(_) => (),
    };

    execute!(&stdout, LeaveAlternateScreen, EnableLineWrap)?;
    disable_raw_mode()?;
    Ok(())
}
