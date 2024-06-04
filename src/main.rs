use std::cmp::min;
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use termion::{color, event::Key, input::TermRead, raw::IntoRawMode, style};

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

// DISPLAY

fn clear_screen() {
    write!(
        stdout(),
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout().flush().unwrap();
}

fn get_term_size() -> (u16, u16) {
    termion::terminal_size().unwrap()
}

fn display(sel: u8, entries: &Vec<Entry>) {
    clear_screen();

    let (_width, height) = get_term_size();
    let mut idx = 0;
    let display_start = calculate_display_start(sel, height as u8, entries.len() as u16);
    let displayed_entries = get_displayed_entries(display_start, height, entries.clone());

    for entry in displayed_entries {
        if idx == sel - display_start as u8 {
            write!(
                stdout(),
                "{}{}",
                color::Bg(color::LightBlack),
                style::Bold.to_string()
            )
            .unwrap();
            write!(stdout(), "> {}", entry.to_string()).unwrap();
            write!(
                stdout(),
                "{}{}\r\n",
                color::Bg(color::Reset),
                style::Reset.to_string()
            )
            .unwrap();
        } else {
            write!(stdout(), "  {}\r\n", entry.to_string()).unwrap();
        }
        stdout().flush().unwrap();

        idx += 1;
    }
}

fn get_displayed_entries(display_start: u16, height: u16, entries: Vec<Entry>) -> Vec<Entry> {
    if entries.len() > height as usize {
        let range =
            (display_start as usize)..min(entries.len(), display_start as usize + height as usize);
        let mut return_vec: Vec<Entry> = Vec::new();
        return_vec.extend_from_slice(&entries[range]);
        return_vec
    } else {
        entries
    }
}

fn calculate_display_start(sel: u8, term_height: u8, num_entries: u16) -> u16 {
    if num_entries < term_height as u16 {
        return 0;
    }

    if (sel as u16) < num_entries / 2 {
        0
    } else if (sel as u16) > num_entries - (term_height as u16) / 2 {
        num_entries - term_height as u16
    } else {
        (sel as u16) - (term_height as u16) / 2
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut current_dir = env::current_dir().unwrap();
    let mut entries = get_entries(&current_dir);
    let mut sel: u8 = 0;

    display(sel, &entries);

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

        display(sel, &entries);
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
