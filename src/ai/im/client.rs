use crate::ai::im::traits::IM;
use crate::errors::ImageGenError;
use crate::types::im::ImageGenResponse;
use reqwest::Client as HttpClient;
use std::{env::var, io::Write};

pub struct Client {
    pub host: String,
    pub version: String,
    pub api_key: String,
}

impl IM for Client {
    fn new() -> Result<Self, ImageGenError> {
        let host = var("REPLICATE_HOST").map_err(|e| ImageGenError::Other(e.to_string()))?;
        let version =
            var("IMAGE_MODEL_VERSION").map_err(|e| ImageGenError::Other(e.to_string()))?;
        let api_key = var("REPLICATE_API_KEY").map_err(|e| ImageGenError::Other(e.to_string()))?;
        Ok(Client {
            host,
            version,
            api_key,
        })
    }

    async fn generate(&self, text: String) -> Result<(), ImageGenError> {
        let query = text.trim();
        let url = format!("{}", self.host);
        let data = format!("{{\"version\": \"{}\",\"input\": {{\"width\": 768,\"height\": 768,\"prompt\": \"{}\",\"scheduler\": \"K_EULER\",\"num_outputs\": 1,\"guidance_scale\": 7.5,\"num_inference_steps\": 50}}}}", self.version, query);

        let client = HttpClient::new();
        let res = client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(data)
            .send()
            .await?;

        let body = res.text().await?;
        let image_gen_response: ImageGenResponse = serde_json::from_str(&body)?;
        let url = &image_gen_response.urls.get;
        let mut count = 0;

        loop {
            let res = client
                .get(url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await?;

            let body = res.text().await?;
            let image_gen_response: ImageGenResponse = serde_json::from_str(&body)?;

            if image_gen_response.status == "succeeded" {
                let image_url = image_gen_response.output.unwrap()[0].clone();
                download_image(&image_url).await?;
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
            println!("Generating image...");
        }

        Ok(())
    }
}

async fn download_image(url: &str) -> Result<(), ImageGenError> {
    let client = HttpClient::new();
    let res = client.get(url).send().await?;
    let body = res.bytes().await?;
    let mut file = std::fs::File::create("output.png")?;
    file.write_all(&body)?;
    Ok(())
}
