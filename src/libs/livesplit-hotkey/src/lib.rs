#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    missing_docs,
    rust_2018_idioms
)]
#![cfg_attr(not(feature = "std"), no_std)]

//! `livesplit-hotkey` is a crate that allows listening to hotkeys even when the
//! application is not in focus. The crate currently supports Windows, macOS,
//! Linux and the web via wasm-bindgen. On unsupported platforms the crate still
//! compiles but uses a stubbed out implementation instead that never receives
//! any hotkeys.

extern crate alloc;

cfg_if::cfg_if! {
    if #[cfg(not(feature = "std"))] {
        mod other;
        use self::other as platform;
    } else if #[cfg(windows)] {
        mod windows;
        use self::windows as platform;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        use self::linux as platform;
    } else if #[cfg(target_os = "macos")] {
        mod macos;
        use self::macos as platform;
    } else if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
        cfg_if::cfg_if! {
            if #[cfg(feature = "wasm-web")] {
                mod wasm_web;
                use self::wasm_web as platform;
            } else {
                mod wasm_unknown;
                use self::wasm_unknown as platform;
            }
        }
    } else {
        mod other;
        use self::other as platform;
    }
}

mod key_code;
pub use self::{key_code::*, platform::*};

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn test() {
        let hook = Hook::new().unwrap();
        hook.register(KeyCode::Numpad1, || println!("A")).unwrap();
        println!("Press Numpad1");
        thread::sleep(Duration::from_secs(5));
        hook.unregister(KeyCode::Numpad1).unwrap();
        hook.register(KeyCode::KeyN, || println!("B")).unwrap();
        println!("Press KeyN");
        thread::sleep(Duration::from_secs(5));
        hook.unregister(KeyCode::KeyN).unwrap();
        hook.register(KeyCode::Numpad1, || println!("C")).unwrap();
        println!("Press Numpad1");
        thread::sleep(Duration::from_secs(5));
        hook.unregister(KeyCode::Numpad1).unwrap();
    }

    #[test]
    fn resolve() {
        // Based on German keyboard layout.
        println!("ß: {}", KeyCode::Minus.resolve());
        println!("ü: {}", KeyCode::BracketLeft.resolve());
        println!("#: {}", KeyCode::Backslash.resolve());
        println!("+: {}", KeyCode::BracketRight.resolve());
        println!("z: {}", KeyCode::KeyY.resolve());
        println!("^: {}", KeyCode::Backquote.resolve());
        println!("<: {}", KeyCode::IntlBackslash.resolve());
        println!("Yen: {}", KeyCode::IntlYen.resolve());
        println!("Enter: {}", KeyCode::Enter.resolve());
        println!("Space: {}", KeyCode::Space.resolve());
        println!("Tab: {}", KeyCode::Tab.resolve());
        println!("Numpad0: {}", KeyCode::Numpad0.resolve());
    }
}
