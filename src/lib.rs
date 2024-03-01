use std::{process, time::Duration};

use log::{debug, error, info, trace};
use nad::nad::trial;

pub async fn run_once(backdoor : & mut bool) {
    trace!("loop start");
    let config = nad::nad::try_get_config(
        "http://157.255.138.34/generate_204",
        "connectivitycheck.platform.hicloud.com",
    )
    .await
    .unwrap_or_else(|err| {
        error!("Problem testing connection: {}", err);
        process::exit(1);
    });

    let nadauth = nad::nad::NadAuth::url("https://nad-worker.pinkfish.workers.dev".to_string());
    if config.is_some() {
        info!("Network Disconnected, trying trial now.");
        trial(&config.unwrap()).await.unwrap_or_else(|err| {
            error!("Problem sending trial: {}", err);
            process::exit(1);
        });
        info!("Trail() return, Authed!");
    } else {
        if *backdoor {
            trace!("Checking version");
            match nadauth.check_version().await {
                Ok(_) => {}
                Err(err) => {
                    error!("Problem checking version: {}", err);
                    process::exit(1);
                }
            }
    
            trace!("verify device");
            let machine_id = mid::get("mykey").unwrap();
            debug!("get mid {}", machine_id);
            match nadauth.verify_mid("mid").await {
                Ok(_) => {}
                Err(err) => {
                    error!("Problem verify device: {}", err);
                    process::exit(1);
                }
            }
            *backdoor = false;
        }

        trace!("loop end.");
    }
}

pub async fn run_loop() {
    let mut backdoor = true;
    loop {
        run_once(&mut backdoor).await;
        std::thread::sleep(Duration::from_secs(1));
    }
}
