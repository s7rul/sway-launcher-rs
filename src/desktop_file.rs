use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct DesktopFile {
    pub name: String,
    pub execution_command: String,
}

impl DesktopFile {
    pub fn from_path(path: PathBuf) -> Option<Self> {
        println!("path: {:?}", path);
        let text = fs::read_to_string(&path).unwrap();

        let mut in_desktop_entry = false;
        let mut name: Option<String> = None;
        let mut exec: Option<String> = None;

        for raw_line in text.lines() {
            let line = raw_line.trim_end_matches('\r').trim_end();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(header) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
                in_desktop_entry = header.trim() == "Desktop Entry";
                continue;
            }
            if !in_desktop_entry {
                continue;
            }
            let (key, value) = line.split_once('=')?;
            let key = key.trim();
            let value = value.trim();

            if key == "Name" {
                name = Some(value.to_string());
            } else if key == "Exec" && exec.is_none() {
                exec = Some(value.to_string());
            }
        }

        if let Some(name) = name
            && let Some(exec) = exec
        {
            Some(Self {
                name,
                execution_command: exec,
            })
        } else {
            None
        }
    }
}

pub struct DesktopFiles {
    desktop_files: Vec<DesktopFile>,
}

impl DesktopFiles {
    pub fn find(desktop_file_dirs: Vec<PathBuf>) -> Self {
        let mut result = Self {
            desktop_files: vec![],
        };

        let mut to_explore = desktop_file_dirs;

        while let Some(current_dir) = to_explore.pop() {
            for entry in fs::read_dir(current_dir).unwrap().flatten() {
                let file_type = entry.file_type().unwrap();
                let path = entry.path();
                if file_type.is_dir() {
                    to_explore.push(entry.path());
                } else if file_type.is_file() && path.extension().is_some_and(|e| e == "desktop") {
                    result
                        .desktop_files
                        .push(DesktopFile::from_path(path).unwrap());
                }
            }
        }

        result
    }

    pub fn find_default() -> Self {
        let mut dirs = Vec::new();

        if let Ok(data_home) = std::env::var("XDG_DATA_HOME") {
            if !data_home.is_empty() {
                dirs.push(PathBuf::from(data_home).join("applications"));
            }
        } else if let Ok(home) = std::env::var("HOME") {
            dirs.push(PathBuf::from(home).join(".local/share/applications"));
        }

        let data_dirs = std::env::var("XDG_DATA_DIRS")
            .unwrap_or_else(|_| "/usr/local/share:/usr/share".to_string());
        for part in data_dirs.split(':') {
            if part.is_empty() {
                continue;
            }
            dirs.push(PathBuf::from(part).join("applications"));
        }

        let dirs: Vec<PathBuf> = dirs
            .iter()
            .filter(|item| item.exists())
            .map(|item| item.to_owned())
            .collect();

        Self::find(dirs)
    }

    pub fn get_all_names<'a>(&'a self) -> Vec<&'a str> {
        self.desktop_files
            .iter()
            .map(|item| item.name.as_str())
            .collect()
    }
}
