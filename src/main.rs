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

    println!("selected: {:?}", chosen);
}
