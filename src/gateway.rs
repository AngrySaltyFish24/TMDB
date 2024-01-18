use crate::core::model;
use crate::core::tmdb;

use async_trait::async_trait; // remove this when possible

pub struct ReqwestGateway {
    api_url: reqwest::Url,
    client: reqwest::Client,
}

impl ReqwestGateway {
    pub fn new(api_url: String, api_token: &str) -> ReqwestGateway {
        let mut headers = reqwest::header::HeaderMap::new();
        let mut header_token =
            reqwest::header::HeaderValue::from_str(format!("Bearer {}", api_token).as_ref())
                .unwrap();
        headers.insert("Authorization", header_token);

        println!("{:#?}", headers);
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = reqwest::Url::parse(api_url.as_ref()).unwrap();
        ReqwestGateway {
            api_url: url,
            client,
        }
    }
}

#[async_trait]
impl tmdb::HttpGateway for ReqwestGateway {
    async fn get(&self, request: model::RequestInfo) {
        let url = self.api_url.join(request.path.as_ref()).unwrap();
        println!("{}", url.as_str());
        let req = self.client.get(url).build().unwrap();
        println!("{:#?}", req);
        let resp = self.client.execute(req).await;

        println!("{:#?}", resp.unwrap().text().await);
        // self.client.get()
    }
    // fn get_many() {}
}
