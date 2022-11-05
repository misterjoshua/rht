use std::fmt::Debug;

use log::info;
use serde::{Deserialize, Serialize};

use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenRequest {
    url: String,
}

impl OpenRequest {
    pub fn new(url: &str) -> Self {
        OpenRequest {
            url: String::from(url),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OpenError {
    Error,
}

pub async fn open(req: &OpenRequest) -> Result<OpenResponse, OpenError> {
    let url = &req.url;
    info!("Opening url: {}", url);

    let url = Url::parse(url)?;

    if url.scheme().eq("ext+granted-containers") {
        std::process::Command::new("C:\\Program Files\\Mozilla Firefox\\firefox.exe")
            .args([url.as_str()])
            .output()?;
    } else if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", "start", url.as_str()])
            .output()?;
    } else {
        std::process::Command::new("xdg-open")
            .args([url.as_str()])
            .output()?;
    }

    Ok(OpenResponse)
}

impl<T: std::error::Error> From<T> for OpenError {
    fn from(_: T) -> Self {
        OpenError::Error
    }
}
