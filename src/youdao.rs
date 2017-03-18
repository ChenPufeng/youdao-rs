extern crate curl;
use self::curl::easy::Easy;

extern crate scraper;
use self::scraper::{Html, Selector};

fn parser(html:&String) -> Vec<String> {
    let mut res = Vec::<String>::new();
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("div#phrsListTab div.trans-container ul li").unwrap();
    let selector_zh = Selector::parse("div#phrsListTab div.trans-container ul p.wordGroup span.contentTitle a").unwrap();
    let selector_mo = Selector::parse("div#authDictTrans ul li span.wordGroup").unwrap();

    // web
    for element in fragment.select(&selector) {
        // println!("trans-container:{:#?}", element.inner_html());
        res.push(element.inner_html());
    }

    // chinese
    for element in fragment.select(&selector_zh) {
        res.push(element.inner_html());
    }

    /* // dict
    for element in fragment.select(&selector_mo) {
        res.push(element.inner_html());
    }
    */

    return res;
}

pub fn query(word:String) -> Vec<String> {
    let mut w : String = String::new();
    let ba = word.as_bytes();
    if ba[0] > 127 {
        for i in ba {
            w = format!("{}%{:02X}",w, i);
        }
    } else {
        w = word.clone();
    }
    let url_str: String = format!("http://dict.youdao.com/search?q={}&keyfrom=dict", w);

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

