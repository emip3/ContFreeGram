
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use dotenv::dotenv;
use std::env;
use serde_json::json;


pub async fn groq_request() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("API_KEY")?;

    // Set up the headers
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Create the client
    let client = reqwest::Client::new();

    // Define the JSON body with a valid `messages` structure
    let body = json!({
        "messages": [
            {
                "role": "user",
                "content": "Hello, can you provide some information?"
            }
        ],
        "model": "llama3-8b-8192",
        "temperature": 1,
        "max_tokens": 1024,
        "top_p": 1,
        "stream": false,
        "stop": null
    });

    // Make the POST request
    let resp = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .headers(headers)
        .json(&body)
        .send()
        .await?;
    // Print the response for debugging
    let resp_text = resp.text().await?;
    println!("Response: {}", resp_text);

    let start = resp_text.find("content")
        .ok_or_else(|| "Content not found")?;
    let content_start = start + 10;

    let end = resp_text[content_start..].find("logprobs")
        .ok_or_else(|| "End of content not found")?;

    let extracted = &resp_text[content_start..content_start + end - 4];
    println!("Extracted content: {}", extracted);
    Ok(())
}

