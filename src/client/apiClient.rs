use reqwest::{Client, header::HeaderMap, StatusCode};
use serde::de::DeserializeOwned;
use crate::model::apiRequest::ApiRequest;


pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        
        Ok(Self {
            client,
            base_url: base_url.to_string(),
        })
    }

    pub async fn send_request<T: DeserializeOwned>(
        &self,
        config: ApiRequest,
    ) -> Result<T, crate::client::error::Error> {
        let url = format!("{}{}", self.base_url, config.path.trim_start_matches('/'));
        let method = reqwest::Method::from_bytes(config.method.as_bytes()).map_err(|e| {
            crate::client::error::Error::HttpError {
                status: StatusCode::BAD_REQUEST,
                message: format!("Invalid HTTP method: {}", e),
            }
        })?;
        let mut request = self.client.request(method, &url);
        
        let mut headers = HeaderMap::new();
        for (key, value) in config.headers.iter() {
            let header_name = key.parse::<reqwest::header::HeaderName>().map_err(|_| {
                crate::client::error::Error::HttpError {
                    status: StatusCode::BAD_REQUEST,
                    message: format!("Invalid header name: {}", key),
                }
            })?;
            let header_value = value.parse::<reqwest::header::HeaderValue>().map_err(|_| {
                crate::client::error::Error::HttpError {
                    status: StatusCode::BAD_REQUEST,
                    message: format!("Invalid header value for {}: {}", key, value),
                }
            })?;
            headers.insert(header_name, header_value);
        }
        request = request.headers(headers);
        
        if let Some(body) = config.body {
            request = request.json(&body);
        }
        
        let response = request.send().await?;
        let status = response.status();
        
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(crate::client::error::Error::HttpError { status, message })
        }
    }
}