extern crate clipboard;

use clipboard::{ClipboardContext, ClipboardProvider};
use fltk::enums::{Color, Key};
use fltk::*;
use fltk::{app, prelude::*, window::Window};
use itertools::Itertools;
use std::env;
use std::fs;

const BG: Color = Color::from_hex(0x30303f);
const FG: Color = Color::from_hex(0xbbaaff);
const BG_SEL: Color = Color::from_hex(0xdddddd);

const WIN_W: i32 = 800;
const WIN_H: i32 = 400;

fn main() {
    let mut ctrl = false;
    let mut gg = 0;
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background2(48, 48, 63);
    app::foreground(187, 170, 255);

    let mut wind = Window::default()
        .with_size(WIN_W, WIN_H)
        .center_screen()
        .with_label("CopyCat");
    wind.set_border(false);
    wind.set_color(BG);

    wind.handle(move |_w, ev| {
        if ev == enums::Event::Unfocus {
            app::add_timeout3(1.00, |_| {
                let l = app::focus();
                if l.is_none() {
                    app::quit();
                    std::process::exit(0);
                }
            });
        }
        true
    });

    let mut b = browser::HoldBrowser::default()
        .with_size(WIN_W, WIN_H)
        .center_of(&wind);
    b.set_text_size(20);

    let widths = &[50, WIN_W - 340, 50];
    b.set_column_widths(widths);

    b.set_column_char('\t');
    b.set_color(BG);
    b.set_label_color(FG);
    b.set_selection_color(BG_SEL);

    b.handle(move |b, ev| {
        // On enter, copy the selected item to the clipboard
        if ev == enums::Event::KeyDown && app::event_key() == Key::Enter {
            unsafe {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                let idx: i32 = b.value();
                let d = b.data::<String>(idx);
                ctx.set_contents(
                    d.expect(
                        "ClipboardContext to be set to data of
                        selected item!",
                    )
                    .to_string(),
                )
                .unwrap();
                app.quit();
            }
        }
        if ev == enums::Event::KeyDown {
            if app::event_key() == Key::Escape || app::event_key() == Key::from_char('q') {
                app::quit();
                std::process::exit(0);
            }
        }
        if ev == enums::Event::KeyDown && app::event_key() == Key::from_char('p') {
            let idx: i32 = b.value();
            let x_coord = b.x() + b.width() / 2 + 150;
            let y_coord = b.y() + b.height() / 2 + 200;

            unsafe {
                let d = b.data::<String>(idx);
                if let Some(d) = d {
                    if d.contains("\n") {
                        let popup = MyPopup::new(&d.to_string(), x_coord, y_coord);
                        let mut w = popup.window;
                        // get main window
                        let mut main_win = b.parent().unwrap();
                        w.handle(move |w, ev| {
                            if ev == enums::Event::Unfocus
                                || (ev == enums::Event::KeyDown
                                    && app::event_key() == Key::from_char('p'))
                            {
                                w.hide();
                                main_win.take_focus().unwrap();
                                true
                            } else {
                                false
                            }
                        });
                    }
                }
            }
        }
        true
    });

    let home = env::var("HOME").unwrap();
    let path = format!("{}/.copycat", home);

    let lines: Vec<String> = fs::read_to_string(path)
        .expect("Failed to read input")
        .split("\u{200b}")
        .map(|line| line.to_string())
        .collect();

    let lines: Vec<String> = lines
        .into_iter()
        .filter(|line| line != "")
        .unique()
        .rev()
        .map(|line| line.trim().to_string())
        .collect();

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            continue;
        }
        // ugly stuff..., changing colors based on selection
        let mut row = format!("@C3148545792 {}\t@C3148545792 ", i)
            + line.split("\n").collect::<Vec<&str>>()[0];
        row = row.chars().take(80).collect::<String>();
        let lines = line.split("\n").count();
        if lines > 1 {
            row = format!("{} \t@C3722304768({} more lines in buffer...)", row, lines);
        }

        let data = line.clone();

        // add the item to the browser, text is just short representation and
        // data is the actual CB data
        b.add_with_data(&row, data);
        b.set_callback(move |b| {
            // format rows, on selection
            format_rows(b);
            if app::event_mouse_button() == app::MouseButton::Right {
                unsafe {
                    let idx: i32 = b.value();
                    let d = b.data::<String>(idx);
                    if let Some(d) = d {
                        if !d.contains("\n") {
                            return;
                        }

                        let popup =
                            MyPopup::new(&d.to_string(), app::event_x_root(), app::event_y_root());
                        let mut w = popup.window;
                        w.handle(move |w, ev| {
                            if ev == enums::Event::Unfocus {
                                w.hide();
                                true
                            } else {
                                false
                            }
                        });
                    }
                }
            } else if app::event_clicks_num() > 0 {
                // on double click, copy the selected item to the clipboard
                unsafe {
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    let idx: i32 = b.value();
                    let d = b.data::<String>(idx);
                    ctx.set_contents(d.expect("").to_string()).unwrap();
                    app.quit();
                }
            }
        });
    }
    b.select(1);
    let row = b.text(1).unwrap().replace("@C3148545792", "@C879871488");
    b.set_text(1, &row);
    wind.make_resizable(true);
    wind.show();

    // app loop, handle vim like motions for scrolling
    while app.wait() {
        let ev = app::event();
        if gg == 2 {
            gg = 0;
            let l = app::focus();
            if l.is_some() {
                let w = l.unwrap();
                if w.parent().unwrap() == b.parent().unwrap() {
                    b.select(1);
                    format_rows(&mut b);
                } else {
                    let widget = w.parent().unwrap().child(0).unwrap();
                    let cb_item_preview_option = widget.window().unwrap().child(0).unwrap();
                    let mut cb_item_preview_browser = unsafe {
                        std::mem::transmute::<widget::Widget, browser::HoldBrowser>(
                            cb_item_preview_option,
                        )
                    };
                    cb_item_preview_browser.select(1);
                    format_rows(&mut cb_item_preview_browser);
                }
            }
        }
        if ev == enums::Event::KeyDown && app::event_text() == "g" {
            gg += 1;
        }
        if ev == enums::Event::KeyDown && app::event_text() == "G" {
            let l = app::focus();
            if l.is_some() {
                let w = l.unwrap();
                if w.parent().unwrap() == b.parent().unwrap() {
                    b.select(b.size());
                    format_rows(&mut b);
                } else {
                    let widget = w.parent().unwrap().child(0).unwrap();
                    let cb_item_preview_option = widget.window().unwrap().child(0).unwrap();
                    let mut cb_item_preview_browser = unsafe {
                        std::mem::transmute::<widget::Widget, browser::HoldBrowser>(
                            cb_item_preview_option,
                        )
                    };
                    cb_item_preview_browser.select(cb_item_preview_browser.size());
                    format_rows(&mut cb_item_preview_browser);
                }
            }
        }
        if ev == enums::Event::KeyDown && app::event_key() == Key::ControlL {
            ctrl = true;
        }
        if ev == enums::Event::KeyUp && app::event_key() == Key::ControlL {
            ctrl = false;
        }
        if ev == enums::Event::KeyDown && app::event_key() == Key::from_char('u') && ctrl {
            let l = app::focus();
            if l.is_some() {
                let w = l.unwrap();
                if w.parent().unwrap() == b.parent().unwrap() {
                    let mut page = 24;
                    if b.value() - 24 < 1 {
                        page = b.value() - 1;
                    }
                    b.select(b.value() - page);
                    format_rows(&mut b);
                } else {
                    let widget = w.parent().unwrap().child(0).unwrap();
                    let cb_item_preview_option = widget.window().unwrap().child(0).unwrap();
                    let mut cb_item_preview_browser = unsafe {
                        std::mem::transmute::<widget::Widget, browser::HoldBrowser>(
                            cb_item_preview_option,
                        )
                    };
                    let idx: i32 = cb_item_preview_browser.value();
                    let mut page = 24;
                    if cb_item_preview_browser.value() - 24 < 1 {
                        page = cb_item_preview_browser.value() - 1;
                    }
                    cb_item_preview_browser.select(idx - page);
                    format_rows(&mut cb_item_preview_browser);
                }
            }
        }
        if ev == enums::Event::KeyDown && app::event_key() == Key::from_char('d') && ctrl {
            let l = app::focus();
            if l.is_some() {
                let w = l.unwrap();
                if w.parent().unwrap() == b.parent().unwrap() {
                    let mut page = 24;
                    if b.value() + 24 > b.size() {
                        page = b.size() - b.value();
                    }
                    b.select(b.value() + page);
                    format_rows(&mut b);
                } else {
                    let widget = w.parent().unwrap().child(0).unwrap();
                    let cb_item_preview_option = widget.window().unwrap().child(0).unwrap();
                    let mut cb_item_preview_browser = unsafe {
                        std::mem::transmute::<widget::Widget, browser::HoldBrowser>(
                            cb_item_preview_option,
                        )
                    };
                    let idx: i32 = cb_item_preview_browser.value();
                    let mut page = 24;
                    if cb_item_preview_browser.value() + 24 > cb_item_preview_browser.size() {
                        page = cb_item_preview_browser.size() - cb_item_preview_browser.value();
                    }
                    cb_item_preview_browser.select(idx + page);
                    format_rows(&mut cb_item_preview_browser);
                }
            }
        }
        if ev == enums::Event::KeyDown && app::event_key() == Key::from_char('j') {
            let l = app::focus();
            if l.is_some() {
                let w = l.unwrap();
                if w.parent().unwrap() == b.parent().unwrap() {
                    b.select(b.value() + 1);
                    format_rows(&mut b);
                } else {
                    let widget = w.parent().unwrap().child(0).unwrap();
                    let cb_item_preview_option = widget.window().unwrap().child(0).unwrap();
                    let mut cb_item_preview_browser = unsafe {
                        std::mem::transmute::<widget::Widget, browser::HoldBrowser>(
                            cb_item_preview_option,
                        )
                    };
                    let idx: i32 = cb_item_preview_browser.value();
                    cb_item_preview_browser.select(idx + 1);
                    format_rows(&mut cb_item_preview_browser);
                }
            }
        }
        if ev == enums::Event::KeyDown && app::event_key() == Key::from_char('k') {
            let l = app::focus();
            if l.is_some() {
                let w = l.unwrap();
                if w.parent().unwrap() == b.parent().unwrap() {
                    b.select(b.value() - 1);
                    format_rows(&mut b);
                } else {
                    let widget = w.parent().unwrap().child(0).unwrap();
                    let cb_item_preview_option = widget.window().unwrap().child(0).unwrap();
                    let mut cb_item_preview_browser = unsafe {
                        std::mem::transmute::<widget::Widget, browser::HoldBrowser>(
                            cb_item_preview_option,
                        )
                    };
                    let idx: i32 = cb_item_preview_browser.value();
                    cb_item_preview_browser.select(idx - 1);
                    format_rows(&mut cb_item_preview_browser);
                }
            }
        }
    }
}

