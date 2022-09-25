extern crate clipboard;

use clipboard::{ClipboardContext, ClipboardProvider};
use fltk::{*};
use std::env;
use fltk::{
    app,
    prelude::*,
    window::Window,
};
use std::fs;
use fltk::text::TextDisplay;
use itertools::Itertools;
use fltk::enums::{ Color, Key };

const BG: Color = Color::from_hex(0x30303f);
const FG: Color = Color::from_hex(0xbbaaff);
const BG_SEL: Color = Color::from_hex(0xdddddd);

const WIN_W: i32 = 800;
const WIN_H: i32 = 400;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background2(48, 48, 63);
    app::foreground(187, 170, 255);

    let mut wind = Window::default().with_size(WIN_W, WIN_H).center_screen().with_label("CopyCat");
    wind.set_border(false);
    wind.set_color(BG);

    wind.handle(move |_w, ev| {
        if ev == enums::Event::Unfocus{
            app::add_timeout3(1.00, |_| {
                let l = app::focus();
                if l.is_none() {
                    app::quit();
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

    // On enter, copy the selected item to the clipboard
    b.handle(move |b, ev| {
        if ev == enums::Event::KeyDown && app::event_key() == Key::Enter {
            unsafe {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                let idx: i32 = b.value();
                let d = b.data::<String>(idx);
                ctx.set_contents(d.expect("ClipboardContext to be set to data of selected item!").to_string()).unwrap();
                app.quit();

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

    let lines: Vec<String> = lines.into_iter().unique().rev().map(|line| line.trim().to_string()).collect();

    for (i,line ) in lines.iter().enumerate() {
        if i == 0 {
            continue;
        }
        // ugly stuff..., changing colors based on selection
        let row = format!("@C3148545792 {}\t@C3148545792 ", i) + line.split("\n").collect::<Vec<&str>>()[0];
        let mut row = row.chars().take(80).collect::<String>();
        let lines = line.split("\n").count();
        if lines > 1 {
            row = format!("{} \t@C3722304768({} more lines in buffer...)", row, lines);
        }

        let data = line.clone();

        // add the item to the browser, text is just short representation and data is the actual CB data
        b.add_with_data(&row, data);
        b.set_callback(move |b| {
            for i in 1..b.size()+1 {
                let row = b.text(i).unwrap().replace("@C879871488","@C3148545792");
                b.set_text(i, &row);

                let idx = b.value();
                if idx > 0 {
                    let row = b.text(idx).unwrap().replace("@C3148545792", "@C879871488");
                    b.set_text(idx, &row);

                }
            }
            if app::event_mouse_button() == app::MouseButton::Right {
                unsafe {
                    let idx: i32 = b.value();
                    let d = b.data::<String>(idx);
                    if let Some(d) = d {
                        if !d.contains("\n") {
                            return;
                        }

                        let popup = MyPopup::new(&d.to_string(), app::event_x_root(), app::event_y_root());
                        let mut w = popup.window;
                        w.handle(move |w, ev| {
                            if ev == enums::Event::Unfocus{
                                w.hide();
                                true
                            } else {
                                false
                            }
                        });
                    }
                }
            } else if app::event_clicks_num() > 0 { // on double click, copy the selected item to the clipboard
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
    wind.make_resizable(true);
    wind.show();

    app.run().unwrap();
}

pub struct MyPopup {
    window: window::Window,
}

impl MyPopup {
    pub fn new(txt: &str, x: i32, y: i32) -> Self {
        let mut window = Window::default().with_size(WIN_W - 100, WIN_H - 100).center_screen().with_label("CB Item Preview");
        window.set_pos(x, y);
        window.make_modal(false);
        window.set_border(false);
        window.set_color(BG);

        let mut text = TextDisplay::default().with_size(WIN_W - 100, WIN_H - 100);
        let mut buffer = text::TextBuffer::default();
        buffer.set_text(txt);
        text.set_buffer(Some(buffer));
        window.end();
        window.show();
        Self { window }
    }
}
