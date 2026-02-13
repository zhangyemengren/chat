use std::{collections::VecDeque, convert::Infallible};
use crate::config::Config;
use serde_json::json;
use axum::response::sse::Event;
use futures_util::{StreamExt, stream::{self, BoxStream} };


pub type SseStream = BoxStream<'static, Result<Event, Infallible>>;


#[derive(serde::Deserialize)]
pub struct ChatRequest {
    pub content: String,
}

pub async fn fetch_chat_sse(
    cfg: &Config,
    req: ChatRequest,
) -> Result<
    SseStream,
    Box<dyn std::error::Error>,
> {
    let client = reqwest::Client::new();
    let content = req.content;
    let api_key = &cfg.api_key;
    let url = "https://openrouter.ai/api/v1/chat/completions";

    let body = json!({
        "model": "arcee-ai/trinity-large-preview:free",
        "messages": [
            {
                "role": "user",
                "content": content
            }
        ],
        "reasoning": {"enabled": true},
        "stream": true
    });

    let res = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;
    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        return Err(format!("OpenRouter error: {} {}", status, text).into());
    }

    let res = res.bytes_stream();
    let buf: Vec<u8> = Vec::new();
    let queue: VecDeque<Event> = VecDeque::new();
    let out_stream = stream::unfold((res, buf, queue), |(mut res, mut buf, mut queue)| async move {
        loop {
            if let Some(event) = queue.pop_front() {
                return Some((Ok(event), (res, buf, queue)));
            }
            match res.next().await {
                Some(Ok(chunk)) => {
                    buf.extend_from_slice(&chunk);
                    while let Some(pos) = buf.iter().position(|&b| b == b'\n') {
                        // 切片 抽走到pos的部分
                        let line = buf.drain(..=pos).collect::<Vec<_>>();
                        let line = String::from_utf8_lossy(&line);
                        // 处理空格包括crlf
                        let line = line.trim_end_matches(&['\r', '\n']);
                        if line.is_empty() {
                            continue;
                        }
                        let data = match line.strip_prefix("data:") {
                            Some(rest) => rest.trim_start(),
                            None => continue,
                        };
                        if data.is_empty() {
                            continue;
                        }
                        println!("{}", data);
                        queue.push_back(Event::default().data(data.to_string()));
                    }
                }
                Some(Err(err)) => {
                    let event = Event::default().event("error").data(err.to_string());
                    return Some((Ok(event), (res, buf, queue)));
                }
                None => return None,
            }
        }
    });
    Ok(Box::pin(out_stream))
}
