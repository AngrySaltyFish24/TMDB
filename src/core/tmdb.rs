use super::model;
use async_trait::async_trait; // remove this when possible

#[async_trait]
pub trait HttpGateway {
    async fn get(&self, request_info: model::RequestInfo);
    // fn get_many();
}

pub struct TMDB {
    // api_url: String,
    gateway: Box<dyn HttpGateway>,
}
// 125359 --django
impl TMDB {
    pub fn new(gateway: Box<dyn HttpGateway>) -> TMDB {
        TMDB { gateway }
    }
    pub async fn movie_details(&self, movie_id: u32) {
        let details = model::RequestInfo {
            path: format!("movie/{}", movie_id),
        };

        self.gateway.get(details).await;
    }
}
