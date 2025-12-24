#![allow(non_snake_case)]
#![allow(unused_unsafe)]

use clipboard::{ClipboardContext, ClipboardProvider};
use std::env;
use std::fs;

#[cfg(target_os = "macos")]
use cocoa::appkit::*;
#[cfg(target_os = "macos")]
use cocoa::base::{id, nil, YES, NO};
#[cfg(target_os = "macos")]
use cocoa::foundation::*;
#[cfg(target_os = "macos")]
use core_graphics::display::CGDisplay;
#[cfg(target_os = "macos")]
use objc::declare::ClassDecl;
#[cfg(target_os = "macos")]
use objc::runtime::{Class, Object, Sel};
#[cfg(target_os = "macos")]
use objc::*;

const WIN_W: f64 = 800.0;
const WIN_H: f64 = 400.0;
const MAX_ITEMS: usize = 1000;

#[cfg(target_os = "macos")]
fn load_clipboard_items() -> Vec<String> {
    let home = env::var("HOME").unwrap();
    let path = format!("{}/.copycat", home);

    let mut items: Vec<String> = fs::read_to_string(&path)
        .unwrap_or_default()
        .split("\u{200b}")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .fold(Vec::new(), |mut acc, item| {
            if !acc.contains(&item) {
                acc.push(item);
            }
            acc
        });

    items.truncate(MAX_ITEMS);
    items
}

#[cfg(target_os = "macos")]
unsafe fn create_bg_color() -> id {
    msg_send![class!(NSColor), colorWithRed:0.188f64 green:0.188f64 blue:0.247f64 alpha:1.0f64]
}

