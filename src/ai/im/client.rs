use std::io::Write;

use crate::ai::im::traits::IM;
use crate::errors::ImageGenError;
use crate::types::im::ImageResponse;
use reqwest::Client as HttpClient;
use dirs::desktop_dir;

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
        let path = desktop_dir().unwrap();
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

        let filename = format!("{}.png", query);
        download_image(&image_url, path.to_str().unwrap(), &filename).await?;

        println!("Image downloaded successfully, saved to {}", path.to_str().unwrap());
        open_image(format!("{}/{}", path.to_str().unwrap(), filename).as_str())?;

        Ok(())
    }
}

async fn download_image(url: &str, path: &str, filename: &str) -> Result<(), ImageGenError> {
    let client = HttpClient::new();
    let res = client.get(url).send().await?;
    let body = res.bytes().await?;
    let mut file = std::fs::File::create(format!("{}/{}", path, filename))?;
    file.write_all(&body)?;
    Ok(())
}

fn open_image(path: &str) -> Result<(), ImageGenError> {
    let os = std::env::consts::OS;
    
    match os {
        "macos" => {
            let _ = std::process::Command::new("open")
                .arg(path)
                .output()
                .expect("failed to open image");
        }
        "windows" => {
            let _ = std::process::Command::new("explorer")
                .arg(path)
                .output()
                .expect("failed to open image");
        }
        _ => {
            let _ = std::process::Command::new("xdg-open")
                .arg(path)
                .output()
                .expect("failed to open image");
        }
    }

    Ok(())
}