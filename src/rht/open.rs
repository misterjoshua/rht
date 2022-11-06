use crate::rht::error::RhtError;
use log::{error, info};
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

impl OpenFileRequest {
    fn from(input: &str) -> Result<Self, RhtError> {
        let local_path = Path::new(&input);
        let bytes = std::fs::read(local_path)
            .map_err(|_| RhtError::Error(format!("Unable to read {}", input)))?;

        let content = base64::encode(bytes);

        Ok(OpenFileRequest {
            name: String::from(input),
            content,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenResponse;

impl OpenRequest {
    pub fn from_user_input(input: String) -> Result<OpenRequest, RhtError> {
        let is_url = Url::parse(&input).map(|_| true).unwrap_or(false);
        let local_path = Path::new(&input);

        if is_url {
            Ok(OpenRequest::Url(OpenUrlRequest { url: input }))
        } else if local_path.is_file() {
            let open_file_request = OpenFileRequest::from(&input)?;
            Ok(OpenRequest::File(open_file_request))
        } else {
            Err(RhtError::Error("File not found".to_string()))
        }
    }
}

pub async fn open(req: &OpenRequest) -> Result<OpenResponse, RhtError> {
    let open_response = match req {
        OpenRequest::Url(x) => open_url(x).await,
        OpenRequest::File(x) => open_file(x).await,
    };

    if let Err(err) = &open_response {
        error!("Unable to open: {}", err);
    }

    open_response
}

async fn open_url(req: &OpenUrlRequest) -> Result<OpenResponse, RhtError> {
    let url =
        Url::parse(&req.url).map_err(|err| RhtError::Error(format!("Invalid url: {}", err)))?;

    let scheme = url.scheme();
    if scheme.eq("ext+granted-containers") {
        open::with(url.as_str(), "firefox")
            .map_err(|err| RhtError::Error(format!("Cannot open with firefox: {}", err)))?;

        Ok(OpenResponse)
    } else if scheme.eq("http") || scheme.eq("https") {
        open::that(&req.url)
            .map_err(|err| RhtError::Error(format!("Cannot open http url: {}", err)))?;

        Ok(OpenResponse)
    } else {
        Err(RhtError::Error("Unsupported scheme".to_string()))
    }
}

async fn open_file(req: &OpenFileRequest) -> Result<OpenResponse, RhtError> {
    info!("Opening file: {}", req.name);

    let dir = std::env::temp_dir().join(format!(".tmp.{}", rand::thread_rng().next_u32()));
    std::fs::create_dir_all(&dir)
        .map_err(|err| RhtError::Error(format!("Cannot create temp directory: {}", err)))?;

    let file = dir.join(&req.name);
    let path = file.as_path();

    let contents = base64::decode(&req.content)
        .map_err(|err| RhtError::Error(format!("Cannot decode base64: {}", err)))?;

    std::fs::write(&file, contents)
        .map_err(|err| RhtError::Error(format!("Cannot write to temp file: {}", err)))?;

    info!("Local file path: {:?}", path);
    open::that(path)
        .map_err(|err| RhtError::Error(format!("Cannot open the given file: {}", err)))?;

    Ok(OpenResponse)
}
