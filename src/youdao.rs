
use regex;
use json;
use reqwest;

use std::io::Read;
use std::string::ToString;

#[derive(Debug, Default)]
struct RichText {
    text: String,
    color: i32,
    style: i32,
    font: i32,
}

impl ToString for RichText {
    fn to_string(&self) -> String {
        self.text.clone()
    }
}

fn parser(html: &String) -> Option<Vec<String>> {
    // FIXME match include newline strings
    let div_re = match regex::Regex::new(r#"<div id="phrsListTab"[^>]*?>(.*?)</div>"#) {
        Ok(x) =>x,
        Err(x) => {println!("Regex: {}", x); return None;},
    };
    let li_re = match regex::Regex::new(r#"<li[^>]*?>([^<>].*?)</li>"#){
        Ok(x) =>x,
        Err(x) => {println!("Regex: {}", x); return None;},
    };

    let mut res = Vec::<String>::new();
    for caps in div_re.captures_iter(html.trim()) {
        let lis = match caps.get(1) {
            Some(x) => x.as_str(),
            None => {println!("not found phrsListTab");return None;},
        };
        println!("div {:?}", lis);
        for caps_li in li_re.captures_iter(lis.trim()) {
            println!("li: {}",
            caps_li.get(1).unwrap().as_str());
            res.push(caps_li.get(1).unwrap().as_str().to_string());
        }
    }
    /*
        for caps_li in li_re.captures_iter(html.trim()) {
            println!("li: {}",
            caps_li.get(1).unwrap().as_str());
            res.push(caps_li.get(1).unwrap().as_str().to_string());
        }
    */
    return Some(res);
}

pub fn query(word: String) -> Option<Vec<String>> {
    let mut w: String = String::new();
    let ba = word.as_bytes();
    if ba[0] > 127 {
        for i in ba {
            w = format!("{}%{:02X}", w, i);
        }
    } else {
        w = word.clone();
    }
    let url: String = format!("http://dict.youdao.com/search?q={}&keyfrom=dict", w);

    let mut res = match reqwest::Client::builder()
        .unwrap()
        .build()
        .unwrap()
        .get(&url)
        .unwrap()
        .send() {
        Ok(x) => x, 
        Err(_) => {
            return None;
        }
    };

    let mut body = String::new();
    res.read_to_string(&mut body);

    let r = parser(&body);
    return r;
}


fn parser2(html: &String) -> Option<Vec<String>> {
    let mut res = Vec::<String>::new();
    let trans = json::from_str::<json::Value>(&html[..]).unwrap();
    if trans["errorCode"].as_i64().unwrap() == 0 {
        match trans["basic"].as_object() {
            Option::Some(base) => {
                for element in base["explains"].as_array().unwrap() {
                    res.push(element.as_str().unwrap().to_string());
                    println!("explains:{}", element);
                }
            }
            Option::None => {}
        }

        match trans["web"].as_object() {
            Option::Some(val) => {
                for element in val["value"].as_array().unwrap() {
                    res.push(element.as_str().unwrap().to_string());
                    println!("explains:{}", element);
                }
            }
            Option::None => {}
        }
    } else {
        println!("bad json object");
        return None;
    }

    return Some(res);
}

pub fn query2(word: String) -> Option<Vec<String>> {
    let mut w: String = String::new();
    let ba = word.as_bytes();
    if ba[0] > 127 {
        for i in ba {
            w = format!("{}%{:02X}", w, i);
        }
    } else {
        w = word.clone();
    }
    /*
    let mut params = HashMap::new();
    params.insert("key", "1787962561");
    params.insert("keyfrom", "f2ec-org");
    params.insert("type", "data");
    params.insert("doctype", "json");
    params.insert("version", "1.1");
    params.insert("q", &w);
    let url = "http://fanyi.youdao.com/openapi.do";
    */
    let url: String = format!(
        "http://fanyi.youdao.com/openapi.\
         do?keyfrom=f2ec-org&key=1787962561&type=data&\
         doctype=json&version=1.\
         1&q={}",w);

    let mut res = match reqwest::Client::builder()
        .unwrap()
        .build()
        .unwrap()
        .get(&url)
        .unwrap()
        .send() {
        Ok(x) => x, 
        Err(_) => {
            return None;
        }
    };

    let mut body = String::new();
    res.read_to_string(&mut body);
    return parser2(&body);
}
