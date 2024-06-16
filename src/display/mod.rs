use std::{
    io::{stdout, Write},
    path::PathBuf,
};

use termion::{color, style};

use crate::file_system::entry::Entry;

pub struct Display {
    w: usize,
    h: usize,
}

impl Display {
    pub fn new() -> Display {
        let (w, h) = termion::terminal_size().unwrap();
        Display {
            w: w as usize,
            h: h as usize,
        }
    }

    pub fn calculate_display_start(&self, sel_idx: u8, num_entries: u16) -> u16 {
        let height = self.usable_height();

        if num_entries < height as u16 {
            return 0;
        }

        if sel_idx < height as u8 / 2 {
            0
        } else if (sel_idx as u16) > num_entries - (height as u16) / 2 {
            num_entries - height as u16
        } else {
            (sel_idx as u16) - (height as u16) / 2
        }
    }

    // This shouldn't live in this module but not sure of a better place for it
    pub fn get_displayed_entries(&self, display_start: u16, entries: Vec<Entry>) -> Vec<Entry> {
        let height = self.usable_height();
        if entries.len() > height as usize {
            let range = (display_start as usize)
                ..std::cmp::min(entries.len(), display_start as usize + height as usize);
            let mut return_vec: Vec<Entry> = Vec::new();
            return_vec.extend_from_slice(&entries[range]);
            return_vec
        } else {
            entries
        }
    }

    pub fn usable_height(&self) -> u16 {
        self.h as u16 - 3
    }

    pub fn render(&self, entries: &Vec<Entry>, sel: u8, current_dir: &PathBuf) {
        let display_start = self.calculate_display_start(sel, entries.len() as u16);
        let displayed_entries = self.get_displayed_entries(display_start, entries.clone());

        // Reset screen
        let mut output_string = self.clear();
        output_string.push_str(&format!(
            "{}{}",
            color::Bg(color::Reset),
            color::Fg(color::Reset)
        ));

        let mut idx = 0;

        // draw top bar
        output_string.push_str(&self.top_bar(current_dir));

        // Draw all entries
        for entry in displayed_entries {
            if idx == sel as u16 - display_start {
                let borrowed_string: &str = &(self.selected_entry_line(entry));
                output_string.push_str(borrowed_string);
            } else {
                output_string.push_str(&(self.normal_entry_line(entry)));
            }

            idx += 1;
        }

        // Draw bottom bar
        output_string.push_str(&(self.bottom_bar(&entries[sel as usize])));

        // Write to stdout
        write!(stdout(), "{}", output_string).unwrap();
        stdout().flush().unwrap();
    }

    fn clear(&self) -> String {
        return format!(
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        );
    }

    fn selected_entry_line(&self, entry: Entry) -> String {
        return format!(
            "{}{}> {}{}{}\r\n",
            color::Bg(color::LightBlack),
            style::Bold.to_string(),
            entry.to_string(),
            color::Bg(color::Reset),
            style::Reset.to_string()
        );
    }

    fn normal_entry_line(&self, entry: Entry) -> String {
        return format!("  {}\r\n", entry.to_string());
    }

    fn top_bar(&self, current_dir: &PathBuf) -> String {
        // Set up color and position
        let mut output_string = String::new();
        output_string.push_str(&format!(
            "{}{}{}",
            termion::cursor::Goto(1, 1),
            color::Bg(color::LightYellow),
            color::Fg(color::Black)
        ));

        // Text and filler (spaces)
        let bar_text = format!(" {}", current_dir.display());
        let filler = std::iter::repeat(" ")
            .take(self.w - bar_text.len())
            .collect::<String>();
        output_string.push_str(&format!(
            "{}{}{}{}{}",
            bar_text,
            filler,
            color::Bg(color::Reset),
            color::Fg(color::Reset),
            std::iter::repeat(" ").take(self.w).collect::<String>()
        ));
        return output_string;
    }

    fn bottom_bar(&self, entry: &Entry) -> String {
        // Set up color and position
        let mut output_string = String::new();
        output_string.push_str(&format!(
            "{}{}{}",
            termion::cursor::Goto(1, self.h as u16),
            color::Bg(color::LightYellow),
            color::Fg(color::Black)
        ));

        // Text and filler (spaces)
        let bar_text = format!(" Modified {}", entry.last_modified_readable());
        // let bar_text = format!(" {}", current_dir.display());
        let filler = std::iter::repeat(" ")
            .take(self.w - bar_text.len())
            .collect::<String>();
        output_string.push_str(&format!(
            "{}{}{}{}",
            bar_text,
            filler,
            color::Bg(color::Reset),
            color::Fg(color::Reset)
        ));
        return output_string;
    }
}
