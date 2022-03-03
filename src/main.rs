use gfycat_transfer::{
    configuration::get_configuration, download_client::DownloadClient, gfycat_client::GfycatClient,
};
use secrecy::ExposeSecret;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let config = get_configuration().expect("Failed to get configuration.");
    let gfycat_client = GfycatClient::new(
        String::from("https://api.gfycat.com/v1"),
        config.client_id,
        config.client_secret,
        config.username,
        config.password,
    );
    let download_client = DownloadClient::new();
    let access_token = gfycat_client.get_access_token().await?.access_token;
    let posts = gfycat_client
        .get_gfycats(access_token.expose_secret().to_owned())
        .await?;

    download_client.download_webms(&posts).await?;

    let posts_json =
        serde_json::to_string_pretty(&posts).expect("Failed to serialize posts into json");
    let mut file =
        File::create("post_metadata.json").expect("Failed to create post_metadata.json file.");
    file.write_all(posts_json.as_bytes())
        .expect("Failed to write posts json to file.");

    Ok(())
}
