struct Config{
    baseurl : String,
    wlanacip : String,
    wlanacname : String,
    mac : String,
    wlanuserip : String,
}

impl Config {
    pub fn new(baseurl : String, wlanacip : String, wlanacname : String, mac : String, wlanuserip : String) -> Config {
        Config { baseurl, wlanacip, wlanacname , mac, wlanuserip }
    }
}

fn run() {
    
}