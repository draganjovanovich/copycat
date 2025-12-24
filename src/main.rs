extern crate clipboard;
extern crate dirs;
extern crate livesplit_hotkey;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use livesplit_hotkey::Hook;
use livesplit_hotkey::KeyCode;
use std::fs::{self, OpenOptions};
use std::io::{prelude::*, BufWriter};
use std::time::{Duration, Instant};

const CLIPBOARD_POLL_INTERVAL_MS: u64 = 150;
const MAX_CLIPBOARD_SIZE: usize = 1_000_000;
const MAX_FILE_ENTRIES: usize = 10_000;
const ROTATION_CHECK_INTERVAL: u32 = 100;

fn main() {
    let path = dirs::home_dir().unwrap().join(".copycat");

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .expect("Failed to open .copycat file");
    let mut writer = BufWriter::with_capacity(8192, file);

    let mut last_content_hash: u64 = 0;
    let mut write_count: u32 = 0;

    let hook = loop {
        match Hook::new() {
            Ok(h) => break h,
            Err(e) => {
                eprintln!("Copycat: Failed to create hotkey hook: {:?}", e);
                eprintln!("Copycat: Accessibility permission may be denied.");
                eprintln!("Copycat: Please enable in System Settings > Privacy & Security > Accessibility");
                eprintln!("Copycat: Retrying in 60 seconds...");
                std::thread::sleep(Duration::from_secs(60));
            }
        }
    };

    hook.register(KeyCode::AltLeft, move || {
        static mut ALT_TIMES_PRESSED: i32 = 0;
        static mut ALT_LAST_TIME: Option<Instant> = None;

        unsafe {
            let now = Instant::now();

            let too_slow = ALT_LAST_TIME
                .map(|last| now.duration_since(last) > Duration::from_millis(500))
                .unwrap_or(true);

            if too_slow {
                ALT_TIMES_PRESSED = 0;
            } else {
                ALT_TIMES_PRESSED += 1;
                if ALT_TIMES_PRESSED == 2 {
                    ALT_TIMES_PRESSED = 0;
                    let _ = std::process::Command::new("pkill")
                        .args(["-9", "-f", "/usr/local/bin/popup"])
                        .output();
                    std::thread::sleep(Duration::from_millis(50));
                    if let Err(e) = std::process::Command::new("/usr/local/bin/popup").spawn() {
                        eprintln!("Failed to spawn popup: {}", e);
                    }
                }
            }

            ALT_LAST_TIME = Some(now);
        }
    })
    .expect("Failed to register hotkey");

    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .expect("Failed to create clipboard context");

    loop {
        match ctx.get_contents() {
            Ok(contents) => {
                let trimmed = contents.trim();

                if trimmed.is_empty() || trimmed.len() > MAX_CLIPBOARD_SIZE {
                    std::thread::sleep(Duration::from_millis(CLIPBOARD_POLL_INTERVAL_MS));
                    continue;
                }

                let hash = simple_hash(trimmed);

                if hash != last_content_hash {
                    last_content_hash = hash;

                    if let Err(e) = writeln!(writer, "{}\u{200b}", trimmed) {
                        eprintln!("Failed to write to file: {}", e);
                    } else {
                        let _ = writer.flush();
                        write_count += 1;

                        if write_count >= ROTATION_CHECK_INTERVAL {
                            write_count = 0;
                            if let Some(new_writer) = rotate_file_if_needed(&path) {
                                writer = new_writer;
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }

        std::thread::sleep(Duration::from_millis(CLIPBOARD_POLL_INTERVAL_MS));
    }
}

#[inline]
fn simple_hash(s: &str) -> u64 {
    let mut hash: u64 = 5381;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}

fn rotate_file_if_needed(path: &std::path::Path) -> Option<BufWriter<std::fs::File>> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return None,
    };

    let entries: Vec<&str> = content.split("\u{200b}").collect();
    let entry_count = entries.len();

    if entry_count <= MAX_FILE_ENTRIES {
        return None;
    }

    let keep_count = MAX_FILE_ENTRIES / 2;
    let start_idx = entry_count.saturating_sub(keep_count);
    let kept_entries: Vec<&str> = entries[start_idx..].to_vec();

    let new_content = kept_entries.join("\u{200b}");
    if fs::write(path, &new_content).is_err() {
        return None;
    }

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .ok()?;

    Some(BufWriter::with_capacity(8192, file))
}
