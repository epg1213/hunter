use std::collections::{HashMap, HashSet};
use base_url::BaseUrl;
use scraper::{Html, Selector};
use crate::forms::Form;

pub fn extract_urls(html: impl AsRef<str>) -> HashSet<String> {
    let fragment = Html::parse_fragment(html.as_ref());
    let selector = match Selector::parse("a") {
        Ok(v) => {v},
        Err(_) => {return HashSet::new();}
    };
    let mut urls = HashSet::new();
    for element in fragment.select(&selector) {
        match element.value().attr("href") {
            Some(href) => { urls.insert(String::from(href)); },
            None => {}
        };
    }
    urls
}

pub fn extract_forms(html: impl AsRef<str>) -> Vec<Form> {
    let fragment = Html::parse_fragment(html.as_ref());
    let selector = match Selector::parse("form") {
        Ok(v) => {v},
        Err(_) => {return Vec::<Form>::new();}
    };
    let mut forms = Vec::<Form>::new();
    for element in fragment.select(&selector) {
        let action = match element.value().attr("action"){
            Some(string) => { string },
            None => { "/" }
        };
        let method = match element.value().attr("method"){
            Some(string) => { string },
            None => { "GET" }
        };
        let subfrag = Html::parse_fragment(element.inner_html().as_str());
        let select = match Selector::parse("input") {
            Ok(v) => {v},
            Err(_) => {return Vec::<Form>::new();}
        };
        let mut fields = HashMap::<String, String>::new();
        for input in subfrag.select(&select) {
            let field_type = match input.value().attr("type"){
                Some(string) => { string },
                None => { "" }
            };
            match input.value().attr("name"){
                Some(string) => { fields.insert(String::from(string), String::from(field_type)); },
                None => {  }
            };
        }
        forms.push(Form {
            action: String::from(action),
            method: String::from(method),
            fields: fields });
    }
    forms
}

pub fn base_url(url: impl AsRef<str>) -> Result<String, String> {
    let mut url_full = match BaseUrl::try_from(url.as_ref()){
        Ok(parsed) => { parsed },
        Err(_) => { return Err(format!("Can't parse url: {}", url.as_ref())); }
    };
    let port = url_full.port();
    url_full.make_host_only();
    url_full.set_port(port);
    Ok(String::from(url_full.as_str()))
}

pub fn full_url_for_target(endpoint: impl AsRef<str>, target: impl AsRef<str>) -> Result<String, String> {
    let mut string_endpoint = String::from(endpoint.as_ref());
    if string_endpoint.contains("://") {
        return Ok(string_endpoint);
    }
    if string_endpoint.starts_with("/") {
        string_endpoint.remove(0);
    }
    let mut base_url = base_url(target)?;
    base_url.push_str(string_endpoint.as_str());
    Ok(base_url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_form() {
        let html = "<form method='POST' action='/'>\
                        <input type='text' name='comment'/>\
                        <input type='submit' value='send'/>\
                    </form>\
                    <form method='PUT' action='/admin'>\
                        <input type='text' name='user'/>\
                        <input type='password' name='pass'/>\
                        <input type='submit' value='submit'/>\
                    </form>";
        let mut expected_forms=Vec::new();
        let mut fields1=HashMap::new();
        fields1.insert(String::from("comment"), String::from("text"));

        expected_forms.push(Form {
            action: String::from("/"),
            method: String::from("POST"),
            fields: fields1
        });
        let mut fields2=HashMap::new();
        fields2.insert(String::from("user"), String::from("text"));
        fields2.insert(String::from("pass"), String::from("password"));
        expected_forms.push(Form {
            action: String::from("/admin"),
            method: String::from("PUT"),
            fields: fields2
        });
        assert_eq!(expected_forms, extract_forms(html));
    }

    #[test]
    fn test_extract_urls() {
        let html = "<p>example</p>\
            <p><a href='http://127.0.0.1/'>localhost</a></p>\
            <p><a href='/index.html'>index</a></p>";
        let mut expected_urls = HashSet::new();
        expected_urls.insert(String::from("http://127.0.0.1/"));
        expected_urls.insert(String::from("/index.html"));
        assert_eq!(expected_urls, extract_urls(html));
    }

    #[test]
    fn test_base_url() -> Result<(), String> {
        let url1 = "http://127.0.0.1:8080/testing/test.html";
        assert_eq!(base_url(url1)?, "http://127.0.0.1:8080/");
        let url2 = "https://127.0.0.1:443";
        assert_eq!(base_url(url2)?, "https://127.0.0.1/");
        Ok(())
    }

    #[test]
    fn test_full_url_for_target() -> Result<(), String> {
        let target = "http://127.0.0.1";
        let endpoint1 = "https://google.com";
        assert_eq!(full_url_for_target(endpoint1, target)?, "https://google.com");
        let endpoint2 = "/index.html";
        assert_eq!(full_url_for_target(endpoint2, target)?, "http://127.0.0.1/index.html");
        let endpoint3 = "data/index.html";
        assert_eq!(full_url_for_target(endpoint3, target)?, "http://127.0.0.1/data/index.html");
        Ok(())
    }
}
