use std::{process, time::Duration};

use log::{error, info, trace};

use crate::nad::nad::trial;

pub mod nad;

pub async fn run() {
    info!("Program Start.");
    loop {
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

        if config.is_some() {
            info!("Network Disconnected, trying trial now.");
            trial(&config.unwrap()).await.unwrap_or_else(|err| {
                error!("Problem sending trial: {}", err);
                process::exit(1);
            });
            info!("Trail() return, Authed!");
        } else {
            trace!("loop end.");
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}
