use std::io::{self, Write};
use reqwest::Error;
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Deserialize)]
struct Config {
    OPENAI_KEY: String,
}

#[derive(Deserialize)]
struct Response {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
    finish_reason: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // config.toml 파일에서 API 키 읽기
    let config: Config = toml::from_str(&std::fs::read_to_string("config.toml").expect("Failed to read config.toml")).unwrap();
    let api_key = config.OPENAI_KEY;

    let client = reqwest::Client::new();

    loop {
        let mut input = String::new();
        print!("질문을 입력해주세요: ");
        io::stdout().flush().unwrap(); // 텍스트를 즉시 출력
        io::stdin().read_line(&mut input).unwrap();

        let res = client.post("https://api.openai.com/v1/completions")
            .bearer_auth(api_key.clone())
            .json(&json!({
                "model": "gpt-3.5-turbo-instruct",
                "prompt": input.trim(),
                "max_tokens": 60,
            }))
            .send()
            .await?;

        let response: Response = res.json().await?;
        for choice in response.choices {
            println!("답변: {}", choice.text.trim());
        }
    }
}