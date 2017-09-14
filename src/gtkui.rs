use gtk;
use gdk;
use glib;
use gtk::prelude::*;
use gtk::{Builder, TextBuffer, TextView, Entry, EntryBuffer, Button, Window, WindowType};
use std::sync::mpsc::{channel, Receiver};
use std::cell::RefCell;

use std::thread;
use youdao;

pub fn run(f: fn(&str)->Option<Vec<String>>) {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let gladeui = include_str!("dict.glade");
    let builder = Builder::new();
    builder.add_from_string(gladeui).unwrap();

    let window: Window = builder.get_object("window").unwrap();
    let view: TextView = builder.get_object("textview").unwrap();
    let entry: Entry = builder.get_object("entry").unwrap();
    let button: Button = builder.get_object("search").unwrap();
    window.show_all();

    let (tx, rx) = channel();
    let (uitx, uirx) = channel();
    let uitx2 = uitx.clone();
    // put TextBuffer and receiver in thread local storage
    GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((view.get_buffer().unwrap(), rx))
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    entry.connect_key_release_event(move |s, evkey| {
        let key = evkey.get_keyval();
        if key == gdk::enums::key::Return || key == gdk::enums::key::KP_Enter {
            let word = s.get_text().unwrap();
            let word_rel = word.trim().to_string();
            if !word_rel.is_empty() {
                uitx.send(word.trim().to_string()).unwrap();
            }
        }
        Inhibit(false)
    });


    button.connect_clicked(move |_| {
        let word = entry.get_text().unwrap();
        let word_rel = word.trim().to_string();
        if !word_rel.is_empty() {
            uitx2.send(word.trim().to_string()).unwrap();
        }
    });

    let q = thread::spawn(move || {
        while true {
            let word: String;
            match uirx.recv() {
                Ok(msg) => word = msg,
                Err(err) => return,
            }
            let res = match youdao::query(&word) {
                Some(x) => x.join("\n"),
                None => {println!("not found"); return;},
            };
            println!("{}", res);
            // send result to channel
            tx.send(res).unwrap();

            // receive will be run on the main thread
            glib::idle_add(receive);
        }
    });

    gtk::main();
    let res = q.join();
}

fn receive() -> glib::Continue {
    GLOBAL.with(|global| if let Some((ref buf, ref rx)) = *global.borrow() {
        if let Ok(text) = rx.try_recv() {
            buf.set_text(&text);
        }
    });
    glib::Continue(false)
}

// declare a new thread local storage key
thread_local!(
    static GLOBAL: RefCell<Option<(gtk::TextBuffer, Receiver<String>)>> = RefCell::new(None)
);
