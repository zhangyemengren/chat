use std::io;
use std::io::Write;
use std::pin::Pin;
use bytes::Bytes;
use crate::config::Config;
use serde_json::json;
use futures_util::{StreamExt, stream, Stream};


pub async fn fetch_chat(
    cfg: &Config,
) -> Result<
    Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + 'static>>,
    Box<dyn std::error::Error>,
> {
    let client = reqwest::Client::new();

    let api_key = &cfg.api_key;
    let url = "https://openrouter.ai/api/v1/chat/completions";

    let body = json!({
        "model": "arcee-ai/trinity-large-preview:free",
        "messages": [
            {
                "role": "user",
                "content": "How many r's are in the word 'strawberry'?"
            }
        ],
        "reasoning": {"enabled": true},
        "stream": true
    });

    let mut res = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?.bytes_stream();
    let mut buf: Vec<u8> = Vec::new();
    let out_stream = stream::unfold((res, buf, false), |(mut res, mut buf, done)| async move {
        if done {
            return None;
        }
        match res.next().await {
            Some(Ok(chunk)) => {
                // 打印
                buf.extend_from_slice(&chunk);
                while let Some(pos) = buf.iter().position(|&b| b == b'\n') {
                    let line = buf.drain(..=pos).collect::<Vec<_>>();
                    let line = String::from_utf8_lossy(&line);
                    print!("{}", line);
                    let _ = io::stdout().flush();
                }
                Some((Ok(chunk), (res, buf, false)))
            }
            Some(Err(err)) => {
                let io_err = io::Error::new(io::ErrorKind::Other, err);
                Some((Err(io_err), (res, buf, true)))
            }
            None => None,
        }
    });
    Ok(Box::pin(out_stream))
}