pub struct MyPopup {
    window: window::Window,
}

impl MyPopup {
    pub fn new(txt: &str, x: i32, y: i32) -> Self {
        let mut window = Window::default()
            .with_size(WIN_W - 100, WIN_H - 100)
            .center_screen()
            .with_label("CB Item Preview");
        window.set_pos(x, y);
        window.make_modal(false);
        window.set_border(false);
        window.set_color(BG);
        let mut b = browser::HoldBrowser::default()
            .with_size(WIN_W - 100, WIN_H - 100)
            .center_of(&window);
        b.set_text_size(20);
        b.set_label("BLA");

        let widths = &[50, WIN_W - 340, 50];
        b.set_column_widths(widths);

        b.set_column_char('\t');
        b.set_color(BG);
        b.set_label_color(FG);
        b.set_selection_color(BG_SEL);
        // convert text to lines
        let lines: Vec<String> = txt.split("\n").map(|line| line.to_string()).collect();

        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                continue;
            }
            // ugly stuff..., changing colors based on selection
            let row = format!("@C3148545792 {}\t@C3148545792 ", i)
                + line.split("\n").collect::<Vec<&str>>()[0];
            let mut row = row.chars().take(80).collect::<String>();
            let lines = line.split("\n").count();
            if lines > 1 {
                row = format!("{} \t@C3722304768({} more lines in buffer...)", row, lines);
            }

