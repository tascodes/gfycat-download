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
        for post in posts {
            println!("Downloading post {} with ID {}", post.title, post.gfy_id);
            let bytes = self.fetch_webm(&post.webm_url).await?;

            // let filepath = &path.join(format!("{}.webm", post.title).as_str());
            let mut file = File::create(format!("{}.webm", &post.title))
                .expect(format!("Failed to create file output/gifs/{}.webm", &post.title).as_str());
            file.write_all(&bytes)
                .expect("Failed to write contents to file.");
        }

        println!("Downloaded and saved all posts.");

        Ok(())
    }

    async fn fetch_webm(&self, url: &String) -> Result<bytes::Bytes, reqwest::Error> {
        let bytes = self
            .http_client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(bytes)
    }
}
