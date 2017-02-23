extern crate gtk;
use gtk::prelude::*;
use gtk::{TextBuffer, TextView, Entry, EntryBuffer, Button, Window, WindowType};

extern crate curl;
//use std::io::{stdout, Write};
use curl::easy::Easy;

extern crate scraper;
use scraper::{Html, Selector};
//use select::document::Document;
//use select::predicate::Name;

//extern crate regex;
//use regex::Regex;

use std::thread;
// pub use std::collections::HashMap;

fn parser(html:&String) -> Vec<String> {
    let mut res = Vec::<String>::new();
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("div#phrsListTab div.trans-container ul li").unwrap();

    for element in fragment.select(&selector) {
        // println!("trans-container:{:#?}", element.inner_html());
        res.push(element.inner_html()); 
    }
    return res;
/*
    let doc = Document::from(&html[..]);
    println!("{}", html);
    println!("find trans-container");
    for i in doc.find(Name("trans-container")).iter() {
        println!("trans-container:{:?}",i.text());       //prints text content of all articles
    };
    */
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

    //let res = respond.clone();
    let html = String::from_utf8(respond).unwrap();
    //println!("{:?}", html);
    //let re = Regex::new(r" {2,}").unwrap();
    //let mat = re.find(&html[..]).unwrap();
    //println!("{}", mat.as_str());
    //for mat in re.find_iter(&html[..]) {
    //    println!("{:?}", mat);
    //}

    return  parser(&html);
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 4);
    let button = Button::new_with_label("Click me!");
    let buffer = EntryBuffer::new(Some("hello"));
    let viewBuffer = TextBuffer::new(None);
    let edit = Entry::new_with_buffer(&buffer);
    let view = TextView::new_with_buffer(&viewBuffer);
    view.set_editable(false);
    hbox.add(&edit);
    hbox.add(&button);
    vbox.add(&hbox);
    vbox.add(&view);
    window.add(&vbox);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(move|_| {
        let word = edit.get_text().unwrap();
        //let q = thread::spawn(move || {
            let res = query(word.trim().to_string());
            //println!("QUERY:{:#?}", res);
            let (mut start,mut end) = viewBuffer.get_bounds();
            viewBuffer.delete(&mut start, &mut end);
            for i in res {
                // println!("{}", i);
                viewBuffer.insert_at_cursor(&i[..]);
                viewBuffer.insert_at_cursor("\n");
            }  
        //});
    });

    gtk::main();
    //let res = q.join();
}

