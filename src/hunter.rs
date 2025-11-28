use crate::{hunter::errors::HunterError, parsing::{extract_forms, extract_urls, full_url_for_target}, requestbin::{RBin, RequestBin}};
use std::{collections::{HashMap, HashSet}, str::FromStr};
use reqwest::{header::HeaderMap, Client, IntoUrl, Method, StatusCode};
mod settings;
pub mod errors;
use crate::forms::{Form, TrackedField};
const DEFAULT_USER_AGENT: &str= "bughunter/v0.1";
use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};
use anyhow::Result;

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}


#[derive(Debug, Clone)]
pub struct Page {
    pub _url: String,
    pub _status: StatusCode,
    pub _html: String,
    pub urls: HashSet<String>,
    pub forms: Vec<Form>
}

#[derive(Debug)]
pub struct HunterClient {
    webclient: Client,
    pub scoped: HashSet<String>,
    pub current_target: Option<String>,
    pub user_agent: String,
    pub cookies: HeaderMap,
    pub visited: HashMap<String, Page>,
    pub requestbin: Option<RequestBin>,
    pub tracked_fields: HashMap<String, TrackedField>,
    pub known_vulnerable_fields: HashMap<String, TrackedField>
}
impl HunterClient {
    pub fn new() -> Result<Self, HunterError> {
        let builder = Client::builder()
            .user_agent(DEFAULT_USER_AGENT);
        Ok(Self {
            webclient: builder.build()?,
            scoped: HashSet::new(),
            current_target: None,
            user_agent: String::from(DEFAULT_USER_AGENT),
            cookies: HeaderMap::new(),
            visited: HashMap::new(),
            requestbin: None,
            tracked_fields: HashMap::new(),
            known_vulnerable_fields: HashMap::new()
        })
    }
    fn update_params(&mut self) -> Result<(), HunterError> {
        let builder = Client::builder()
            .user_agent(self.user_agent.as_str())
            .default_headers(self.cookies.clone());
        self.webclient = builder.build()?;
        Ok(())
    }
    pub async fn make_request(&mut self,
        method: impl AsRef<str>, url: impl AsRef<str>+IntoUrl, json: Option<HashMap<String, String>>)
            -> Result<Page, HunterError> {
        if !self.is_in_scope(url.as_ref()) {
            return Err(HunterError::out_of_scope(url.as_ref()));
        }
        let san_method = Method::from_str(method.as_ref())?;
        let mut request = self.webclient.request(san_method, url.as_ref());
        match json {
            Some(ref data) => { request = request.form(&data); },
            None => {}
        };
        let response = request.send().await?;
        let status = response.status();
        let html = response.text().await?;
        let mut urls = HashSet::new();
        for endpoint in extract_urls(html.as_str()).iter() {
            urls.insert(full_url_for_target(endpoint, url.as_ref())?);
        }
        let mut forms = Vec::new();
        for form in extract_forms(html.as_str()).iter() {
            forms.push(Form {
                method: form.method.clone(),
                action: full_url_for_target(form.action.clone(), url.as_ref())?,
                fields: form.fields.clone()
            });
        }
        let page = Page {
                _url: url.as_ref().to_string(),
                _status: status,
                _html: html.clone(),
                urls: urls,
                forms: forms
            };
        self.visited.insert(url.as_ref().to_string(), page.clone());
        Ok(page)
    }

    pub async fn _get_html(&mut self, url: impl AsRef<str>+IntoUrl) -> Result<String, HunterError> {
        Ok(self.make_request("GET", url, None).await?._html)
    }

    pub async fn _fetch_urls(&mut self, url: impl AsRef<str>+IntoUrl) -> Result<HashSet<String>, HunterError> {
        Ok(self.make_request("GET", url.as_ref(), None).await?.urls)
    }

    pub async fn _fetch_forms(&mut self, url: impl AsRef<str>+IntoUrl) -> Result<Vec<Form>, HunterError> {
        Ok(self.make_request("GET", url, None).await?.forms)
    }

    pub async fn crawl(&mut self, depth: usize) -> Result<(), HunterError> {
        let target = self.current_target.clone().ok_or(HunterError::no_target())?;
        let _ = self.make_request("GET", target, None).await;
        if depth==0 {
            return Ok(());
        }
        for page in self.visited.clone().values() {
            for target in page.urls.clone().iter() {
                if !self.visited.contains_key(target) {
                    let _ = self.make_request("GET", target.clone(), None).await;
                }
            }
        }
        let _ = Box::pin(self.crawl(depth-1)).await;
        Ok(())
    }

