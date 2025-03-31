use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Serialize;
use std::error::Error;

/// Cliente HTTP reutilizable para llamadas REST.
pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    /// Crea una nueva instancia con una URL base.
    pub fn new(base_url: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("No se pudo crear el cliente HTTP");

        Self {
            client,
            base_url: base_url.to_string(),
        }
    }

    /// Realiza una petición GET a un endpoint.
    pub fn get(&self, endpoint: &str) -> Result<Response, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let res = self.client.get(&url).send()?;
        Ok(res)
    }

    /// Realiza una petición POST con JSON.
    pub fn post<T: Serialize>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<Response, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let res = self.client.post(&url).json(body).send()?;
        Ok(res)
    }

    /// Puedes agregar PUT, DELETE, etc., de forma similar.
}