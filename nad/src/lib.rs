pub mod nadauth;
pub mod remoteconfig;
pub mod test;

use log::{debug, warn};
use remoteconfig::RemoteConfig;
use reqwest::redirect::Policy;
use std::time::Duration;

async fn send_retry(
    builder: reqwest::RequestBuilder,
    mut retry: usize,
) -> Result<reqwest::Response, reqwest::Error> {
    loop {
        retry -= 1;
        let future = builder.try_clone().unwrap().send().await;
        if future.is_ok() {
            return Ok(future.unwrap());
        }

        warn!("request failed, left retry:{}", retry);
        if retry == 0 {
            return future;
        }
    }
}

pub struct Client {
    config: RemoteConfig,
    retry: usize,
    timeout: usize,
}

impl Client {
    pub fn build(config: RemoteConfig) -> Client {
        Client {
            config,
            retry: 5,
            timeout: 5,
        }
    }

    pub fn set_retry(mut self, retry: usize) -> Client {
        self.retry = retry;
        self
    }

    pub fn set_timeout(mut self, timeout: usize) -> Client {
        self.timeout = timeout;
        self
    }

    pub async fn trial(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/quickAuthShare.do?wlanacip={}&wlanacname={}&userId=radius_share42401725&passwd=radius_share&mac={}&wlanuserip={}", self.config.baseurl, self.config.wlanacip, self.config.wlanacname, self.config.mac, self.config.wlanuserip);
        debug!("sending trial. url:{}", url);
        let builder = reqwest::Client::builder()
            .no_proxy()
            .redirect(Policy::none())
            .build()?
            .post(url)
            .timeout(Duration::from_secs(5));
        let res = send_retry(builder, 5).await?;
        if res.status() == 302 {
            return Err(Box::from(
                "trial return 302,the protal may not support trial.",
            ));
        }
        if res.status() != 200 {
            return Err(Box::from("trial return non 200 status code"));
        }
        return Ok(());
    }

    //TODO : pub async fn Auth(name: &str, password: &str) {}
}
