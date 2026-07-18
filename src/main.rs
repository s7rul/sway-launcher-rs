use sway_groups_core::sway::SwayIpcClient;

use crate::{app::App, desktop_file::{DesktopFile, DesktopFiles}};

mod app;
mod desktop_file;
mod fuzzy_search_list;
mod input_box;

fn main() {
    let desktop_files = DesktopFiles::find_default();

    let items = desktop_files.extract().iter().map(|item| (item.name.to_owned(), item.to_owned())).collect();

    let mut chosen: Option<DesktopFile> = None;

    ratatui::run(|terminal| chosen = App::new(items).run(terminal).unwrap());
    
    if let Some(item) = chosen {
        let ipc_client = SwayIpcClient::new().unwrap();
        let command_string: String = item.execution_command.split_whitespace().filter(|item| !item.starts_with('%')).enumerate().map(|(index, item)| {
            if index > 0 {
                " ".to_string() + item
            } else {
                item.to_string()
            }
        }).collect();

        ipc_client.run_command(&("exec ".to_string() + &command_string)).unwrap();
    }
}
