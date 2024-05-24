use std::io::Write;

use crate::ai::im::traits::IM;
use crate::errors::ImageGenError;
use crate::types::im::ImageResponse;
use reqwest::Client as HttpClient;

pub struct Client {
    pub host: String,
    pub model: String,
    pub api_key: String,
}

impl IM for Client {
    fn new(host: String, model: String, api_key: String) -> Result<Self, ImageGenError> {
        Ok(Client {
            host,
            model,
            api_key,
        })
    }

    async fn generate(&self, text: String) -> Result<(), ImageGenError> {
        let query = text.trim();
        let url = format!("{}", self.host);
        let data = format!(
            "{{\"model\": \"{}\",\"prompt\": \"{}\",\"n\": 1,\"size\": \"1024x1024\"}}",
            self.model, query
        );
        let client = HttpClient::new();
        let res = client
            .post(&format!("{}", format!("{}/images/generations", url)))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(data)
            .send()
            .await?;

        println!("Generating image...please wait");
        let body = res.text().await?;
        let response: ImageResponse = serde_json::from_str(&body)?;

        if response.data.len() == 0 {
            println!("No image found");
            return Err(ImageGenError::Other("No image found".to_string()));
        }

        let image_url = response.data[0].url.clone();
        println!("Image url: {}", image_url);

        println!("Downloading image...");
        download_image(&image_url).await?;

        println!("Image downloaded successfully, opening image...");

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
