// This is an example of how the block object works

// OBJECTIVE: do example, then change and experiment

/*
 get all modules needed to render the block
 and then some. Looks like they really like
 to grab specific functions and structs
 instead of the whole module in rust.
*/
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create the app and run it
    let app = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = app {
        println!("{:?}", err)
    }

    Ok(())
}

// The actual function that contains the main loop
// need to look at syntax book
// because this is kinda confusing
fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui);

        // quit key
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }

    Ok(())
}

fn ui<B: Backend>(frame: &mut Frame<B>) {
    // Wrapping block for a group
    // Just draw the block and the group
    // with some margin

    let size = frame.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Main Block with Round Corners")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    frame.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(size);

    // top two inner blocks
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    // top left inner block with green background
    let block = Block::default()
        .title(vec![
            Span::styled("With", Style::default().fg(Color::DarkGray)),
            Span::styled(" background", Style::default().fg(Color::Red)),
        ])
        .style(Style::default().bg(Color::Green));
    frame.render_widget(block, top_chunks[0]);

    // Top right inner block styled w/ title aligned right
    let block = Block::default()
        .title(Span::styled(
            "Style Title",
            Style::default()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::ITALIC),
        ))
        .title_alignment(Alignment::Right);
    frame.render_widget(block, top_chunks[1]);

    // Bottom two inner blocks
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Bottom left block with all default borders
    let block = Block::default().title("With borders").borders(Borders::ALL);
    frame.render_widget(block, bottom_chunks[0]);

    // Bottom right block with styled left and right border
    let block = Block::default()
        .title("With styled borders and doubled borders")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan))
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .border_type(BorderType::Double);
    frame.render_widget(block, bottom_chunks[1]);
}
