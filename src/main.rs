use std::io::{self, Read, Stderr};

use clap::Parser;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use sway_groups_core::sway::SwayIpcClient;

use crate::{
    app::App,
    desktop_file::{DesktopFile, DesktopFiles},
};

mod app;
mod desktop_file;
mod fuzzy_search_list;
mod input_box;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// Launch a D-menu style menu.
    Menu,
    /// A program launcher.
    ProgramLauncher,
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Menu => menu(),
        Command::ProgramLauncher => program_launcher(),
    }
}

fn menu() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    match handle.read_to_string(&mut buffer) {
        Ok(_i) => (),
        Err(_e) => {
            println!("Unable to read input from stdin.");
            return;
        }
    }

    let items = buffer
        .lines()
        .map(|line| (line.to_owned(), line.to_owned()))
        .collect();

    let mut chosen: Option<String> = None;

    run_tui(|terminal| chosen = App::new(items).run(terminal).unwrap());

    if let Some(v) = chosen {
        println!("{}", v)
    }
}

fn program_launcher() {
    let desktop_files = DesktopFiles::find_default();

    let items = desktop_files
        .extract()
        .iter()
        .map(|item| (item.name.to_owned(), item.to_owned()))
        .collect();

    let mut chosen: Option<DesktopFile> = None;

    run_tui(|terminal| chosen = App::new(items).run(terminal).unwrap());

    if let Some(item) = chosen {
        let ipc_client = SwayIpcClient::new().unwrap();
        let command_string: String = item
            .execution_command
            .split_whitespace()
            .filter(|item| !item.starts_with('%'))
            .enumerate()
            .map(|(index, item)| {
                if index > 0 {
                    " ".to_string() + item
                } else {
                    item.to_string()
                }
            })
            .collect();

        ipc_client
            .run_command(&("exec ".to_string() + &command_string))
            .unwrap();
    }
}

pub type TuiTerminal = Terminal<CrosstermBackend<Stderr>>;

/// Like `ratatui::run`, but draws on stderr so stdout only carries the result
/// (dmenu/fzf style), letting callers capture it with `$(...)`.
fn run_tui<F, R>(f: F) -> R
where
    F: FnOnce(&mut TuiTerminal) -> R,
{
    enable_raw_mode().unwrap();
    execute!(io::stderr(), EnterAlternateScreen).unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stderr())).unwrap();
    let result = f(&mut terminal);
    let _ = terminal.show_cursor();
    let _ = execute!(io::stderr(), LeaveAlternateScreen);
    let _ = disable_raw_mode();
    result
}
