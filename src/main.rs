extern crate base64;
extern crate regex;
extern crate reqwest;
extern crate serde_json as json;

extern crate gtk;
extern crate glib;
extern crate gdk;

use gtk::prelude::*;
use gtk::{Builder, TextBuffer, TextView, Entry, EntryBuffer, Button, Window, WindowType};
use std::sync::mpsc::{channel, Receiver};
use std::cell::RefCell;

use std::thread;

mod youdao;
mod kugou;
mod gtkui;

fn main() {
    gtkui::run(youdao::query);
}
