use std::io::Read;

use reqwest::header::HeaderMap;

use crate::error::{BotError, BotResult};

pub async fn http_get(url: &str) -> BotResult<String> {
    let data = reqwest::get(url).await?.text().await;
    match data {
        Ok(data) => Ok(data),
        Err(err) => Err(BotError::from(err)),
    }
}

pub async fn http_get_image(url: &str) -> BotResult<Vec<u8>> {
    let bytes = match reqwest::get(url).await {
        Ok(res) => {
            res.error_for_status()
        }
        Err(err) => {return Err(BotError::from(err))}
    };
    match bytes {
        Ok(bytes) => {
            match tokio::time::timeout(std::time::Duration::from_secs(60), bytes.bytes()).await {
                Ok(bytes) => {
                    let bytes = bytes?.to_vec();
                    Ok(bytes)
                },
                Err(_) => {
                    Err(BotError::from("获取图片超时喵..."))
                }
            }
        }
        Err(err) => Err(BotError::from(err)),
    }
}

pub async fn http_post_json(url: &str, json: &serde_json::Value) -> BotResult<String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let data = client
        .post(url)
        .headers(headers)
        .json(json)
        .send()
        .await?
        .text()
        .await;
    match data {
        Ok(data) => Ok(data),
        Err(err) => Err(BotError::from(err)),
    }
}
