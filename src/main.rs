use argh::FromArgs;
use std::{error::Error, io, time::Duration};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

mod app;
mod ui;
mod util;
use app::App;
use util::event::{Config, Event, Events};
mod game;

// TODO: rename in game config
#[derive(Debug, FromArgs)]
/// GameConfig
struct Options {
    /// time in ms between two ticks.
    #[argh(option, default = "250", short = 't')]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true", short = 'u')]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let game_config = game::CONFIG_BEGINNER;
    let options: Options = argh::from_env();

    let events = Events::with_config(Config {
        tick_rate: Duration::from_millis(options.tick_rate),
        ..Config::default()
    });

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mut app = App::new("MineSweeper", &game_config, options.enhanced_graphics);
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match events.next()? {
            Event::Input(key) => match key {
                //   Key::Up | Key::Char('k') => {
                //     app.on_up();
                //   }
                //   Key::Down | Key::Char('j') => {
                //     app.on_down();
                //   }
                //   Key::Left | Key::Char('h') => {
                //     app.on_left();
                //   }
                //   Key::Right | Key::Char('l') => {
                //     app.on_right();
                //   }
                Key::Char(c) => {
                    app.on_key(c);
                }
                _ => {}
            },
            Event::Click(x, y) => {
                app.set_click(x, y);
            }
            // Event::Tick => {
            //   app.on_tick();
            // }
            _ => {}
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
