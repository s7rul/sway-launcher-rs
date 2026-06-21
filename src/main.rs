use crate::desktop_file::DesktopFiles;

mod desktop_file;

fn main() {
    let desktop_files = DesktopFiles::find_default();
    let names = desktop_files.get_all_names();

    for name in names {
        println!("name: {name}")
    }
}
