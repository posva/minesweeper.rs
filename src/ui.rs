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

    draw_title(f, chunks[0], &app);
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

fn draw_title<B>(f: &mut Frame<B>, area: Rect, app: &app::App)
where
    B: Backend,
{
    let text = Span::styled(
        format!(
            "Minesweeper {} for ({}x{})",
            app.last_reveal, app.click.0, app.click.1
        ),
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

    draw_minefield(f, chunks[0], field);

    // let block = Block::default().borders(Borders::ALL);

    // let paragraph = Paragraph::new(field.as_text_ascii(true))
    //     .block(block)
    //     .wrap(Wrap { trim: true });
    // let paragraph = Paragraph::new(field.as_lines(false).map(|text| Spans::from(text)).collect()).block(block).wrap(Wrap { trim: true });

    // f.render_widget(paragraph, chunks[1]);
}

fn draw_minefield<B>(f: &mut Frame<B>, area: Rect, field: &game::Field)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                // + 2 for borders
                Constraint::Length(field.config.rows as u16 + 2),
                Constraint::Max(4),
                Constraint::Max(0),
            ]
            .as_ref(),
        )
        .split(area);

    let number_styles = vec![
        Style::default(),
        // 1
        Style::default().fg(Color::LightBlue),
        // 2
        Style::default().fg(Color::LightGreen),
        // 3
        Style::default().fg(Color::LightYellow),
        // 4
        Style::default().fg(Color::LightMagenta),
        // 5
        Style::default().fg(Color::LightRed),
        // 6
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
        // 7
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD),
        // 8
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        // 9 Mine
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        // 10 Empty
        Style::default(),
    ];

    // actual minefield
    let block = Block::default().borders(Borders::ALL);
    let paragraph = Paragraph::new(
        field
            .get_field()
            .into_iter()
            .map(|line| {
                Spans::from(
                    line.into_iter()
                        .map(|cell| {
                            Span::styled(
                                cell_to_string(cell),
                                *number_styles.get(cell as usize).unwrap(),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>(),
    )
    .block(block);
    // not necessary
    // .wrap(Wrap { trim: false });

    f.render_widget(paragraph, chunks[0]);

    draw_field_config(f, chunks[1], field);
}

fn cell_to_string(mines: u32) -> String {
    if mines == 0 {
        String::from("  ")
    } else if mines == 8 {
        String::from("🚩")
    } else if mines == 9 {
        String::from("💣")
    } else if mines == 10 {
        String::from("🔲")
    } else {
        let codepoint = 0x245f + mines as u16;
        String::from_utf16(&[codepoint, 0x20]).unwrap()
    }
}

fn draw_field_config<B>(f: &mut Frame<B>, area: Rect, field: &game::Field)
where
    B: Backend,
{
    let block = Block::default()
        .title("Config")
        // .style(Style::default().fg(Color::White).bg(Color::Black))
        .borders(Borders::ALL);

    let label_style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let text = vec![
        Spans::from(vec![
            Span::styled("Size: ", label_style),
            Span::raw(format!("{}x{}", field.config.columns, field.config.rows)),
        ]),
        Spans::from(vec![
            Span::styled("Mines: ", label_style),
            Span::from(format!("{} 💣", field.config.mines)),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block);

    f.render_widget(paragraph, area);
}
