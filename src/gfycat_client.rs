use reqwest::Client;
use secrecy::{ExposeSecret, Secret};

// GfycatClient handles requests to the Gfycat API including auth and fetching posts.
pub struct GfycatClient {
    http_client: Client,
    base_url: String,
    client_id: Secret<String>,
    client_secret: Secret<String>,
    username: String,
    password: Secret<String>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
struct ClientCredentialsGrantRequest<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    username: &'a str,
    password: &'a str,
    grant_type: &'a str,
}

// A proper response from the Gfycat authorization endpoint.
#[derive(serde::Deserialize)]
pub struct ClientCredentialsGrantResponse {
    pub token_type: String,
    pub refresh_token_expires_in: u32,
    pub refresh_token: Secret<String>,
    pub scope: String,
    pub resource_owner: String,
    pub expires_in: u16,
    pub access_token: Secret<String>,
}

// A single post in the list of posts fetched from Gfycat.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GfycatPost {
    pub webm_url: String,
    pub title: String,
    pub gfy_id: String,
}

// Response from the /me/gfycats endpoint
#[derive(serde::Deserialize)]
pub struct AllGfycatsResponse {
    pub cursor: String,
    pub gfycats: Vec<GfycatPost>,
}

impl GfycatClient {
    pub fn new(
        base_url: String,
        client_id: Secret<String>,
        client_secret: Secret<String>,
        username: String,
        password: Secret<String>,
    ) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap();

        Self {
            http_client,
            base_url,
            client_id,
            client_secret,
            username,
            password,
        }
    }

    pub async fn get_access_token(&self) -> Result<ClientCredentialsGrantResponse, reqwest::Error> {
        let url = format!("{}/oauth/token", self.base_url);
        let request_body = ClientCredentialsGrantRequest {
            client_id: self.client_id.expose_secret().as_ref(),
            client_secret: self.client_secret.expose_secret().as_ref(),
            username: self.username.as_ref(),
            password: self.password.expose_secret().as_ref(),
            grant_type: "password",
        };
        let res = self
            .http_client
            .post(&url)
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;
        let response_json = res.json::<ClientCredentialsGrantResponse>().await?;
        Ok(response_json)
    }

    pub async fn get_gfycats(
        &self,
        access_token: String,
    ) -> Result<Vec<GfycatPost>, reqwest::Error> {
        let url = format!("{}/me/gfycats", self.base_url);
        let mut posts: Vec<GfycatPost> = Vec::new();

        let mut cursor = String::from("");

        loop {
            let mut res = self
                .http_client
                .get(&url)
                .query(&[("cursor", cursor.as_str()), ("count", "100")])
                .bearer_auth(&access_token)
                .send()
                .await?
                .error_for_status()?
                .json::<AllGfycatsResponse>()
                .await?;

            posts.append(&mut res.gfycats);

            if res.cursor.len() == 0 {
                break;
            }

            cursor = String::from(&res.cursor);
        }

        Ok(posts)
    }
}
