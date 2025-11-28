use reqwest::{header::{HeaderMap, HeaderValue}, Url};
use crate::{hunter::HunterClient, hunter::errors::HunterError};
use crate::requestbin::{RequestBin, PublicRequestBin, PrivateRequestBin};

impl HunterClient {
    pub fn set_user_agent(&mut self, user_agent: impl AsRef<str>) -> Result<(), HunterError> {
        self.user_agent = user_agent.as_ref().to_string();
        self.update_params()?;
        Ok(())
    }
    pub fn set_cookies(&mut self, cookies: impl AsRef<str>) -> Result<(), HunterError> {
        let mut headers = HeaderMap::new();
        let cookie = HeaderValue::from_str(cookies.as_ref())?;
        headers.insert("Cookie", cookie);
        self.cookies = headers;
        self.update_params()?;
        Ok(())
    }
    pub fn scope<S: AsRef<str>>(&mut self, target: S) -> Result<(), HunterError> {
        if !self.is_in_scope(target.as_ref()) {
            Url::parse(target.as_ref())?;
            self.scoped.insert(String::from(target.as_ref()));
        }
        Ok(())
    }
    pub fn is_in_scope<S: AsRef<str>>(&self, target: S) -> bool {
        for scope in self.scoped.iter() {
            if String::from(target.as_ref()).starts_with(scope) {
                return true;
            }
        }
        false
    }
    pub fn set_target<S: AsRef<str>>(&mut self, target: S) -> Result<(), HunterError> {
        if !self.is_in_scope(target.as_ref()) {
            return Err(HunterError::out_of_scope(target.as_ref()));
        }
        self.current_target = Some(String::from(target.as_ref()));
        Ok(())
    }
    pub async fn set_public_request_bin(&mut self) -> Result<(), HunterError> {
        let bin = PublicRequestBin::new().await?;
        self.requestbin = Some(RequestBin::Public(bin));
        Ok(())
    }
    pub fn set_private_request_bin(&mut self, ip: impl AsRef<str>, port: u16) -> Result<(), HunterError> {
        let bin = PrivateRequestBin::new(ip, port)?;
        self.requestbin = Some(RequestBin::Private(bin));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn test_set_user_agent() -> Result<(), HunterError> {
        let mut client = HunterClient::new()?;
        client.set_user_agent("user-agent")?;
        assert_eq!(client.user_agent, "user-agent");
        Ok(())
    }

    #[test]
    fn test_set_cookies() -> Result<(), HunterError> {
        let mut client = HunterClient::new()?;
        client.set_cookies("session=connect")?;
        let mut expected_cookies = HeaderMap::new();
        expected_cookies.insert("Cookie",
            HeaderValue::from_str("session=connect")?);
        assert_eq!(client.cookies, expected_cookies);
        Ok(())
    }

    #[test]
    fn test_scope() -> Result<(), HunterError> {
        let mut client = HunterClient::new()?;
        client.scope("http://127.0.0.1/")?;
        assert!(client.scope("garbage").is_err());
        let mut expected_scope = HashSet::new();
        expected_scope.insert("http://127.0.0.1/".to_string());
        assert_eq!(client.scoped, expected_scope);
        Ok(())
    }

    #[test]
    fn test_is_in_scope() -> Result<(), HunterError> {
        let mut client = HunterClient::new()?;
        let _ = client.scope("http://127.0.0.1/");
        assert_eq!(client.is_in_scope("http://127.0.0.1/index.html"), true);
        assert_eq!(client.is_in_scope("http://google.com/index.html"), false);
        Ok(())
    }

    #[test]
    fn test_set_target() -> Result<(), HunterError> {
        let mut client = HunterClient::new()?;
        assert!(client.set_target("http://127.0.0.1/").is_err());
        assert_eq!(client.current_target, None);
        let _ = client.scope("http://127.0.0.1/");
        client.set_target("http://127.0.0.1/")?;
        assert_eq!(client.current_target, Some("http://127.0.0.1/".to_string()));
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_set_public_request_bin() -> Result<(), HunterError>{
        let mut client = HunterClient::new()?;
        assert!(client.requestbin.is_none());
        client.set_public_request_bin().await?;
        assert!(client.requestbin.is_some());
        Ok(())
    }

    #[test]
    fn test_set_private_request_bin() -> Result<(), HunterError>{
        let mut client = HunterClient::new()?;
        assert!(client.requestbin.is_none());
        client.set_private_request_bin("127.0.0.1", 8083)?;
        assert!(client.requestbin.is_some());
        Ok(())
    }

}