#[cfg(target_os = "macos")]
fn main() {
    unsafe {
        let items = load_clipboard_items();
        let bg_color = create_bg_color();

        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);

        let display = CGDisplay::main();
        let bounds = display.bounds();
        let screen_w = bounds.size.width;

        let win_x = (screen_w - WIN_W) / 2.0;
        let win_y = (bounds.size.height - WIN_H) / 2.0;

        let frame = NSRect::new(
            NSPoint::new(win_x, win_y),
            NSSize::new(WIN_W, WIN_H),
        );

        register_custom_panel_class();

        let style_mask = NSWindowStyleMask::NSBorderlessWindowMask;

        let panel_class = Class::get("KeyablePanel").unwrap();
        let panel: id = msg_send![panel_class, alloc];
        let panel: id = msg_send![panel,
            initWithContentRect:frame
            styleMask:style_mask
            backing:NSBackingStoreType::NSBackingStoreBuffered
            defer:NO];

        let _: () = msg_send![panel, setLevel: 3i64];
        let _: () = msg_send![panel, setFloatingPanel: YES];
        let _: () = msg_send![panel, setBecomesKeyOnlyIfNeeded: NO];
        let _: () = msg_send![panel, setHidesOnDeactivate: NO];
        let _: () = msg_send![panel, setWorksWhenModal: YES];

        let behavior: u64 = (1 << 0) | (1 << 8);
        let _: () = msg_send![panel, setCollectionBehavior: behavior];

        let _: () = msg_send![panel, setOpaque: YES];
        let _: () = msg_send![panel, setBackgroundColor: bg_color];
        let _: () = msg_send![panel, setMovableByWindowBackground: YES];
        let _: () = msg_send![panel, setHasShadow: YES];

        let content_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(WIN_W, WIN_H));
        let content_view: id = msg_send![class!(NSView), alloc];
        let content_view: id = msg_send![content_view, initWithFrame: content_frame];
        let _: () = msg_send![content_view, setWantsLayer: YES];
        let layer: id = msg_send![content_view, layer];
        let cg_color: id = msg_send![bg_color, CGColor];
        let _: () = msg_send![layer, setBackgroundColor: cg_color];

        let overlay_view: id = msg_send![class!(NSView), alloc];
        let overlay_view: id = msg_send![overlay_view, initWithFrame: content_frame];
        let _: () = msg_send![overlay_view, setWantsLayer: YES];
        let overlay_layer: id = msg_send![overlay_view, layer];
        let overlay_color: id = msg_send![class!(NSColor), colorWithRed:0.0f64 green:0.0f64 blue:0.0f64 alpha:0.5f64];
        let overlay_cg: id = msg_send![overlay_color, CGColor];
        let _: () = msg_send![overlay_layer, setBackgroundColor: overlay_cg];
        let _: () = msg_send![overlay_view, setHidden: YES];

        let search_bar_height: f64 = 28.0;
        let search_field_height: f64 = 22.0;
        let search_y_offset: f64 = (search_bar_height - search_field_height) / 2.0;
        let search_frame = NSRect::new(
            NSPoint::new(8.0, WIN_H - search_bar_height + search_y_offset),
            NSSize::new(WIN_W - 16.0, search_field_height)
        );
        let search_field: id = msg_send![class!(NSTextField), alloc];
        let search_field: id = msg_send![search_field, initWithFrame: search_frame];
        let _: () = msg_send![search_field, setBordered: NO];
        let _: () = msg_send![search_field, setBezeled: NO];
        let _: () = msg_send![search_field, setDrawsBackground: YES];
        let search_bg: id = msg_send![class!(NSColor), colorWithRed:0.25f64 green:0.25f64 blue:0.32f64 alpha:1.0f64];
        let _: () = msg_send![search_field, setBackgroundColor: search_bg];
        let fg_color: id = msg_send![class!(NSColor), colorWithRed:0.733f64 green:0.667f64 blue:1.0f64 alpha:1.0f64];
        let _: () = msg_send![search_field, setTextColor: fg_color];
        let _: () = msg_send![search_field, setFocusRingType: 1i64];
        let _: () = msg_send![search_field, setEditable: YES];
        let _: () = msg_send![search_field, setSelectable: YES];
        let placeholder = NSString::alloc(nil).init_str("Search...");
        let _: () = msg_send![search_field, setPlaceholderString: placeholder];
        let _: () = msg_send![search_field, setHidden: YES];
        let search_font: id = msg_send![class!(NSFont), monospacedSystemFontOfSize:14.0f64 weight:0.0f64];
        let _: () = msg_send![search_field, setFont: search_font];
        let _: () = msg_send![content_view, addSubview: search_field];

        let scroll_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(WIN_W, WIN_H));
        let scroll_view: id = msg_send![class!(NSScrollView), alloc];
        let scroll_view: id = msg_send![scroll_view, initWithFrame: scroll_frame];
        let _: () = msg_send![scroll_view, setHasVerticalScroller: YES];
        let _: () = msg_send![scroll_view, setHasHorizontalScroller: NO];
        let _: () = msg_send![scroll_view, setBorderType: 0i64];
        let _: () = msg_send![scroll_view, setBackgroundColor: bg_color];
        let _: () = msg_send![scroll_view, setDrawsBackground: YES];

        let table_view: id = msg_send![class!(NSTableView), alloc];
        let table_view: id = msg_send![table_view, initWithFrame: scroll_frame];

        let _: () = msg_send![table_view, setBackgroundColor: bg_color];
        let _: () = msg_send![table_view, setRowHeight: 24.0f64];
        let _: () = msg_send![table_view, setIntercellSpacing: NSSize::new(0.0, 1.0)];
        let _: () = msg_send![table_view, setSelectionHighlightStyle: 0i64];
        let _: () = msg_send![table_view, setGridStyleMask: 0u64];
        let _: () = msg_send![table_view, setHeaderView: nil];
        let _: () = msg_send![table_view, setFocusRingType: 1i64];
        let _: () = msg_send![table_view, setUsesAlternatingRowBackgroundColors: NO];

        let column: id = msg_send![class!(NSTableColumn), alloc];
        let column: id = msg_send![column, initWithIdentifier: NSString::alloc(nil).init_str("content")];
        let _: () = msg_send![column, setWidth: WIN_W];
        let _: () = msg_send![table_view, addTableColumn: column];

        register_table_delegate();

        let delegate_class = Class::get("TableDelegate").unwrap();
        let delegate: id = msg_send![delegate_class, alloc];
        let delegate: id = msg_send![delegate, init];

        let items_array: id = msg_send![class!(NSMutableArray), arrayWithCapacity: items.len()];
        for item in &items {
            let ns_string = NSString::alloc(nil).init_str(item);
            let _: () = msg_send![items_array, addObject: ns_string];
        }
        let filtered_array: id = msg_send![items_array, mutableCopy];
        (*delegate).set_ivar("items", items_array);
        (*delegate).set_ivar("filteredItems", filtered_array);
        (*delegate).set_ivar("panel", panel);
        (*delegate).set_ivar("previewPanel", nil);
        (*delegate).set_ivar("overlayView", overlay_view);
        (*delegate).set_ivar("searchMode", false);

        let _: () = msg_send![table_view, setDelegate: delegate];
        let _: () = msg_send![table_view, setDataSource: delegate];
        let _: () = msg_send![search_field, setDelegate: delegate];

        let _: () = msg_send![table_view, setTarget: delegate];
        let _: () = msg_send![table_view, setDoubleAction: sel!(tableViewDoubleClick:)];

        (*delegate).set_ivar("tableView", table_view);
        (*delegate).set_ivar("scrollView", scroll_view);
        (*delegate).set_ivar("searchField", search_field);

        let _: () = msg_send![scroll_view, setDocumentView: table_view];
        let _: () = msg_send![content_view, addSubview: scroll_view];
        let _: () = msg_send![content_view, addSubview: overlay_view];
        let _: () = msg_send![panel, setContentView: content_view];

        let monitor = create_keyboard_monitor(delegate);
        (*delegate).set_ivar("monitor", monitor);

        let notification_center: id = msg_send![class!(NSNotificationCenter), defaultCenter];
        let notification_name = NSString::alloc(nil).init_str("NSApplicationDidResignActiveNotification");

        let resign_block = block::ConcreteBlock::new(move |_notif: id| {
            let app = NSApp();
            let _: () = msg_send![app, terminate: nil];
        });
        let resign_block = resign_block.copy();

        let _: () = msg_send![notification_center,
            addObserverForName: notification_name
            object: nil
            queue: nil
            usingBlock: &*resign_block];

        let _: () = msg_send![panel, makeKeyAndOrderFront: nil];
        let _: () = msg_send![app, activateIgnoringOtherApps: YES];

        if !items.is_empty() {
            let index_set: id = msg_send![class!(NSIndexSet), indexSetWithIndex: 0usize];
            let _: () = msg_send![table_view, selectRowIndexes: index_set byExtendingSelection: NO];
        }

        let _: () = msg_send![app, run];
    }
}

