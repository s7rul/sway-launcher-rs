use crate::{app::App, desktop_file::DesktopFiles};

mod desktop_file;
mod app;
mod fuzzy_search_list;

fn main() {
    let desktop_files = DesktopFiles::find_default();

    let names = desktop_files.get_all_names().iter().map(|item| item.to_string()).collect();

    ratatui::run(|terminal| App::new(names).run(terminal)).unwrap();
}
