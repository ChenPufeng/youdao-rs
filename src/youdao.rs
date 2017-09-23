
use regex;
use json;
use reqwest;
use ansi_term::Colour;

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
    // get results contents 
    let div_re = match regex::Regex::new(
        r#"<div id="phrsListTab"[^>]*?>(?s)(.*?)<div id="webTrans""#,
        // class="additional">,
    ) {
        Ok(x) => x,
        Err(x) => {
            println!("Regex: {}", x);
            return None;
        }
    };
    // get translate result
    let li_re = match regex::Regex::new(r#"<li[^>]*?>([^<>].*?)</li>"#) {
        Ok(x) => x,
        Err(x) => {
            println!("Regex: {}", x);
            return None;
        }
    };
    // get search keyword 
    let keyword_re = match regex::Regex::new(r#"<span class="keyword"[^>]*?>(?s)(.+?)</span>"#) {
        Ok(x) => x,
        Err(x) => {
            println!("Regex: {}", x);
            return None;
        }
    };

    // for chinese; get translate result
    let ul_re = match regex::Regex::new(r#"<ul[^>]*?>(?s)(.*?)</ul>"#) {
        Ok(x) => x,
        Err(x) => {
            println!("Regex: {}", x);
            return None;
        }
    };
    // get result content
    let a_re = match regex::Regex::new(r#"<a[^>]*?>(?s)(.*?)</a>"#) {
        Ok(x) => x,
        Err(x) => {
            println!("Regex: {}", x);
            return None;
        }
    };
    // get pronounce for english
    let span_re = match regex::Regex::new(r#"<span[^>]*?>(?s)([a-z.].+?)</span>"#) {
        Ok(x) => x,
        Err(x) => {
            println!("Regex: {}", x);
            return None;
        }
    };
    // get chinese result content
    let p_re = match regex::Regex::new(r#"<p class="wordGroup"[^>]*?>(?s)(.*?)</p>"#) {
        Ok(x) => x,
        Err(x) => {
            println!("Regex: {}", x);
            return None;
        }
    };

    let mut res = Vec::<String>::new();
    for caps in div_re.captures_iter(html.trim()) {
        let lis = match caps.get(1) {
            Some(x) => x.as_str(),
            None => {
                println!("not found phrsListTab");
                return Some(res);
            }
        };

        for caps_keyword in keyword_re.captures_iter(lis.trim()) {
            match caps_keyword.get(1) {
                Some(x) => {
                    let mut line = String::new();
                    line.push_str("  ");
                    line.push_str(x.as_str());
                    res.push(Colour::White.paint(line).to_string());
                    break;
                },
                None => {},
            };
        }

        for caps_li in li_re.captures_iter(lis.trim()) {
            match caps_li.get(1) {
                Some(x) => {
                    let mut line = String::new();
                    line.push_str("  ");
                    line.push_str(x.as_str());
                    res.push(Colour::Cyan.paint(line).to_string());
                    break;
                },
                None => {},
            };
        }

        for cap_p in p_re.captures_iter(lis.trim()) {
            let a = match cap_p.get(1) {
                Some(x) => x.as_str(),
                None => {
                    return Some(res);
                }
            };

            let mut line = String::new();
            line.push_str("  ");
            for cap_span in span_re.captures_iter(a.trim()) {
                let ref mut line_ref = line;
                match cap_span.get(1) {
                    Some(x) => {
                        line_ref.push_str(x.as_str());
                        line_ref.push_str(" ");
                    },
                    None => {}
                };
            }
            for cap_a in a_re.captures_iter(a.trim()) {
                let ref mut line_ref = line;
                match cap_a.get(1) {
                    Some(x) => {
                        if !line_ref.ends_with(". ") && !line_ref.ends_with("  ") {
                            line_ref.push_str("; ");
                        }
                        line_ref.push_str(x.as_str());
                    },
                    None => {}
                };
            }
            if line.len() > 0 {
                res.push(Colour::Cyan.paint(line).to_string());
            }
        }
    }
    return Some(res);
}

pub fn query(word: &str) -> Option<Vec<String>> {
    let url: String = format!("http://dict.youdao.com/search?q={}&keyfrom=dict", word);

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
         1&q={}",
        word
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
    return parser2(&body);
}
