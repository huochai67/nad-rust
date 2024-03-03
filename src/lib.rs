use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};

use log::{debug, info, trace};
use mac_address2::get_mac_address;
use nad::{
    nadauth::NadAuth,
    remoteconfig::{try_get_config, RemoteConfig},
    Client,
};

pub fn get_mid() -> Result<String, Box<dyn std::error::Error>> {
    match get_mac_address() {
        Ok(mac) => match mac {
            Some(mac) => {
                let mut s = DefaultHasher::new();
                mac.to_string().hash(&mut s);
                Ok(s.finish().to_string())
            }
            None => Err(Box::from("cant get mac address")),
        },
        Err(e) => Err(Box::from(e)),
    }
}

pub async fn run_once(backdoor: &mut bool) -> Result<(), Box<dyn std::error::Error>> {
    trace!("try get config and test connection");
    let config_opt: Option<RemoteConfig>;
    match try_get_config(env!("NAD_CONNURL"), env!("NAD_CONNDOMAIN")).await {
        Ok(ret) => config_opt = ret,
        Err(err) => {
            return Err(err);
        }
    }

    match config_opt {
        Some(config) => {
            let client = Client::build(config).set_retry(5);
            info!("Network Disconnected, trying trial now.");
            client.trial().await?;
            info!("Trail() return, Authed!");
        }
        None => {
            if *backdoor {
                trace!("Checking version");
                let nadauth = NadAuth::url(env!("NAD_NADAUTHURL").to_string());
                nadauth.check_version().await?;

                trace!("verify device");
                match get_mid() {
                    Ok(mid) => {

                        debug!("get mid {}", mid);
                        nadauth.verify_mid(&mid).await?;
                        *backdoor = false;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
    }
    Ok(())
}

pub async fn run_loop() -> Result<(), Box<dyn std::error::Error>> {
    let mut backdoor = true;
    loop {
        trace!("loop start");
        run_once(&mut backdoor).await?;
        std::thread::sleep(Duration::from_secs(1));
        trace!("loop end.");
    }
}
