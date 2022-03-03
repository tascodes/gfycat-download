use config::Config;
use dotenv;
use rpassword::read_password;
use secrecy::Secret;
use std::io::{self};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Settings {
    pub client_id: Secret<String>,
    pub client_secret: Secret<String>,
    pub username: String,
    pub password: Secret<String>,
}

pub fn get_configuration() -> Result<Settings, std::io::Error> {
    let _ = dotenv::dotenv();

    let settings = Config::builder()
        .add_source(config::Environment::with_prefix("GFYCAT"))
        .build()
        .unwrap();

    let r = match settings.try_deserialize::<Settings>() {
        Err(_) => prompt_for_settings(),
        Ok(f) => Ok(f),
    };

    r
}

pub fn prompt_for_settings() -> Result<Settings, std::io::Error> {
    println!(
"Failed to determine configuration from environment variables.

If you want to set these values without being prompted every time, set the following environment variables:
- GFYCAT_CLIENT_ID
- GFYCAT_CLIENT_SECRET
- GFYCAT_USERNAME
- GFYCAT_PASSWORD
");
    println!("Enter Gfycat API Client ID:");
    let mut client_id = String::new();
    io::stdin().read_line(&mut client_id)?;
    client_id = client_id.trim().into();

    println!("Enter Gfycat API Client Secret:");
    let client_secret = String::from(read_password()?.trim());

    println!("Enter Gfycat username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    username = username.trim().into();

    println!("Enter Gfycat password:");
    let password = String::from(read_password()?.trim());

    Ok(Settings {
        client_id: Secret::new(client_id),
        client_secret: Secret::new(client_secret),
        username,
        password: Secret::new(password),
    })
}
