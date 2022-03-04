use crate::gfycat_client::GfycatPost;
use std::fs::File;
use std::io::prelude::*;

// DownloadClient manages downloading WEBMs from Gfycat.
pub struct DownloadClient {
    http_client: reqwest::Client,
}

impl DownloadClient {
    // Creates a new DownloadClient
    pub fn new() -> Self {
        let http_client = reqwest::Client::new();

        Self { http_client }
    }

    // Downloads all posts in the WEBM format to the base local directory
    pub async fn download_webms(&self, posts: &Vec<GfycatPost>) -> Result<(), reqwest::Error> {
        println!("Downloading {} posts", posts.len());

        let mut successful_downloads: u16 = 0;
        let mut failures: Vec<String> = vec![];
        for post in posts {
            println!("Downloading post {} with ID {}", post.title, post.gfy_id);

            if let Ok(bytes) = self.fetch_webm(&post.webm_url).await {
                if let Ok(mut file) = File::create(format!("{}.webm", &post.title)) {
                    if let Err(e) = file.write_all(&bytes) {
                        failures.push(format!(
                            "Failed to write to file {}.webm:\n{}",
                            &post.title, e
                        ));
                    } else {
                        successful_downloads += 1;
                    }
                } else {
                    failures.push(format!("Failed to create file {}.webm", &post.title));
                }
            } else {
                failures.push(format!(
                    "Failed to download file {} from URL {}",
                    &post.title, post.webm_url
                ));
            }
        }

        println!("Successfully downloaded {} posts.", successful_downloads);

        if failures.len() > 0 {
            eprintln!("Some downloads failed:");
            for message in failures {
                eprintln!("{}", message);
            }
        }

        Ok(())
    }

    async fn fetch_webm(&self, url: &String) -> Result<bytes::Bytes, reqwest::Error> {
        self.http_client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await
    }
}
