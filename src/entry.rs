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
