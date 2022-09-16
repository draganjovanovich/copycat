extern crate clipboard;
extern crate dirs;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let mut last_line = String::new();

    let path = dirs::home_dir().unwrap().join(".copycat");
    let mut file = OpenOptions::new ()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
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
    }
}