    pub fn known_forms(&self) -> Vec<Form> {
        let mut all_forms = Vec::new();
        for forms in self.visited.values().map(|page| {page.forms.clone()}) {
            for form in forms.iter() {
                if !all_forms.contains(form) {
                    all_forms.push(form.clone());
                }
            }
        }
        all_forms
    }
    pub async fn track_form(&mut self, form: &Form) -> Result<(), HunterError> {
        let requestbin = self.requestbin.as_ref().ok_or(HunterError::no_rbin())?;
        let mut form_data_to_send = HashMap::new();
        let mut new_trackers = HashMap::<String, TrackedField>::new();
        for field_name in form.fields.keys() {
            let tracked_field = TrackedField::new(form.clone(), field_name.clone());
            let payload = requestbin.get_payload(tracked_field.get_id());
            new_trackers.insert(tracked_field.get_id(), tracked_field);
            form_data_to_send.insert(field_name.clone(), payload);
        }
        let _ = self.make_request(form.method.clone(), form.action.clone(),
            Some(form_data_to_send.clone())).await?;
        self.tracked_fields.extend(new_trackers);
        Ok(())
    }
    pub async fn update_known_vulnerable_fields(&mut self) -> Result<bool, HunterError> {
        let data = self.requestbin.as_ref().ok_or(HunterError::no_rbin())?.read().await?;
        let mut new_field_found = false;
        for id in data.iter() {
            if !self.known_vulnerable_fields.contains_key(id) {
                match self.tracked_fields.get(id) {
                    Some(v) => {
                        self.known_vulnerable_fields.insert(id.to_string(), v.to_owned());
                        new_field_found=true;
                    },
                    None => {}
                }
            }
        }
        Ok(new_field_found)
    }
    pub async fn track_all_forms_and_wait(&mut self) -> Result<(), HunterError> {
        for form in self.known_forms().iter() {
            self.track_form(form).await?;
        }
        let ctrl_c_events = ctrl_channel()?;
        let ticks = tick(Duration::from_secs(1));
        println!("[!] Listening (you can visit some web pages to activate JS and CTRL+C when you're done)...");
        loop {
            select! {
                recv(ticks) -> _ => {
                    if self.update_known_vulnerable_fields().await? {
                        println!("[+] Found new vulnerable field(s).");
                    }
                }
                recv(ctrl_c_events) -> _ => {
                    println!("\r[+] Recieved SIGKILL, stopping...");
                    break;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() -> Result<(), HunterError> {
        HunterClient::new()?;
        Ok(())
    }

    #[test]
    fn test_update_params() -> Result<(), HunterError>{
        let mut client = HunterClient::new()?;
        client.update_params()
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_make_request() -> Result<(), HunterError> {
        let mut client = HunterClient::new()?;
        let _ = client.scope("https://httpbin.org/ip");
        client.make_request("GET", "https://httpbin.org/ip", None).await?;
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_get_html() -> Result<(), HunterError> {
        let mut client = HunterClient::new()?;
        let _ = client.scope("https://httpbin.org/ip");
        match client._get_html("https://httpbin.org/ip").await {
            Ok(_) => { Ok(()) },
            Err(e) => { Err(e) }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_fetch_urls() -> Result<(), HunterError> {
        let mut client = HunterClient::new()?;
        let _ = client.scope("https://httpbin.org/ip");
        let urls = client._fetch_urls("https://httpbin.org/ip").await?;
        assert_eq!(urls, HashSet::<String>::new());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_fetch_forms() -> Result<(), HunterError> {
        let mut client = HunterClient::new()?;
        let _ = client.scope("https://httpbin.org/ip");
        let urls = client._fetch_forms("https://httpbin.org/ip").await?;
        assert_eq!(urls, Vec::<Form>::new());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_crawl() -> Result<(), HunterError> {
        let mut client = HunterClient::new()?;
        let _ = client.scope("https://httpbin.org/ip");
        let _ = client.set_target("https://httpbin.org/ip");
        let _ = client.crawl(0).await;
        assert_eq!(client.visited.len(), 1);
        Ok(())
    }
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_known_forms() -> Result<(), HunterError> {
        let client = HunterClient::new()?;
        assert_eq!(client.known_forms(), Vec::<Form>::new());
        Ok(())
    }
}

