// std
use std::{
    fs,
    borrow::Cow,
    io::{self, BufRead, BufReader},
};

// external
use termion::{raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Layout, Constraint, Alignment, Rect},
    style::{Color, Style},
    terminal::Terminal,
    buffer::Buffer,
    widgets::{Block, Borders, Paragraph, Text, Widget},
};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    // let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear();
    let buffer = BufReader::new(fs::File::open("test.txt")?);

    let lines = buffer
        .lines()
        .filter_map(|line| line.ok())
        .map(|mut line| {
            line.push('\n');

            line
        })
        .map(|line| Text::Raw(Cow::Owned(line)))
        .collect::<Vec<_>>();

    let mut p = Paragraph::new(lines.iter())
        .block(Block::default().title("Paragraph").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(true);

    terminal.draw(|mut f| {
        let size = f.size();
        let chunks = Layout::default()
            .constraints([
                Constraint::Percentage(100),
            ].as_ref())
            .split(size);

        p.render(&mut f, chunks[0]);
    });

    Ok(())
}
