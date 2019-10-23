// std
use std::{
    borrow::Cow,
    fs,
    io::{self, BufRead, BufReader},
};

// external
use termion::{
    async_stdin,
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    terminal::Terminal,
    widgets::{Block, Borders, Paragraph, Text, Widget},
};

// modules
mod cursor;

// imports
use cursor::Cursor;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    // let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let buffer = BufReader::new(fs::File::open("test.txt")?);

    // let stdin = async_stdin();
    let stdin = io::stdin();

    let lines = buffer
        .lines()
        .filter_map(|line| line.ok())
        .map(|mut line| {
            line.push('\n');

            line
        })
        .map(|line| Text::Raw(Cow::Owned(line)))
        .collect::<Vec<_>>();

    let mut cursor = Cursor { x: 0, y: 0 };

    println!("ssij1");

    stdin
        .events()
        .filter_map(|event| event.ok())
        .try_for_each(|event| {
            use termion::event::{Event, Key};

            match event {
                Event::Key(Key::Char('k')) => {
                    cursor.y = cursor.y.saturating_sub(1);
                }
                Event::Key(Key::Char('j')) => {
                    cursor.y = cursor.y.saturating_add(1);
                }
                _ => {}
            }
            terminal.draw(|mut f| {
                let size = f.size();
                let chunks = Layout::default()
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(size);

                Paragraph::new(lines.iter())
                    .block(
                        Block::default()
                            .title("Paragraph")
                            .borders(Borders::ALL),
                    )
                    .style(Style::default().fg(Color::White))
                    .alignment(Alignment::Left)
                    .wrap(true)
                    .render(&mut f, chunks[0]);
                cursor.render(&mut f, chunks[0]);
            })
        }).unwrap();

    Ok(())
}
