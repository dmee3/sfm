use std::path::Path;

use entry::Entry;

pub mod entry;

pub fn get_entries(dir: &Path) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();

    if let Ok(dir_entries) = dir.read_dir() {
        for entry in dir_entries {
            if let Ok(entry) = entry {
                if let Some(fname) = entry.file_name().to_str() {
                    entries.push(Entry {
                        path: entry.path(),
                        name: String::from(fname),
                    });
                }
            }
        }
    }

    entries.sort();
    entries
}

pub fn current_dir() -> std::path::PathBuf {
    std::env::current_dir().unwrap()
}
