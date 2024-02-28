pub mod nad {
    use std::time::Duration;
    use log::trace;
    use reqwest::redirect::Policy;

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

    pub async fn check_connection(
        connurl: &str,
        conndomain: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let res = reqwest::Client::builder()
            .redirect(Policy::none())
            .build()?
            .get(connurl)
            .header("Host", conndomain)
            .timeout(Duration::from_secs(5))
            .send()
            .await?;
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
    fn parse_config(url: &str) -> Result<Option<Config>, Box<dyn std::error::Error>> {
        let mut wlanacip: String = String::new();
        let mut wlanacname: String = String::new();
        let mut mac: String = String::new();
        let mut wlanuserip: String = String::new();
        let url_parsed = reqwest::Url::parse(url)?;
        for arg in url_parsed.query_pairs(){
            if arg.0 == "wlanacip"
            {
                wlanacip = arg.1.to_string();
            }
            if arg.0 == "wlanacname"
            {
                wlanacname = arg.1.to_string();
            }
            if arg.0 == "mac"
            {
                mac = arg.1.to_string();
            }
            if arg.0 == "wlanuserip"
            {
                wlanuserip = arg.1.to_string();
            }
        }
        let baseurl = url_parsed.host_str().unwrap().to_string();
        Ok(Some(Config::new(baseurl, wlanacip, wlanacname, mac, wlanuserip)))
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

    pub async fn trial(config : &Config)-> Result<(), Box<dyn std::error::Error>> 
    {
        let url = format!("http://{}/quickAuthShare.do?wlanacip={}&wlanacname={}&userId=radius_share42401725&passwd=radius_share&mac={}&wlanuserip={}", config.baseurl, config.wlanacip, config.wlanacname, config.mac, config.wlanuserip);
        trace!("sending trial. url:{}", url);
        let res = reqwest::Client::builder()
        .no_proxy()
        .redirect(Policy::none())
        .build()?
        .post(url)
        .timeout(Duration::from_secs(5))
        .send()
        .await?;
        if res.status() == 302
        {
            return Err(Box::from("trial return 302,the protal may not support trial."))
        }
        if res.status() != 200
        {
            return Err(Box::from("trial return non 200 status code"));
        }
        return Ok(());
    }
    #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let url = r"http://192.16.99.5/webauth.do?wlanacip=192.16.99.2&wlanacname=zkbras1&wlanuserip=10.200.132.22&mac=2c:33:58:e5:b6:04&vlan=3840&url=http://www.msftconnecttest.com";
        let result = parse_config(url).unwrap().unwrap();
        print!("{:?}", result);
        assert_eq!("192.16.99.5".to_string(), result.baseurl);
        assert_eq!("192.16.99.2".to_string(), result.wlanacip);
        assert_eq!("zkbras1".to_string(), result.wlanacname);
        assert_eq!("10.200.132.22".to_string(), result.wlanuserip);
        assert_eq!("2c:33:58:e5:b6:04".to_string(), result.mac);
    }
}
}

