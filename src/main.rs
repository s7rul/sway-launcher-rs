use crate::desktop_file::DesktopFiles;

mod desktop_file;

fn main() {
    let desktop_files = DesktopFiles::find_default();

    for entry in desktop_files {
        println!("name: {}, exec: {}", entry.name, entry.execution_command);
    }
}
