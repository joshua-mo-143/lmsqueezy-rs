use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client as ReqwestClient;
use reqwest::{Error, Response};

pub struct Client {
    ctx: ReqwestClient,
}

impl Client {
    pub fn init(&self, key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let headers = self.headers(key);

        let ctx = ReqwestClient::builder().default_headers(headers);

        let Ok(res) = ctx.build() else {
            return Err("Some problems were encountered trying to build the client, please report this as an issue on GitHub!".into());
        };

        Ok(Client { ctx: res })
    }

    pub fn headers(&self, key: &str) -> HeaderMap {
        let auth_string = format!("Bearer {key}");
        let mut headers = HeaderMap::new();

        headers.insert(ACCEPT, "application/vnd.api+json".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/vnd.api+json".parse().unwrap());
        headers.insert(AUTHORIZATION, auth_string.parse().unwrap());

        headers
    }

    pub async fn send_req(&self, uri: String, reqtype: RequestType) -> Result<Response, Error> {
        let req = match reqtype {
            RequestType::Get => self.ctx.get(uri).send(),
            RequestType::Post => todo!(),
        };

        match req.await {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        }
    }
}

pub enum RequestType {
    Get,
    Post,
}
