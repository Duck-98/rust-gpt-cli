# Rust ChatGPT CLI

이 CLI는 Rust로 작성된 ChatGPT를 사용하는 간단한 채팅 프로그램입니다.

## 사용법

1. **Rust 설치**: 만약 시스템에 Rust가 설치되어 있지 않다면, [Rust Document 한글 번역 웹사이트](https://rinthel.github.io/rust-lang-book-ko/ch01-01-installation.html)에서 설치방법을 보고 Rust를 설치해주세요.

2. **프로젝트 클론**: 이 저장소를 클론합니다.

   ```bash
   git clone https://github.com/Duck-98/rust-gpt-cli.git
   ```

3. **의존성 설치**: 프로젝트 폴더에서 다음 명령어를 실행하여 필요한 라이브러리를 설치합니다.

   ```bash
   cargo build
   ```

4. **ChatGPT 토큰 설정**: OpenAI에서 발급받은 ChatGPT API 토큰을 설정합니다. `gpt_cli`폴더 경로에 `config.toml` 파일을 생성하고 다음과 같이 설정합니다.

   ```
   openai_key="your-api-key"
   ```

5. **실행**: 프로젝트 폴더에서 CLI를 실행합니다.

   ```bash
   cargo run
   ```

6. **채팅**: CLI가 시작되면 ChatGPT와의 대화를 시작할 수 있습니다. 사용자 입력에 따라 ChatGPT가 응답합니다.

## 참고

cli를 사용할 때 길게 답변하길 원한다면 `main.rs`내에 있는

```rust
 let res = client.post("https://api.openai.com/v1/completions")
            .bearer_auth(api_key.clone())
            .json(&json!({
                "model": "gpt-3.5-turbo-instruct",
                "prompt": input.trim(),
                "max_tokens": 60, // 100(글자)
            }))
            .send()
            .await?;
```

> "max_tokens": 60을 원하는 글자수로 바꿔주시면 됩니다.

## 사용 영상


https://github.com/Duck-98/rust-gpt-cli/assets/72850354/5181d7bc-f614-4a4b-8512-4d6a5b67b5d8

