use app::App;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{Backend, CrosstermBackend},
    restore, Terminal,
};
use std::{
    io::{self},
    path::PathBuf,
};
use ui::Ui;

pub mod app;
pub mod ui;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Path to the database file
    #[arg(short = 'f', long = "file", value_name = "sqlite database")]
    file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    let cli = CliArgs::parse();

    let mut app = App::default();
    let mut ui = Ui::new()?;

    // setup terminal
    enable_raw_mode()?;
    set_panic_hook();
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Load file if given as argument
    handle_cli(cli, &mut app, &mut ui)?;

    let res = run_app(&mut terminal, &mut app, &mut ui);

    if let Err(err) = res {
        eprintln!("{err}")
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn handle_cli(cli: CliArgs, app: &mut App, ui: &mut Ui) -> Result<(), Box<dyn std::error::Error>> {
    Ok(if let Some(file_path) = cli.file.as_deref() {
        app.load_db(file_path.display().to_string().as_str())?;
        if let Some(db) = &app.current_db {
            ui.table_view.load_nav(db);
        }
    })
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    ui: &mut Ui,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui.ui(f, app))?;
        if let Event::Key(event) = event::read()? {
            if event.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            ui.handle_input(&event, app)?;
            if event.code == KeyCode::Esc {
                break;
            }
        }
    }
    Ok(())
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        restore(); // ignore any errors as we are already failing
        hook(panic_info);
    }));
}
