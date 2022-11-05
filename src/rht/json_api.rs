use anyhow::anyhow;
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;
use url::Url;

pub struct JsonApi<T: Serialize, U: DeserializeOwned> {
    api_url: Url,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
}

impl<T: Serialize, U: DeserializeOwned> JsonApi<T, U> {
    pub fn new(api_url: Url) -> Self {
        JsonApi {
            api_url,
            _t: PhantomData,
            _u: PhantomData,
        }
    }

    pub async fn post(&self, req: T) -> Result<U, anyhow::Error> {
        use reqwest::Client;

        let res = Client::new()
            .post(self.api_url.as_str())
            .json(&req)
            .send()
            .await?;

        let res = match res.status() {
            reqwest::StatusCode::OK => res,
            _ => return Err(anyhow!("Bad HTTP code: {}", res.status())),
        };

        match res.json::<U>().await {
            Ok(u) => Ok(u),
            Err(err) => Err(anyhow!("Failed to decode the response: {}", err)),
        }
    }
}
