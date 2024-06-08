use chrono::{DateTime, Local};
use std::{cmp, path::PathBuf};
use termion::color;

#[derive(PartialEq, Eq, Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
}

impl Entry {
    pub fn to_string(&self) -> String {
        if self.path.is_dir() {
            format!(
                "{}{}{}",
                color::Fg(color::Cyan),
                self.name,
                color::Fg(color::Reset)
            )
        } else if self.path.is_symlink() {
            format!(
                "{}{}{}{}{}",
                color::Fg(color::Green),
                self.name,
                " -> ",
                self.path.read_link().unwrap().to_str().unwrap(),
                color::Fg(color::Reset)
            )
        } else {
            format!("{}", self.name)
        }
    }

    pub fn metadata(&self) -> std::fs::Metadata {
        std::fs::metadata(&self.path).unwrap()
    }

    pub fn last_modified(&self) -> std::time::SystemTime {
        self.metadata().modified().unwrap()
    }

    pub fn last_modified_readable(&self) -> String {
        let elapsed = self.last_modified().elapsed().unwrap().as_secs();
        if elapsed < 60 {
            format!("{} seconds ago", elapsed)
        } else if elapsed < 60 * 60 {
            format!("{} minutes ago", elapsed / 60)
        } else {
            let time: DateTime<Local> = self.last_modified().clone().into();
            time.format("%a, %b %e %Y, %l:%M %P").to_string()
        }
    }
}

impl cmp::PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.name.to_lowercase().cmp(&other.name.to_lowercase()))
    }
}

impl cmp::Ord for Entry {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}
