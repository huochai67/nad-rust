use std::time::Duration;

use log::{debug, info, trace};
use nad::{trial, try_get_config, NadAuth};

pub async fn run_once(backdoor: &mut bool) -> Result<(), Box<dyn std::error::Error>> {
    trace!("loop start");
    trace!("try get config and test connection");
    let config_opt: Option<nad::Config>;
    match try_get_config(
        env!("NAD_CONNURL"),
        env!("NAD_CONNDOMAIN"),
    )
    .await
    {
        Ok(ret) => config_opt = ret,
        Err(err) => {
            return Err(err);
        }
    }

    match config_opt {
        Some(config) => {
            info!("Network Disconnected, trying trial now.");
            trial(&config).await?;
            info!("Trail() return, Authed!");
        }
        None => {
            if *backdoor {
                trace!("Checking version");
                let nadauth = NadAuth::url(env!("NAD_NADAUTHURL").to_string());
                nadauth.check_version().await?;

                trace!("verify device");
                let machine_id = mid::get("NAD").unwrap();
                debug!("get mid {}", machine_id);
                nadauth.verify_mid(&machine_id).await?;
                *backdoor = false;
            }
        }
    }
    trace!("loop end.");
    Ok(())
}

pub async fn run_loop() -> Result<(), Box<dyn std::error::Error>> {
    let mut backdoor = true;
    loop {
        run_once(&mut backdoor).await?;
        std::thread::sleep(Duration::from_secs(1));
    }
}
