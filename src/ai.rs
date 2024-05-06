use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{error::Error, io::Write};

pub struct LLMClient {
    pub host: String,
    pub model: String,
    pub api_key: String,
}

pub struct ImageGenClient {
    pub host: String,
    pub version: String,
    pub api_key: String,
}

impl LLMClient {
    pub fn new(host: String, model: String, api_key: String) -> Result<Self, &'static str> {
        Ok(LLMClient {
            host,
            model,
            api_key,
        })
    }

    pub async fn run(&self, query: &str) -> Result<String, Box<dyn Error>> {
        let query = query.trim();
        let url = format!("{}/chat/completions", self.host);
        let data = format!("{{\"model\": \"{}\",\"messages\": [{{\"role\": \"system\",\"content\": \"You are a helpful assistant.\"}},{{\"role\": \"user\",\"content\": \"{}\"}}]}}", self.model, query);

        let client = Client::new();
        let res = client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(data)
            .send()
            .await?;

        let body = res.text().await?;
        let completion: Completion = serde_json::from_str(&body).unwrap();
        let message = &completion.choices[0].message.content;

        return Ok(message.clone());
    }
}

impl ImageGenClient {
    pub fn new(host: String, version: String, api_key: String) -> Result<Self, &'static str> {
        Ok(ImageGenClient {
            host,
            version,
            api_key,
        })
    }

    pub async fn run(&self, query: &str) -> Result<(), Box<dyn Error>> {
        let query = query.trim();
        let url = format!("{}", self.host);

        let data = format!("{{\"version\": \"{}\",\"input\": {{\"width\": 768,\"height\": 768,\"prompt\": \"{}\",\"scheduler\": \"K_EULER\",\"num_outputs\": 1,\"guidance_scale\": 7.5,\"num_inference_steps\": 50}}}}", self.version, query);

        let client = Client::new();
        let res = client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(data)
            .send()
            .await?;

        let body = res.text().await?;
        let image_gen_response: ImageGenResponse = serde_json::from_str(&body).unwrap();
        let url = &image_gen_response.urls.get;

        let mut count = 0;
        loop {
            let res = client
                .get(url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await?;
            let body = res.text().await?;
            let image_gen_response: ImageGenResponse = serde_json::from_str(&body).unwrap();
            if image_gen_response.status == "succeeded" {
                let image_url = image_gen_response.output.unwrap()[0].clone();
                download_image(&image_url).await?;
                // run a command open output.png
                std::process::Command::new("open")
                    .arg("output.png")
                    .output()
                    .expect("failed to execute process");
                break;
            }
            if count == 5 {
                break;
            }
            count += 1;
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        return Ok(());
    }
}

pub async fn download_image(url: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let res = client.get(url).send().await?;
    let body = res.bytes().await?;
    let mut file = std::fs::File::create("output.png")?;
    file.write_all(&body)?;
    Ok(())
}

// llm completion response
#[derive(Serialize, Deserialize)]
struct Completion {
    id: String,
    object: String,
    created: i64,
    model: String,
    system_fingerprint: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    index: i32,
    message: Message,
    logprobs: Option<Logprobs>,
    finish_reason: String,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Logprobs {
    token_logprobs: Vec<f32>,
    top_logprobs: Vec<TopLogprobs>,
}

#[derive(Serialize, Deserialize)]
struct TopLogprobs {
    token: String,
    logprob: f32,
}

#[derive(Serialize, Deserialize)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

// image generation response
#[derive(Serialize, Deserialize)]
struct ImageGenResponse {
    id: String,
    model: String,
    version: String,
    input: Input,
    logs: String,
    output: Option<Vec<String>>,
    error: Option<String>,
    status: String,
    created_at: String,
    started_at: Option<String>,
    completed_at: Option<String>,
    urls: Urls,
}

#[derive(Serialize, Deserialize)]
struct Input {
    guidance_scale: f32,
    height: i32,
    num_inference_steps: i32,
    num_outputs: i32,
    prompt: String,
    scheduler: String,
    width: i32,
}

#[derive(Serialize, Deserialize)]
struct Urls {
    cancel: String,
    get: String,
}
