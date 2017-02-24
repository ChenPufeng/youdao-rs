extern crate gtk;
extern crate glib;
use gtk::prelude::*;
use gtk::{Builder, TextBuffer, TextView, Entry, EntryBuffer, Button, Window, WindowType};
use std::sync::mpsc::{channel, Receiver};
use std::cell::RefCell;

extern crate curl;
use curl::easy::Easy;

extern crate scraper;
use scraper::{Html, Selector};

//extern crate regex;
//use regex::Regex;

use std::thread;

fn parser(html:&String) -> Vec<String> {
    let mut res = Vec::<String>::new();
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("div#phrsListTab div.trans-container ul li").unwrap();

    for element in fragment.select(&selector) {
        // println!("trans-container:{:#?}", element.inner_html());
        res.push(element.inner_html()); 
    }
    return res;
}

fn query(word:String) -> Vec<String> {
    let url_str: String = format!("http://dict.youdao.com/search?q={}&keyfrom=dict", word);

    let mut easy = Easy::new();
    let mut respond = Vec::new();
    
    easy.url(&url_str).unwrap();
    {
    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        // save respond
        respond.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
    }
    //println!("{}", easy.response_code().unwrap());

    let html = String::from_utf8(respond).unwrap();
    return  parser(&html);
}

fn main() {
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
    let edit: Entry = builder.get_object("entry").unwrap();
    let button: Button = builder.get_object("search").unwrap();
    window.show_all();

    let (tx, rx) = channel();
    let (uitx, uirx) = channel();
    // put TextBuffer and receiver in thread local storage
    GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((view.get_buffer().unwrap(), rx))
    });

    window.connect_delete_event(|_, _| {
       //let empty = String::new();
       //uitx.send(empty).unwrap();

        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(move|_| {
        let word = edit.get_text().unwrap();
        let word_rel = word.trim().to_string();
        if !word_rel.is_empty() {
            uitx.send(word.trim().to_string()).unwrap();
        }
    });

    let q = thread::spawn(move || {
        while true {
        let word: String;
        match uirx.recv() {
            Ok(msg)=> word = msg,
            Err(err)=> return
        }
        let res = query(word);
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