#[cfg(target_os = "macos")]
fn register_custom_panel_class() {
    unsafe {
        if Class::get("KeyablePanel").is_some() {
            return;
        }

        let superclass = Class::get("NSPanel").unwrap();
        let mut decl = ClassDecl::new("KeyablePanel", superclass).unwrap();

        extern "C" fn can_become_key(_this: &Object, _sel: Sel) -> bool {
            true
        }
        decl.add_method(
            sel!(canBecomeKeyWindow),
            can_become_key as extern "C" fn(&Object, Sel) -> bool,
        );

        extern "C" fn can_become_main(_this: &Object, _sel: Sel) -> bool {
            true
        }
        decl.add_method(
            sel!(canBecomeMainWindow),
            can_become_main as extern "C" fn(&Object, Sel) -> bool,
        );

        decl.register();
    }
}

#[cfg(target_os = "macos")]
fn register_table_delegate() {
    unsafe {
        if Class::get("TableDelegate").is_some() {
            return;
        }

        let superclass = Class::get("NSObject").unwrap();
        let mut decl = ClassDecl::new("TableDelegate", superclass).unwrap();

        decl.add_ivar::<id>("items");
        decl.add_ivar::<id>("filteredItems");
        decl.add_ivar::<id>("panel");
        decl.add_ivar::<id>("tableView");
        decl.add_ivar::<id>("scrollView");
        decl.add_ivar::<id>("searchField");
        decl.add_ivar::<id>("previewPanel");
        decl.add_ivar::<id>("overlayView");
        decl.add_ivar::<id>("monitor");
        decl.add_ivar::<bool>("searchMode");

        decl.add_method(
            sel!(numberOfRowsInTableView:),
            number_of_rows as extern "C" fn(&Object, Sel, id) -> i64,
        );

        decl.add_method(
            sel!(tableView:viewForTableColumn:row:),
            view_for_row as extern "C" fn(&Object, Sel, id, id, i64) -> id,
        );

        decl.add_method(
            sel!(tableView:rowViewForRow:),
            row_view_for_row as extern "C" fn(&Object, Sel, id, i64) -> id,
        );

        decl.add_method(
            sel!(controlTextDidChange:),
            control_text_did_change as extern "C" fn(&Object, Sel, id),
        );

        decl.add_method(
            sel!(tableViewDoubleClick:),
            table_view_double_click as extern "C" fn(&Object, Sel, id),
        );

        decl.register();
    }
}

