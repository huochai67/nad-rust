use std::time::Duration;

use log::{debug, warn};
use reqwest::redirect::Policy;

use crate::send_retry;

#[derive(Debug)]
pub struct RemoteConfig {
    pub baseurl: String,
    pub wlanacip: String,
    pub wlanacname: String,
    pub mac: String,
    pub wlanuserip: String,
}

impl RemoteConfig {
    pub fn new(
        baseurl: String,
        wlanacip: String,
        wlanacname: String,
        mac: String,
        wlanuserip: String,
    ) -> RemoteConfig {
        RemoteConfig {
            baseurl,
            wlanacip,
            wlanacname,
            mac,
            wlanuserip,
        }
    }
}

pub fn parse_config(url: &str) -> Result<Option<RemoteConfig>, Box<dyn std::error::Error>> {
    debug!("try parse config : {}", url);
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
    Ok(Some(RemoteConfig::new(
        baseurl.to_string(),
        wlanacip,
        wlanacname,
        mac,
        wlanuserip,
    )))
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

pub async fn try_get_config(
    connurl: &str,
    conndomain: &str,
) -> Result<Option<RemoteConfig>, Box<dyn std::error::Error>> {
    let connret = check_connection(connurl, conndomain).await?;
    if connret.is_none() {
        return Ok(None);
    }

    Ok(parse_config(connret.unwrap().as_str())?)
}
