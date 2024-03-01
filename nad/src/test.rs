#[cfg(test)]
mod tests {
    use crate::{nadauth::NadAuth, parse_config};

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