#[cfg(target_os = "macos")]
extern "C" fn number_of_rows(this: &Object, _sel: Sel, _table_view: id) -> i64 {
    unsafe {
        let filtered_items: id = *this.get_ivar("filteredItems");
        let count: usize = msg_send![filtered_items, count];
        count as i64
    }
}

#[cfg(target_os = "macos")]
extern "C" fn row_view_for_row(_this: &Object, _sel: Sel, _table_view: id, _row: i64) -> id {
    unsafe {
        if Class::get("CustomRowView").is_none() {
            let superclass = Class::get("NSTableRowView").unwrap();
            let mut decl = ClassDecl::new("CustomRowView", superclass).unwrap();

            extern "C" fn draw_selection(_this: &Object, _sel: Sel, rect: NSRect) {
                unsafe {
                    let sel_color: id = msg_send![class!(NSColor),
                        colorWithRed:0.353f64 green:0.353f64 blue:0.478f64 alpha:1.0f64];
                    let _: () = msg_send![sel_color, setFill];
                    let _: () = msg_send![class!(NSBezierPath), fillRect: rect];
                }
            }

            decl.add_method(
                sel!(drawSelectionInRect:),
                draw_selection as extern "C" fn(&Object, Sel, NSRect),
            );

            decl.register();
        }

        let row_class = Class::get("CustomRowView").unwrap();
        let row_view: id = msg_send![row_class, alloc];
        let row_view: id = msg_send![row_view, init];
        row_view
    }
}

#[cfg(target_os = "macos")]
extern "C" fn view_for_row(this: &Object, _sel: Sel, table_view: id, _column: id, row: i64) -> id {
    unsafe {
        let filtered_items: id = *this.get_ivar("filteredItems");
        let item: id = msg_send![filtered_items, objectAtIndex: row as usize];

        let fg_color: id = msg_send![class!(NSColor), colorWithRed:0.733f64 green:0.667f64 blue:1.0f64 alpha:1.0f64];

        let identifier = NSString::alloc(nil).init_str("cell");
        let mut cell: id = msg_send![table_view, makeViewWithIdentifier: identifier owner: nil];

        if cell == nil {
            let cell_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(WIN_W, 24.0));
            cell = msg_send![class!(NSTableCellView), alloc];
            cell = msg_send![cell, initWithFrame: cell_frame];
            let _: () = msg_send![cell, setIdentifier: identifier];

            let text_frame = NSRect::new(NSPoint::new(8.0, 3.0), NSSize::new(WIN_W - 16.0, 18.0));
            let text_field: id = msg_send![class!(NSTextField), alloc];
            let text_field: id = msg_send![text_field, initWithFrame: text_frame];
            let _: () = msg_send![text_field, setBezeled: NO];
            let _: () = msg_send![text_field, setDrawsBackground: NO];
            let _: () = msg_send![text_field, setEditable: NO];
            let _: () = msg_send![text_field, setSelectable: NO];
            let _: () = msg_send![text_field, setTextColor: fg_color];

            let font: id = msg_send![class!(NSFont), monospacedSystemFontOfSize:13.0f64 weight:0.0f64];
            let _: () = msg_send![text_field, setFont: font];
            let _: () = msg_send![text_field, setLineBreakMode: 5i64];
            let _: () = msg_send![cell, setTextField: text_field];
            let _: () = msg_send![cell, addSubview: text_field];
        }

        let text_field: id = msg_send![cell, textField];

        let rust_string: String = {
            let ptr: *const i8 = msg_send![item, UTF8String];
            let c_str = std::ffi::CStr::from_ptr(ptr);
            c_str.to_string_lossy().into_owned()
        };

        let first_line = rust_string.lines().next().unwrap_or("");
        let truncated: String = first_line.chars().take(85).collect();
        let line_count = rust_string.lines().count();

        let display = if line_count > 1 {
            format!(" {:>3}  {}  (+{} lines)", row + 1, truncated, line_count - 1)
        } else {
            format!(" {:>3}  {}", row + 1, truncated)
        };

        let ns_display = NSString::alloc(nil).init_str(&display);
        let _: () = msg_send![text_field, setStringValue: ns_display];

        cell
    }
}

#[cfg(target_os = "macos")]
extern "C" fn control_text_did_change(_this: &Object, _sel: Sel, _notification: id) {
}

