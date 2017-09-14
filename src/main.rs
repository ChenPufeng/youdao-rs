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
use std::io::{self, Write};
mod youdao;
mod kugou;
mod gtkui;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut gui = false;
    if args.len() == 1 {
        println!("Input world to continue, input Q/q to quit.");
        loop {
            let mut line = String::new();
            let input = std::io::stdin().read_line(&mut line).expect(
                "Failed to read line",
            );
            let res = match youdao::query(line.as_str()) {
                Some(x) => x.join("\n"),
                None => {
                    println!("not found");
                    return;
                }
            };
            println!("{}", res);
        }
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
