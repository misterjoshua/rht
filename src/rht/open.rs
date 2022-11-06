use log::info;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, path::Path};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub enum OpenRequest {
    Url(OpenUrlRequest),
    File(OpenFileRequest),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenUrlRequest {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenFileRequest {
    name: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OpenError {
    Error,
    UnsupportedScheme,
}

impl OpenRequest {
    pub fn from_user_input(input: String) -> Result<OpenRequest, anyhow::Error> {
        let local_path = Path::new(&input);

        let res = if local_path.is_file() {
            info!("Reading local file: {}", input);
            let bytes = std::fs::read(local_path)?;
            let content = base64::encode(bytes);

            OpenRequest::File(OpenFileRequest {
                name: input,
                content,
            })
        } else {
            OpenRequest::Url(OpenUrlRequest { url: input })
        };

        Ok(res)
    }
}

pub async fn open(req: &OpenRequest) -> Result<OpenResponse, OpenError> {
    match req {
        OpenRequest::Url(x) => open_url(x).await,
        OpenRequest::File(x) => open_file(x).await,
    }
}

async fn open_url(req: &OpenUrlRequest) -> Result<OpenResponse, OpenError> {
    let url = Url::parse(&req.url)?;
    info!("Opening url: {}", url.scheme());

    let scheme = url.scheme();
    if scheme.eq("ext+granted-containers") {
        open::with(url.as_str(), "firefox")?;
        Ok(OpenResponse)
    } else if scheme.eq("http") || scheme.eq("https") {
        open::that(&req.url)?;
        Ok(OpenResponse)
    } else {
        Err(OpenError::UnsupportedScheme)
    }
}

async fn open_file(req: &OpenFileRequest) -> Result<OpenResponse, OpenError> {
    info!("Opening file: {}", req.name);

    let dir = std::env::temp_dir().join(format!(".tmp.{}", rand::thread_rng().next_u32()));
    std::fs::create_dir_all(&dir)?;
    let file = dir.join(&req.name);
    let path = file.as_path();

    std::fs::write(&file, base64::decode(&req.content)?)?;

    info!("Local file path: {:?}", path);
    open::that(path)?;

    Ok(OpenResponse)
}

impl<T: std::error::Error> From<T> for OpenError {
    fn from(_: T) -> Self {
        OpenError::Error
    }
}
