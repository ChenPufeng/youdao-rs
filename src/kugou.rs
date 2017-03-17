extern crate curl;
use curl::easy::Easy;

extern crate serde_json;
use self::serde_json::Value;


extern crate base64;
use self::base64::decode;

pub fn lyrics_download(id:&str, accesskey:&str) {
    let url_str: String = format!("http://lyrics.kugou.com/download?ver=1&client=pc&id={}&accesskey={}&fmt=lrc&charset=utf8", id, accesskey);
    // println!("REQUEST:{:?}", url_str);
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
    // println!("{}", easy.response_code().unwrap());

    let html = String::from_utf8(respond).unwrap();
    // println!("lrc:{}", html);

    let lrc = serde_json::from_str::<Value>(&html[..]).unwrap();
    if lrc["status"].as_i64().unwrap() == 200 {
        // let mut config = STANDARD;
        println!("content:{}", String::from_utf8(decode(lrc["content"].as_str().unwrap()).unwrap()).unwrap());
    } else {
        println!("bad json object: {}", lrc["info"]);
    }
}

pub fn lyrics_search(name:&str, msec:i32) {
   // println!("Music Name: {:?}", name); 
    let mut w : String = String::new();
    let ba = name.as_bytes();
    if ba[0] > 127 {
        for i in ba {
            w = format!("{}%{:02X}",w, i);
        }
    } else {
        w = name.to_string();
    }
    let url_str: String = format!("http://lyrics.kugou.com/search?ver=1&man=yes&client=pc&keyword={}&duration={}&hash=", w, msec);
    // println!("REQUEST:{:?}", url_str);
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
    // println!("{}", easy.response_code().unwrap());

    let html = String::from_utf8(respond).unwrap();
    // println!("RESULT:{:?}", html);
    // return  parser(&html);
    //
    // use serde_json::Value;

    let v = serde_json::from_str::<Value>(&html[..]).unwrap();
    // println!("JSON OBJECT:{:?}", v);
    println!("JSON: {:?} {:?} {:?} {:?} ", v["info"], v["status"], v["proposal"], v["keyword"]);

    if v["status"].as_i64().unwrap() == 200 {
    for ele in v["candidates"].as_array().unwrap() {
        //println!("ELEMENT: {:?}", ele);
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
        lyrics_download(ele["id"].as_str().unwrap(), ele["accesskey"].as_str().unwrap());
        
    }
    } else {
        println!("bad json object: {}", v["info"]);
    }
}
