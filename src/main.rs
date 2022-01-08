use std::{io, thread, time};
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Termur")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
    
    thread::sleep(time::Duration::from_millis(2000));

    Ok(())
}
