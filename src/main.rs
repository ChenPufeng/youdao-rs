extern crate gtk;
extern crate glib;
extern crate gdk;
use gtk::prelude::*;
use gtk::{Builder, TextBuffer, TextView, Entry, EntryBuffer, Button, Window, WindowType};
use std::sync::mpsc::{channel, Receiver};
use std::cell::RefCell;


//extern crate regex;
//use regex::Regex;

use std::thread;

mod db;
mod youdao;
mod kugou;
mod mplayer;

fn main() {
	//mplayer::init();
    db::db_init();
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("dict.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).unwrap();
/*
    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 4);
    let button = Button::new_with_label("search");
    let buffer = EntryBuffer::new(Some("hello"));
    let edit = Entry::new_with_buffer(&buffer);
    //let viewBuffer = TextBuffer::new(None);
    let view = TextView::new(); //_with_buffer(&viewBuffer);
    view.set_editable(false);
    hbox.add(&edit);
    hbox.add(&button);
    vbox.add(&hbox);
    vbox.add(&view);
    window.add(&vbox);
*/
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

    entry.connect_key_release_event(move|s,evkey| {
        let key = evkey.get_keyval();
        if key ==  gdk::enums::key::Return || key == gdk::enums::key::KP_Enter{
            let word = s.get_text().unwrap();
            let word_rel = word.trim().to_string();
            if !word_rel.is_empty() {
            uitx.send(word.trim().to_string()).unwrap();
            }
        }
        Inhibit(false)
    });


    button.connect_clicked(move|_| {
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
            Ok(msg)=> word = msg,
            Err(err)=> return
        }
        let res = youdao::query2(word);
        let resstr = res.join("\n");
        println!("{}", resstr);
        // send result to channel
        tx.send(resstr).unwrap();

        // receive will be run on the main thread
        glib::idle_add(receive);
        }
    });

    gtk::main();
    let res = q.join();
}

fn receive() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref buf, ref rx)) = *global.borrow() {
            if let Ok(text) = rx.try_recv() {
                buf.set_text(&text);
            }
        }
    });
    glib::Continue(false)
}

// declare a new thread local storage key
thread_local!(
    static GLOBAL: RefCell<Option<(gtk::TextBuffer, Receiver<String>)>> = RefCell::new(None)
);
