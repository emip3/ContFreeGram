use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use dotenv::dotenv;
use std::env;
use serde_json::json;

pub async fn groqRequest(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();

    let apiKey = env::var("GROG_CLOUD_API_DEV_TOKEN")?;

    // Set up the headers
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", apiKey))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Create the client
    let client = reqwest::Client::new();

    // Define the JSON body with a valid `messages` structure
    let body = json!({
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "model": "mixtral-8x7b-32768",
        "temperature": 0.7,
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

    if !resp.status().is_success() {
        let error_body = resp.text().await?;
        return Err(format!("API request failed: {}", error_body).into());
    }

    // Parse the response
    let respJson: serde_json::Value = resp.json().await?;

    println!("Debug: Full API Response: {}", serde_json::to_string_pretty(&respJson)?);

    let content = respJson["choices"]
        .get(0)
        .and_then(|choice| choice["message"]["content"].as_str())
        .ok_or_else(|| {
            let error_msg = format!("Failed to extract content. Response structure: {:?}", respJson);
            println!("Debug: {}", error_msg);
            error_msg
        })?
        .to_string();

    Ok(content)
}


