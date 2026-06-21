use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct DesktopFile {
    pub name: String,
    pub execution_command: String,
}

impl DesktopFile {
    pub fn from_path(path: PathBuf) -> Option<Self> {
        let text = fs::read_to_string(&path).unwrap();

        let mut in_desktop_entry = false;
        let mut name: Option<String> = None;
        let mut exec: Option<String> = None;
        let mut no_display: Option<bool> = None;

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
            } else if key == "NoDisplay" {
                no_display = Some(value == "true");
            }
        }

        if let Some(name) = name
            && let Some(exec) = exec
            && Some(true) != no_display
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
    iterator_index: usize,
    desktop_files: Vec<DesktopFile>,
}

impl DesktopFiles {
    pub fn find(desktop_file_dirs: Vec<PathBuf>) -> Self {
        let mut result = Self {
            iterator_index: 0,
            desktop_files: vec![],
        };

        let mut to_explore = desktop_file_dirs;

        while let Some(current_dir) = to_explore.pop() {
            for entry in fs::read_dir(current_dir).unwrap().flatten() {
                let file_type = entry.file_type().unwrap();
                let path = entry.path();
                if file_type.is_dir() {
                    to_explore.push(entry.path());
                } else if file_type.is_file()
                    && path.extension().is_some_and(|e| e == "desktop")
                    && let Some(desktop_file) = DesktopFile::from_path(path.to_owned())
                {
                    result.desktop_files.push(desktop_file)
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

    pub fn reset_iterator(&mut self) {
        self.iterator_index = 0;
    }
}

impl Iterator for DesktopFiles {
    type Item = DesktopFile;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.desktop_files.get(self.iterator_index);
        self.iterator_index += 1;

        match ret {
            Some(item) => Some(item.to_owned()),
            None => {
                self.iterator_index = 0;
                None
            }
        }
    }
}