#[cfg(target_os = "macos")]
extern "C" fn table_view_double_click(this: &Object, _sel: Sel, _sender: id) {
    unsafe {
        let table_view: id = *this.get_ivar("tableView");
        let filtered_items: id = *this.get_ivar("filteredItems");
        let clicked_row: i64 = msg_send![table_view, clickedRow];

        if clicked_row >= 0 {
            let count: i64 = msg_send![filtered_items, count];
            if clicked_row < count {
                let item: id = msg_send![filtered_items, objectAtIndex: clicked_row as usize];
                let ptr: *const i8 = msg_send![item, UTF8String];
                let c_str = std::ffi::CStr::from_ptr(ptr);
                let content = c_str.to_string_lossy().into_owned();

                if let Ok(mut ctx) = ClipboardContext::new() {
                    let _ = ctx.set_contents(content);
                }

                let app = NSApp();
                let _: () = msg_send![app, terminate: nil];
            }
        }
    }
}

#[cfg(target_os = "macos")]
unsafe fn create_keyboard_monitor(delegate: id) -> id {
    let block = block::ConcreteBlock::new(move |event: id| -> id {
        let event_type: u64 = msg_send![event, type];

        if event_type == 10 {
            let keycode: u16 = msg_send![event, keyCode];
            let table_view: id = *(&*delegate as &Object).get_ivar("tableView");
            let search_field: id = *(&*delegate as &Object).get_ivar("searchField");
            let scroll_view: id = *(&*delegate as &Object).get_ivar("scrollView");
            let filtered_items: id = *(&*delegate as &Object).get_ivar("filteredItems");
            let items: id = *(&*delegate as &Object).get_ivar("items");
            let count: i64 = msg_send![filtered_items, count];
            let selected: i64 = msg_send![table_view, selectedRow];

            let search_hidden: i8 = msg_send![search_field, isHidden];
            let in_search_mode = search_hidden == 0;

            if in_search_mode {
                match keycode {
                    53 => {
                        exit_search_mode(delegate, table_view, search_field, scroll_view, items);
                        return nil;
                    }
                    48 => {
                        let _: () = msg_send![search_field, setHidden: YES];
                        let scroll_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(WIN_W, WIN_H));
                        let _: () = msg_send![scroll_view, setFrame: scroll_frame];
                        if count > 0 {
                            let index_set: id = msg_send![class!(NSIndexSet), indexSetWithIndex: 0usize];
                            let _: () = msg_send![table_view, selectRowIndexes: index_set byExtendingSelection: NO];
                            let _: () = msg_send![table_view, scrollRowToVisible: 0i64];
                        }
                        let panel: id = *(&*delegate as &Object).get_ivar("panel");
                        let _: () = msg_send![panel, makeFirstResponder: table_view];
                        return nil;
                    }
                    36 => {
                        if selected >= 0 && selected < count {
                            let item: id = msg_send![filtered_items, objectAtIndex: selected as usize];
                            let ptr: *const i8 = msg_send![item, UTF8String];
                            let c_str = std::ffi::CStr::from_ptr(ptr);
                            let content = c_str.to_string_lossy().into_owned();

                            if let Ok(mut ctx) = ClipboardContext::new() {
                                let _ = ctx.set_contents(content);
                            }

                            let app = NSApp();
                            let _: () = msg_send![app, terminate: nil];
                        }
                        return nil;
                    }
                    51 => {
                        let current: id = msg_send![search_field, stringValue];
                        let len: usize = msg_send![current, length];
                        if len > 0 {
                            let range = NSRange::new((len - 1) as u64, 1);
                            let new_str: id = msg_send![current, stringByReplacingCharactersInRange:range withString:NSString::alloc(nil).init_str("")];
                            let _: () = msg_send![search_field, setStringValue: new_str];

                            let full_search: id = msg_send![search_field, stringValue];
                            let full_ptr: *const i8 = msg_send![full_search, UTF8String];
                            let search_lower = std::ffi::CStr::from_ptr(full_ptr).to_string_lossy().to_lowercase();
                            let all_items: id = *(&*delegate as &Object).get_ivar("items");
                            let items_count: usize = msg_send![all_items, count];
                            let filtered: id = msg_send![class!(NSMutableArray), arrayWithCapacity: items_count];

                            for i in 0..items_count {
                                let item: id = msg_send![all_items, objectAtIndex: i];
                                let item_ptr: *const i8 = msg_send![item, UTF8String];
                                let item_str_val = std::ffi::CStr::from_ptr(item_ptr).to_string_lossy().to_lowercase();
                                if search_lower.is_empty() || item_str_val.contains(&*search_lower) {
                                    let _: () = msg_send![filtered, addObject: item];
                                }
                            }

                            let _: () = msg_send![filtered, retain];
                            let old_filtered: id = *(&*delegate as &Object).get_ivar("filteredItems");
                            if old_filtered != nil {
                                let _: () = msg_send![old_filtered, release];
                            }
                            (&mut *delegate as &mut Object).set_ivar("filteredItems", filtered);
                            let _: () = msg_send![table_view, reloadData];
                        }
                        return nil;
                    }
                    _ => {
                        let chars: id = msg_send![event, characters];
                        if chars != nil {
                            let chars_ptr: *const i8 = msg_send![chars, UTF8String];
                            if !chars_ptr.is_null() {
                                let char_str = std::ffi::CStr::from_ptr(chars_ptr).to_string_lossy();
                                if !char_str.is_empty() && char_str.chars().all(|c| !c.is_control()) {
                                    let current: id = msg_send![search_field, stringValue];
                                    let new_str: id = msg_send![current, stringByAppendingString: chars];
                                    let _: () = msg_send![search_field, setStringValue: new_str];

                                    let full_search: id = msg_send![search_field, stringValue];
                                    let full_ptr: *const i8 = msg_send![full_search, UTF8String];
                                    let search_lower = std::ffi::CStr::from_ptr(full_ptr).to_string_lossy().to_lowercase();
                                    let all_items: id = *(&*delegate as &Object).get_ivar("items");
                                    let items_count: usize = msg_send![all_items, count];
                                    let filtered: id = msg_send![class!(NSMutableArray), arrayWithCapacity: items_count];

                                    for i in 0..items_count {
                                        let item: id = msg_send![all_items, objectAtIndex: i];
                                        let item_ptr: *const i8 = msg_send![item, UTF8String];
                                        let item_str_val = std::ffi::CStr::from_ptr(item_ptr).to_string_lossy().to_lowercase();
                                        if item_str_val.contains(&*search_lower) {
                                            let _: () = msg_send![filtered, addObject: item];
                                        }
                                    }

                                    let _: () = msg_send![filtered, retain];
                                    let old_filtered: id = *(&*delegate as &Object).get_ivar("filteredItems");
                                    if old_filtered != nil {
                                        let _: () = msg_send![old_filtered, release];
                                    }
                                    (&mut *delegate as &mut Object).set_ivar("filteredItems", filtered);
                                    let _: () = msg_send![table_view, reloadData];
                                }
                            }
                        }
                        return nil;
                    }
                }
            }

            let chars: id = msg_send![event, characters];
            let chars_ptr: *const i8 = msg_send![chars, UTF8String];
            let char_str = if chars_ptr.is_null() {
                String::new()
            } else {
                std::ffi::CStr::from_ptr(chars_ptr).to_string_lossy().into_owned()
            };

            if char_str == "/" {
                enter_search_mode(delegate, table_view, search_field, scroll_view);
                return nil;
            }

            match keycode {
                53 | 12 => {
                    close_preview_if_open(delegate);
                    let app = NSApp();
                    let _: () = msg_send![app, terminate: nil];
                    return nil;
                }
                35 => {
                    if selected >= 0 && selected < count {
                        toggle_preview(delegate, table_view, filtered_items, selected);
                    }
                    return nil;
                }
                38 => {
                    close_preview_if_open(delegate);
                    if selected < count - 1 {
                        let new_index: i64 = selected + 1;
                        let index_set: id = msg_send![class!(NSIndexSet), indexSetWithIndex: new_index as usize];
                        let _: () = msg_send![table_view, selectRowIndexes: index_set byExtendingSelection: NO];
                        let _: () = msg_send![table_view, scrollRowToVisible: new_index];
                    }
                    return nil;
                }
                40 => {
                    close_preview_if_open(delegate);
                    if selected > 0 {
                        let new_index: i64 = selected - 1;
                        let index_set: id = msg_send![class!(NSIndexSet), indexSetWithIndex: new_index as usize];
                        let _: () = msg_send![table_view, selectRowIndexes: index_set byExtendingSelection: NO];
                        let _: () = msg_send![table_view, scrollRowToVisible: new_index];
                    }
                    return nil;
                }
                5 => {
                    close_preview_if_open(delegate);
                    let flags: u64 = msg_send![event, modifierFlags];
                    let shift_pressed = (flags & (1 << 17)) != 0;

                    let new_index: i64 = if shift_pressed {
                        count - 1
                    } else {
                        0
                    };

                    if new_index >= 0 && new_index < count {
                        let index_set: id = msg_send![class!(NSIndexSet), indexSetWithIndex: new_index as usize];
                        let _: () = msg_send![table_view, selectRowIndexes: index_set byExtendingSelection: NO];
                        let _: () = msg_send![table_view, scrollRowToVisible: new_index];
                    }
                    return nil;
                }
                36 => {
                    if selected >= 0 && selected < count {
                        let item: id = msg_send![filtered_items, objectAtIndex: selected as usize];
                        let ptr: *const i8 = msg_send![item, UTF8String];
                        let c_str = std::ffi::CStr::from_ptr(ptr);
                        let content = c_str.to_string_lossy().into_owned();

                        if let Ok(mut ctx) = ClipboardContext::new() {
                            let _ = ctx.set_contents(content);
                        }

                        let app = NSApp();
                        let _: () = msg_send![app, terminate: nil];
                    }
                    return nil;
                }
                _ => {}
            }
        }

        event
    });

    let block = block.copy();

    let monitor: id = msg_send![
        class!(NSEvent),
        addLocalMonitorForEventsMatchingMask: (1u64 << 10)
        handler: &*block
    ];

    monitor
}

