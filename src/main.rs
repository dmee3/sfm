use std::env;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

mod display;

mod entry;
use crate::entry::Entry;

fn get_entries(dir: &Path) -> Vec<Entry> {
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

fn move_up(current_dir: &mut PathBuf, entries: &mut Vec<Entry>, sel: &mut u8) {
    let cd = current_dir.clone();
    let child = cd.file_name().unwrap().to_str().unwrap();
    if current_dir.pop() {
        *entries = get_entries(&current_dir);
        match entries.iter().position(|e| e.name == child) {
            Some(idx) => *sel = idx as u8,
            None => *sel = 0,
        }
    }
}

fn move_down(current_dir: &mut PathBuf, entries: &mut Vec<Entry>, sel: &mut u8) {
    let selected = &entries[*sel as usize];
    if selected.path.is_dir() {
        *current_dir = selected.path.clone();
        *entries = get_entries(&current_dir);
        *sel = 0;
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut current_dir = env::current_dir().unwrap();
    let mut entries = get_entries(&current_dir);
    let mut sel: u8 = 0;

    let display = display::Display::new();
    display.render(&entries, sel);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Esc => break,
            Key::Up => {
                if sel > 0 {
                    sel -= 1
                }
            }
            Key::Down => {
                if sel < entries.len() as u8 - 1 {
                    sel += 1
                }
            }
            Key::Left => move_up(&mut current_dir, &mut entries, &mut sel),
            Key::Right => move_down(&mut current_dir, &mut entries, &mut sel),
            Key::Char('\n') => move_down(&mut current_dir, &mut entries, &mut sel),
            _ => {}
        }

        display.render(&entries, sel);
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
