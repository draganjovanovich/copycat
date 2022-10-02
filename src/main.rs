extern crate clipboard;
extern crate dirs;
extern crate livesplit_hotkey;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::fs::OpenOptions;
use std::io::prelude::*;
use livesplit_hotkey::Hook;
use livesplit_hotkey::KeyCode;

fn main() {
    let mut ctrl_times_pressed = 0;
    let mut ctrl_timespan_pressed : std::time::Instant = std::time::Instant::now();
    let mut last_line = String::new();
    let path = dirs::home_dir().unwrap().join(".copycat");
    let mut file = OpenOptions::new ()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    let hook = Hook::new().unwrap();
    hook.register(KeyCode::ControlLeft, move || {
        let now = std::time::Instant::now();
        // This is debouncer for Ctrl key, because hook fires twice on key press, and we cannot
        // filter it out in the hook itself.
        if now - ctrl_timespan_pressed > std::time::Duration::from_millis(500) {
            ctrl_times_pressed = 0;
        } else {
            ctrl_times_pressed += 1;
            if ctrl_times_pressed == 2 {
                ctrl_times_pressed = 0;
                std::process::Command::new("/usr/local/bin/popup").spawn().unwrap();
            }
        }
        ctrl_timespan_pressed = now;
    }).expect("Failed to register hotkey");

    loop {
        let mut ctx : ClipboardContext = ClipboardProvider::new().unwrap();
        let mut contents = ctx.get_contents().unwrap().trim().to_string();
        // add special char that we will use in popup.py to split lines
        contents.push('\u{200b}');

        if contents != last_line && contents != "" {
            last_line = contents.clone();
            if let Err(e) = writeln!(file, "{}", contents) {
                eprintln !("Couldn't write to file: {}", e);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
