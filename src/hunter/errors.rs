use reqwest;
use reqwest::header::InvalidHeaderValue;
use crate::requestbin::RequestBinError;
use http::method::InvalidMethod;
use url::ParseError;

#[derive(Debug, PartialEq)]
pub enum HunterErrType {
    OutOfScopeError,
    NoTargetError,
    NoRequestBinError,
    RequestBinError,
    ReqwestError,
    InvalidHeaderValueError,
    InvalidMethodError,
    UrlParsingError,
    SIGKILLError,
    Other
}

#[derive(Debug)]
pub struct HunterError {
    pub value: String,
    pub _errtype: HunterErrType
}
impl From<InvalidMethod> for HunterError {
    fn from(item: InvalidMethod) -> Self {
        Self { value: item.to_string(),
        _errtype: HunterErrType::InvalidMethodError}
    }
}
impl From<reqwest::Error> for HunterError {
    fn from(item: reqwest::Error) -> Self {
        Self { value: item.to_string(),
        _errtype: HunterErrType::ReqwestError}
    }
}
impl From<std::string::String> for HunterError {
    fn from(item: String) -> Self {
        Self { value: item,
        _errtype: HunterErrType::Other}
    }
}
impl From<RequestBinError> for HunterError {
    fn from(item: RequestBinError) -> Self {
        Self { value: item.value,
        _errtype: HunterErrType::RequestBinError}
    }
}
impl From<InvalidHeaderValue> for HunterError {
    fn from(item: InvalidHeaderValue) -> Self {
        Self { value: item.to_string(),
        _errtype: HunterErrType::InvalidHeaderValueError}
    }

}
impl From<ParseError> for HunterError {
    fn from(item: ParseError) -> Self {
        Self { value: item.to_string(),
        _errtype: HunterErrType::UrlParsingError}
    }

}

impl From<ctrlc::Error> for HunterError {
    fn from(item: ctrlc::Error) -> Self {
        Self { value: item.to_string(),
        _errtype: HunterErrType::SIGKILLError}
    }

}

impl HunterError {
    pub fn out_of_scope(url: impl AsRef<str>) -> HunterError {
        HunterError { value: format!("Url \"{}\" out of scope.", url.as_ref()),
            _errtype: HunterErrType::OutOfScopeError}
    }
    pub fn no_target() -> HunterError {
        HunterError { value: String::from("No target set."),
            _errtype: HunterErrType::NoTargetError}
    }
    pub fn no_rbin() -> HunterError {
        HunterError { value: String::from("No Request Bin set."),
            _errtype: HunterErrType::NoRequestBinError}
    }
}

