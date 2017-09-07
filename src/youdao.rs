extern crate reqwest;
extern crate scraper;
extern crate serde_json;

use self::scraper::{Html, Selector};
use self::serde_json::Value;

use std::io::Read;

fn parser(html: &String) -> Option<Vec<String>> {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("div#phrsListTab div.trans-container ul li").unwrap();
    let selector_zh = Selector::parse(
        "div#phrsListTab div.trans-container ul p.wordGroup \
                                       span.contentTitle a",
    ).unwrap();
    let selector_mo = Selector::parse("div#authDictTrans ul li span.wordGroup").unwrap();

    let mut res = Vec::<String>::new();
    // web
    for element in fragment.select(&selector) {
        // println!("trans-container:{:#?}", element.inner_html());
        res.push(element.inner_html());
    }

    // chinese
    for element in fragment.select(&selector_zh) {
        res.push(element.inner_html());
    }

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

    // println!("{}", easy.response_code().unwrap());

    let r = parser(&body);
    return r;
}


fn parser2(html: &String) -> Option<Vec<String>> {
    let mut res = Vec::<String>::new();


    let trans = serde_json::from_str::<Value>(&html[..]).unwrap();
    if trans["errorCode"].as_i64().unwrap() == 0 {
        // let mut config = STANDARD;
        // println!("content:{}", String::from_utf8(decode(lrc["content"].as_str().unwrap()).unwrap()).unwrap());
        match trans["basic"].as_object() {
            Option::Some(base) => {
                // let explains = base["explains"].as_array().unwrap()
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
    let url: String = format!(
        "http://fanyi.youdao.com/openapi.\
                                   do?keyfrom=f2ec-org&key=1787962561&type=data&doctype=json&version=1.\
                                   1&q={}",
        w
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
            return None;
        }
    };

    let mut body = String::new();
    res.read_to_string(&mut body);
    // println!("{}", easy.response_code().unwrap());

    return parser2(&body);
}
