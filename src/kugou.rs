//extern crate base64;
//extern crate reqwest;
//extern crate serde_json as json;

use reqwest;
use json;
use std::io::Read;
use base64;

pub fn lyrics_download(id: &str, accesskey: &str) {
    let url: String = format!(
        "http://lyrics.kugou.com/download?ver=1&client=pc&id={}&accesskey={}&fmt=lrc&charset=utf8",
        id,
        accesskey
    );
    let mut res = match reqwest::Client::builder()
        .unwrap()
        .build()
        .unwrap()
        .get(&url)
        .unwrap()
        .send() {
        Ok(x) => x, 
        Err(_) => {
            return;
        }
    };
    let mut body = String::new();
    res.read_to_string(&mut body);


    let lrc = json::from_str::<json::Value>(&body[..]).unwrap();
    if lrc["status"].as_i64().unwrap() == 200 {
        println!(
            "content:{}",
            String::from_utf8(base64::decode(lrc["content"].as_str().unwrap()).unwrap()).unwrap()
        );
    } else {
        println!("bad json object: {}", lrc["info"]);
    }
}

pub fn lyrics_search(name: &str, msec: i32) {
    let mut w: String = String::new();
    let ba = name.as_bytes();
    if ba[0] > 127 {
        for i in ba {
            w = format!("{}%{:02X}", w, i);
        }
    } else {
        w = name.to_string();
    }
    let url: String = format!(
        "http://lyrics.kugou.com/search?ver=1&man=yes&client=pc&keyword={}&duration={}&hash=",
        w,
        msec
    );
    let mut res = match reqwest::Client::builder()
        .unwrap()
        .build()
        .unwrap()
        .get(&url)
        .unwrap()
        .send() {
        Ok(x) => x, 
        Err(_) => {
            return;
        }
    };
    let mut body = String::new();
    res.read_to_string(&mut body);

    let v = json::from_str::<json::Value>(&body[..]).unwrap();
    println!(
        "JSON: {:?} {:?} {:?} {:?} ",
        v["info"],
        v["status"],
        v["proposal"],
        v["keyword"]
    );

    if v["status"].as_i64().unwrap() == 200 {
        for ele in v["candidates"].as_array().unwrap() {
            /*
        println!("id:{} accesskey:{} duration:{} song:{} soundname:{} singer:{} language:{}", 
                 ele["id"],
                 ele["accesskey"], 
                 ele["duration"],
                 ele["song"],
                 ele["soundname"],
                 ele["singer"],
                 ele["language"],
                 );
                 */
            lyrics_download(
                ele["id"].as_str().unwrap(),
                ele["accesskey"].as_str().unwrap(),
            );

        }
    } else {
        println!("bad json object: {}", v["info"]);
    }
}
