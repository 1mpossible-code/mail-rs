use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};
use tui::{
    backend::{self, CrosstermBackend}, layout::{Constraint, Direction, Layout}, widgets::{Block, Borders}, Terminal
};

fn main(){
    // set up the terminal
    enable_raw_mode().expect("Failed to enable the raw mode");
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("Failed to enter alternate screen");
    // initialize TUI backend
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal");
    // run UI loop
    let res = run_app(&mut terminal);
    // restore on exit
    disable_raw_mode().expect("Failed to disable raw mode");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).expect("Failed to exit")
}

fn run_app<B: tui::backend::Backend>(terminal: &mut Terminal<B>) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Rust Mail Client").borders(Borders::ALL);
            f.render_widget(block, size); }).expect("Failed to draw the app");

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }
    }
}
