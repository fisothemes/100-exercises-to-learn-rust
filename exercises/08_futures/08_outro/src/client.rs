use url::Url;
use crate::{
    error::Result,
    data::{TicketId, Ticket, TicketDraft, TicketPatch},
};

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    base_url: Url,
}

impl Client {

    pub const DEFAULT_URL: &'static str = "http://127.0.0.1:20202/tickets";

    pub fn new() -> Self {
        Self::with_url(Self::DEFAULT_URL).expect("Default url format is invalid!")
    }

    pub fn with_url(url: impl AsRef<str>) -> Result<Self> {
        Ok(Self{
            client: reqwest::Client::new(),
            base_url: Url::parse(url.as_ref())?,
        })
    }

    pub async fn list_all(&self) -> Result<Vec<Ticket>> {
        Ok(
            self.client
                .get(self.base_url.as_ref())
                .send().await?
                .json().await?
        )
    }

    pub async fn create(&self, draft: &TicketDraft) -> Result<TicketId> {
        Ok(
            self.client
                .post (self.base_url.as_ref())
                .json(draft)
                .send().await?
                .json().await?
        )
    }

    pub async fn retrieve(&self, TicketId(id): TicketId) -> Result<Ticket> {

        let url = self.base_url.join(&id.to_string())?;

        Ok(
            self.client
                .get(url)
                .send().await?
                .json().await?
        )
    }

    pub async fn patch(&self, patch: TicketPatch) -> Result<Ticket> {
        use serde_json::{Value, Map};

        let url = self.base_url.join(&patch.id.to_string())?;

        let mut map = Map::new();

       if let Some(title) = patch.title {
           map.insert("title".into(), serde_json::to_value(title)?);
       }

        if let Some(desc) = patch.description {
            map.insert("description".into(), serde_json::to_value(desc)?);
        }

        if let Some(status) = patch.status {
            map.insert("status".into(), serde_json::to_value(status)?);
        }

        Ok(
            self.client
                .patch(url)
                .json(&Value::Object(map))
                .send().await?
                .json().await?
        )
    }
}

impl Default for Client {
    fn default() -> Self { Self::new() }
}