#[cfg(target_os = "macos")]
unsafe fn enter_search_mode(delegate: id, _table_view: id, search_field: id, scroll_view: id) {
    (&mut *delegate as &mut Object).set_ivar("searchMode", true);

    let search_height: f64 = 28.0;
    let scroll_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(WIN_W, WIN_H - search_height));
    let _: () = msg_send![scroll_view, setFrame: scroll_frame];

    let _: () = msg_send![search_field, setHidden: NO];
    let _: () = msg_send![search_field, setStringValue: NSString::alloc(nil).init_str("")];

    let superview: id = msg_send![search_field, superview];
    let _: () = msg_send![search_field, removeFromSuperview];
    let _: () = msg_send![superview, addSubview: search_field];

    let panel: id = *(&*delegate as &Object).get_ivar("panel");
    let _: () = msg_send![panel, makeKeyAndOrderFront: nil];
    let _: () = msg_send![panel, makeFirstResponder: search_field];
}

#[cfg(target_os = "macos")]
unsafe fn exit_search_mode(delegate: id, table_view: id, search_field: id, scroll_view: id, items: id) {
    (&mut *delegate as &mut Object).set_ivar("searchMode", false);

    let _: () = msg_send![search_field, setHidden: YES];

    let scroll_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(WIN_W, WIN_H));
    let _: () = msg_send![scroll_view, setFrame: scroll_frame];

    let filtered_copy: id = msg_send![items, mutableCopy];
    (&mut *delegate as &mut Object).set_ivar("filteredItems", filtered_copy);
    let _: () = msg_send![table_view, reloadData];

    let count: usize = msg_send![filtered_copy, count];
    if count > 0 {
        let index_set: id = msg_send![class!(NSIndexSet), indexSetWithIndex: 0usize];
        let _: () = msg_send![table_view, selectRowIndexes: index_set byExtendingSelection: NO];
    }

    let panel: id = *(&*delegate as &Object).get_ivar("panel");
    let _: () = msg_send![panel, makeFirstResponder: table_view];
}

