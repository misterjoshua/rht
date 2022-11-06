use log::info;
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
        std::process::Command::new("C:\\Program Files\\Mozilla Firefox\\firefox.exe")
            .args([url.as_str()])
            .output()?;

        return Ok(OpenResponse);
    }

    if scheme.eq("http") || scheme.eq("https") {
        return platform_start(&req.url).await;
    }

    Err(OpenError::UnsupportedScheme)
}

async fn open_file(file: &OpenFileRequest) -> Result<OpenResponse, OpenError> {
    info!("Opening file: {}", file.name);

    let temp_dir = tempfile::tempdir()?;
    let temp_file = temp_dir.path().join(&file.name);

    std::fs::write(&temp_file, base64::decode(&file.content)?)?;

    platform_start(&String::from(temp_file.clone().to_string_lossy())).await
}

async fn platform_start(local: &String) -> Result<OpenResponse, OpenError> {
    info!("Using platform start action: {}", local);

    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", "start", local])
            .output()?;
    } else {
        std::process::Command::new("xdg-open")
            .args([local])
            .output()?;
    }

    Ok(OpenResponse)
}

impl<T: std::error::Error> From<T> for OpenError {
    fn from(_: T) -> Self {
        OpenError::Error
    }
}
