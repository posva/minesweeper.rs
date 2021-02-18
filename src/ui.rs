use crate::app;
use crate::game;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut app::App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
        .split(f.size());

    draw_title(f, chunks[0]);
    draw_screen(f, chunks[1], &app.field);

    let screen = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(app.field.config.columns as u16 * 2)].as_ref())
        .split(f.size());

    // f.render_widget(screen, chunks[1]);

    let block = Block::default()
        .title(format!("({}, {})", app.click.0, app.click.1))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .borders(Borders::ALL);
    // f.render_widget(block, screen[0]);

    // let chunks = Layout::default()
    //   .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
    //   .split(f.size());
    // let titles = app
    //   .tabs
    //   .titles
    //   .iter()
    //   .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
    //   .collect();
}

fn draw_title<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = Span::styled(
        "Minesweeper",
        Style::default()
            // TODO: why only one style?
            .add_modifier(Modifier::BOLD)
            .fg(Color::LightMagenta),
    );
    // let block = Block::default().borders(Borders::BOTTOM);

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn draw_screen<B>(f: &mut Frame<B>, area: Rect, field: &game::Field)
where
    B: Backend,
{
    // TODO: responsive layout vertical /horizontal
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                // * 2 because double size
                // + 2 for borders
                Constraint::Length(field.config.columns as u16 * 2 + 2),
                Constraint::Min(5),
            ]
            .as_ref(),
        )
        .split(area);

    let block = Block::default().borders(Borders::ALL);
    let paragraph = Paragraph::new(
        field
            .as_lines(true)
            .into_iter()
            .map(Spans::from)
            .collect::<Vec<_>>(),
    )
    .block(block);
    // not necessary
    // .wrap(Wrap { trim: false });

    f.render_widget(paragraph, chunks[0]);

    // let block = Block::default().borders(Borders::ALL);

    // let paragraph = Paragraph::new(field.as_text_ascii(true))
    //     .block(block)
    //     .wrap(Wrap { trim: true });
    // let paragraph = Paragraph::new(field.as_lines(false).map(|text| Spans::from(text)).collect()).block(block).wrap(Wrap { trim: true });

    // f.render_widget(paragraph, chunks[1]);
}
