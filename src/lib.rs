use crate::core::tmdb::TMDB;
mod core;
mod gateway;

pub enum ApiVersion {
    V3,
}

pub struct TMDBBuilder<'a> {
    pub api_version: ApiVersion,
    pub api_token: &'a str,
}

// impl Default for TMDBBuilder {
//     fn default() -> TMDBBuilder {
//         TMDBBuilder::new("")
//     }
// }

impl<'a> TMDBBuilder<'a> {
    fn new(api_token: &str) -> TMDBBuilder {
        TMDBBuilder {
            api_version: ApiVersion::V3,
            api_token,
        }
    }
    pub fn set_api_version(&mut self, api_version: ApiVersion) {
        self.api_version = api_version;
    }

    fn build_url(&self) -> String {
        let api_version = match self.api_version {
            ApiVersion::V3 => "3",
        };
        format!("https://api.themoviedb.org/{}/", api_version)
    }

    pub fn build(&self) -> TMDB {
        let gateway_ = Box::new(gateway::ReqwestGateway::new(
            self.build_url(),
            self.api_token,
        ));

        // api_url: format!("https://api.themoviedb.org/{}", api_version),
        TMDB::new(gateway_)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_test() {
        assert_eq!(true, true);
    }

    // fn build_test_tmdb() -> TMDB {
    //     TMDBBuilder::new().build()
    // }

    #[tokio::test]
    async fn get_movie() {
        let observed = TMDBBuilder::new(
            "eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiI0YmMzMmU4Y2RiMTk3YzA5NjRiYTFjODgxODM1YTRiYiIsInN1YiI6IjY1OWMzYTJkMGQxMWYyN2I3MjhhYzcwOCIsInNjb3BlcyI6WyJhcGlfcmVhZCJdLCJ2ZXJzaW9uIjoxfQ.cLes3tzUX1wd61EenRSdL2D7ABjJK1HYezmRb3nXjAw").build();

        observed.movie_details(125359).await;
    }

    #[test]
    fn builder_builds_url() {
        let observed = TMDBBuilder::new("").build_url();
        assert_eq!(observed, "https://api.themoviedb.org/v3/")
    }
}
