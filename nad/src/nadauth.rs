use serde::{Deserialize, Serialize};

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
