
use log::{trace, warn};
use reqwest::redirect::Policy;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug)]
pub struct Config {
    baseurl: String,
    wlanacip: String,
    wlanacname: String,
    mac: String,
    wlanuserip: String,
}

impl Config {
    pub fn new(
        baseurl: String,
        wlanacip: String,
        wlanacname: String,
        mac: String,
        wlanuserip: String,
    ) -> Config {
        Config {
            baseurl,
            wlanacip,
            wlanacname,
            mac,
            wlanuserip,
        }
    }
}

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

fn parse_config(url: &str) -> Result<Option<Config>, Box<dyn std::error::Error>> {
    let key1 = "/webauth.do?";
    let index1 = url.find(key1);
    if index1.is_none() {
        return Err(Box::from(format!("cant find /webauth.do?. url:{}", url)));
    }
    let (baseurl, parm) = url.split_at(index1.unwrap());
    let (_, argv) = parm.split_at(key1.len());
    let arg: Vec<&str> = argv.split('&').collect();

    let mut wlanacip: String = String::new();
    let mut wlanacname: String = String::new();
    let mut mac: String = String::new();
    let mut wlanuserip: String = String::new();
    for x in arg {
        let index = x.find('=');
        if index.is_none() {
            return Err(Box::from(format!("cant find '=', x:{}", x)));
        }
        let (key, vaule__) = x.split_at(index.unwrap());
        let (_, vaule) = vaule__.split_at(1);
        match key {
            "wlanacip" => wlanacip = vaule.to_string(),
            "wlanacname" => wlanacname = vaule.to_string(),
            "mac" => mac = vaule.to_string(),
            "wlanuserip" => wlanuserip = vaule.to_string(),
            &_ => warn!(
                "unknown arg while parsing config. key:{}, vaule:{}",
                key, vaule
            ),
        }
    }
    Ok(Some(Config::new(
        baseurl.to_string(),
        wlanacip,
        wlanacname,
        mac,
        wlanuserip,
    )))
}

pub async fn try_get_config(
    connurl: &str,
    conndomain: &str,
) -> Result<Option<Config>, Box<dyn std::error::Error>> {
    let connret = check_connection(connurl, conndomain).await?;
    if connret.is_none() {
        return Ok(None);
    }

    Ok(parse_config(connret.unwrap().as_str())?)
}

pub async fn check_connection(
    connurl: &str,
    conndomain: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let builder = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()?
        .get(connurl)
        .header("Host", conndomain)
        .timeout(Duration::from_secs(5));
    let res = send_retry(builder, 5).await?;
    let code = res.status().as_u16();
    if code == 204 {
        return Ok(None);
    }
    if code == 302 {
        let url = res.headers().get("Location");
        if url.is_none() {
            return Err(Box::from("connurl return 302 but cant find the location."));
        }
        return Ok(Some(url.unwrap().to_str()?.to_string()));
    }

    Err(Box::from("connurl return unhandle status code."))
}

pub async fn trial(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/quickAuthShare.do?wlanacip={}&wlanacname={}&userId=radius_share42401725&passwd=radius_share&mac={}&wlanuserip={}", config.baseurl, config.wlanacip, config.wlanacname, config.mac, config.wlanuserip);
    trace!("sending trial. url:{}", url);
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

pub struct NadAuth {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MsgVersion {
    version: String,
    msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MsgVerify {
    success: bool,
    data: String,
}

impl NadAuth {
    pub fn url(url_: String) -> NadAuth {
        NadAuth { url: url_ }
    }

    pub async fn check_version(&self) -> Result<(), Box<dyn std::error::Error>> {
        let res = reqwest::get(format!("{}/version", self.url))
            .await?
            .json::<MsgVersion>()
            .await?;
        let remoteversion = res.version;
        let localversion = env!("CARGO_PKG_VERSION");
        if remoteversion != localversion {
            return Err(Box::from(format!(
                "check version failed, local: {}, remote: {}",
                localversion, remoteversion
            )));
        }
        Ok(())
    }

    pub async fn verify_mid(&self, mid: &str) -> Result<(), Box<dyn std::error::Error>> {
        let res = reqwest::get(format!("{}/verify?mid={}", self.url, mid))
            .await?
            .json::<MsgVerify>()
            .await?;
        if !res.success {
            return Err(Box::from(format!(
                "verify device failed, please contact admin to verify you device, mid:{}",
                mid
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let url = r"http://192.16.99.5/webauth.do?wlanacip=192.16.99.2&wlanacname=zkbras1&wlanuserip=10.200.132.22&mac=2c:33:58:e5:b6:04&vlan=3840&url=http://www.msftconnecttest.com";
        let result = parse_config(url).unwrap().unwrap();
        print!("{:?}", result);
        assert_eq!("http://192.16.99.5".to_string(), result.baseurl);
        assert_eq!("192.16.99.2".to_string(), result.wlanacip);
        assert_eq!("zkbras1".to_string(), result.wlanacname);
        assert_eq!("10.200.132.22".to_string(), result.wlanuserip);
        assert_eq!("2c:33:58:e5:b6:04".to_string(), result.mac);
    }

    #[tokio::test]
    async fn nadauth_test() {
        let nadauth = NadAuth::url("https://nad-worker.pinkfish.workers.dev".to_string());
        nadauth.check_version().await.expect("error check version");
        nadauth
            .verify_mid("test")
            .await
            .expect("error verify device");
    }
}