#[cfg(target_os = "macos")]
unsafe fn close_preview_if_open(delegate: id) {
    let existing_preview: id = *(&*delegate as &Object).get_ivar("previewPanel");
    if existing_preview != nil {
        let _: () = msg_send![existing_preview, close];
        (&mut *delegate as &mut Object).set_ivar("previewPanel", nil);

        let overlay: id = *(&*delegate as &Object).get_ivar("overlayView");
        let _: () = msg_send![overlay, setHidden: YES];

        let panel: id = *(&*delegate as &Object).get_ivar("panel");
        let _: () = msg_send![panel, makeKeyAndOrderFront: nil];
    }
}

#[cfg(target_os = "macos")]
unsafe fn toggle_preview(delegate: id, table_view: id, filtered_items: id, row: i64) {
    let existing_preview: id = *(&*delegate as &Object).get_ivar("previewPanel");
    if existing_preview != nil {
        let _: () = msg_send![existing_preview, close];
        (&mut *delegate as &mut Object).set_ivar("previewPanel", nil);

        let overlay: id = *(&*delegate as &Object).get_ivar("overlayView");
        let _: () = msg_send![overlay, setHidden: YES];

        let panel: id = *(&*delegate as &Object).get_ivar("panel");
        let _: () = msg_send![panel, makeKeyAndOrderFront: nil];
        return;
    }

    let item: id = msg_send![filtered_items, objectAtIndex: row as usize];
    let ptr: *const i8 = msg_send![item, UTF8String];
    let content = std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned();

    let panel: id = *(&*delegate as &Object).get_ivar("panel");
    let bg_color = create_bg_color();
    let fg_color: id = msg_send![class!(NSColor), colorWithRed:0.733f64 green:0.667f64 blue:1.0f64 alpha:1.0f64];

    let preview_w: f64 = WIN_W;
    let preview_h: f64 = 200.0;

    let panel_frame: NSRect = msg_send![panel, frame];

    let row_rect: NSRect = msg_send![table_view, rectOfRow: row];
    let scroll_view: id = *(&*delegate as &Object).get_ivar("scrollView");
    let visible_rect: NSRect = msg_send![scroll_view, documentVisibleRect];

    let row_visual_y = row_rect.origin.y - visible_rect.origin.y;

    let row_screen_y = panel_frame.origin.y + WIN_H - row_visual_y - 24.0;

    let space_below = row_screen_y - preview_h;

    let preview_y = if space_below > 50.0 {
        row_screen_y - preview_h - 2.0
    } else {
        row_screen_y + 24.0 + 2.0
    };

    let preview_frame = NSRect::new(
        NSPoint::new(panel_frame.origin.x, preview_y),
        NSSize::new(preview_w, preview_h)
    );

    let style_mask = NSWindowStyleMask::NSBorderlessWindowMask;

    let preview_panel: id = msg_send![class!(NSPanel), alloc];
    let preview_panel: id = msg_send![preview_panel,
        initWithContentRect:preview_frame
        styleMask:style_mask
        backing:NSBackingStoreType::NSBackingStoreBuffered
        defer:NO];

    let _: () = msg_send![preview_panel, setBackgroundColor: bg_color];
    let _: () = msg_send![preview_panel, setLevel: 4i64];
    let _: () = msg_send![preview_panel, setHasShadow: YES];

    let content_frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(preview_w, preview_h));
    let scroll: id = msg_send![class!(NSScrollView), alloc];
    let scroll: id = msg_send![scroll, initWithFrame: content_frame];
    let _: () = msg_send![scroll, setHasVerticalScroller: YES];
    let _: () = msg_send![scroll, setBorderType: 0i64];
    let _: () = msg_send![scroll, setBackgroundColor: bg_color];
    let _: () = msg_send![scroll, setDrawsBackground: YES];

    let text_view: id = msg_send![class!(NSTextView), alloc];
    let text_view: id = msg_send![text_view, initWithFrame: content_frame];
    let _: () = msg_send![text_view, setEditable: NO];
    let _: () = msg_send![text_view, setBackgroundColor: bg_color];
    let _: () = msg_send![text_view, setTextColor: fg_color];
    let font: id = msg_send![class!(NSFont), monospacedSystemFontOfSize:13.0f64 weight:0.0f64];
    let _: () = msg_send![text_view, setFont: font];
    let _: () = msg_send![text_view, setString: NSString::alloc(nil).init_str(&content)];

    let _: () = msg_send![scroll, setDocumentView: text_view];
    let preview_content: id = msg_send![preview_panel, contentView];
    let _: () = msg_send![preview_content, addSubview: scroll];

    let overlay: id = *(&*delegate as &Object).get_ivar("overlayView");
    let _: () = msg_send![overlay, setHidden: NO];

    (&mut *delegate as &mut Object).set_ivar("previewPanel", preview_panel);
    let _: () = msg_send![preview_panel, makeKeyAndOrderFront: nil];
}

#[cfg(not(target_os = "macos"))]
fn main() {
    std::process::exit(1);
}