            let data = line.clone();

            // add the item to the browser, text is just short representation and data is the actual CB data
            b.add_with_data(&row, data);
            b.set_callback(move |b| {
                // format rows, on selection
                format_rows(b);
                if app::event_mouse_button() == app::MouseButton::Right {
                    unsafe {
                        let idx: i32 = b.value();
                        let d = b.data::<String>(idx);
                        if let Some(d) = d {
                            if !d.contains("\n") {
                                return;
                            }

                            let popup = MyPopup::new(
                                &d.to_string(),
                                app::event_x_root(),
                                app::event_y_root(),
                            );
                            let mut w = popup.window;
                            w.handle(move |w, ev| {
                                if ev == enums::Event::Unfocus {
                                    w.hide();
                                    true
                                } else {
                                    false
                                }
                            });
                        }
                    }
                } else if app::event_clicks_num() > 0 {
                    // on double click, copy the selected item to the clipboard
                    unsafe {
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        let idx: i32 = b.value();
                        let d = b.data::<String>(idx);
                        ctx.set_contents(d.expect("").to_string()).unwrap();
                    }
                }
            });
        }
        b.select(1);
        let row = b.text(1).unwrap().replace("@C3148545792", "@C879871488");
        b.set_text(1, &row);
        b.handle(move |b, _ev| {
            if app::event_key() == Key::from_char('p') {
                false
            } else {
                let mut cb_item_preview_win = b.parent().unwrap();
                if app::event_key() == Key::Escape {
                    cb_item_preview_win.hide();
                    let mut main_win = app::first_window().unwrap();
                    main_win.take_focus().unwrap();
                    false
                } else {
                    true
                }
            }
        });
        window.end();
        window.show();
        Self { window }
    }
}

// format rows, on selection, colorize the selected row
fn format_rows(b: &mut browser::HoldBrowser) {
    for i in 1..b.size() + 1 {
        let row = b.text(i).unwrap().replace("@C879871488", "@C3148545792");
        b.set_text(i, &row);

        let idx = b.value();
        if idx > 0 {
            let row = b.text(idx).unwrap().replace("@C3148545792", "@C879871488");
            b.set_text(idx, &row);
        }
    }
}
