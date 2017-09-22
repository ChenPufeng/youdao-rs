extern crate base64;
extern crate regex;
extern crate reqwest;
extern crate serde_json as json;

extern crate gtk;
extern crate glib;
extern crate gdk;

extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use gtk::prelude::*;
use gtk::{Builder, TextBuffer, TextView, Entry, EntryBuffer, Button, Window, WindowType};
use std::sync::mpsc::{channel, Receiver};
use std::cell::RefCell;
use std::io::{self, Write};

mod youdao;
mod kugou;
mod gtkui;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut gui = false;
    if args.len() == 1 {
        println!("youdao 0.0.1 to_alen@sina.com");
        let mut rl = Editor::<()>::new();
        //if let Err(_) = rl.load_history("history.txt") {
        //    println!("No previous history.");
        //}
        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    if line.len() == 0 {
                        continue;
                    }
                    rl.add_history_entry(&line);
                    let res = match youdao::query(line.as_str()) {
                        Some(x) => x.join("\n"),
                        None => {
                            println!("not found");
                            return;
                        }
                    };
                    println!("{}", res);
                }
                Err(ReadlineError::Interrupted) => {
                    break;
                }
                Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        // rl.save_history("history.txt").unwrap();

        std::process::exit(0x0000);
    } else {
        for i in 1..args.len() {
            let arg = &args[i][..];
            match arg {
                "--gui" => {
                    gui = true;
                    break;
                }
                "--version" | "-v" => {
                    println!("youdao dict v1.0");
                    return;
                }
                "--help" | "-h" => {
                    println!("usage: {} <option> [word]", args[0]);
                    println!("  option:");
                    println!("      --gui          start with gtk front-end");
                    println!("      -h, --help     show help information");
                    println!("      -v, --version  show version information");
                    return;
                }
                _ => {
                    let res = match youdao::query(args[1].as_str()) {
                        Some(x) => x.join("\n"),
                        None => {
                            println!("not found");
                            return;
                        }
                    };
                    println!("{}", res);
                    return;
                }
            }
        }
    }
    if gui {
        gtkui::run(youdao::query);
    }
}
