extern crate reqwest;

#[derive(Debug, Clone, Copy)]
pub struct ResultValue {
    data: String,
}

#[derive(Debug, Clone, Copy)]
pub struct SearchOption<T> {
    url: String,
    parameters: Option<T>
}

impl std::fmt::Debug for SearchOption where {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}", url)   
    }
}

impl SearchOption<T> {
    pub fn new<T>(url: &str, t: T)->SearchOption<T>{
        SearchOption {
            url:url,
            parameters:Some(t),
        }
    }
}

pub fn search<T>(optio:T) -> ResultValue {
    ResultValue {
        data: String::new(),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn search_option_new() {
       let option = SearchOption::new("http://baidu.com", "bid=1231");
        println!("{:?}", option);
    }
}