use super::feature::*;
use nvim_oxi::{api, api::opts, api::types};

pub async fn nvim_info(s: String) {
    NvimFuture::new(Box::new(move || {
        let _ = api::notify(s.as_ref(), types::LogLevel::Info, &opts::NotifyOpts {});
    }))
    .await
}

pub async fn nvim_error(s: String) {
    NvimFuture::new(Box::new(move || {
        let _ = api::notify(s.as_ref(), types::LogLevel::Error, &opts::NotifyOpts {});
    }))
    .await
}

pub async fn nvim_exec_lua(script: String) -> Result<Option<String>, String> {
    NvimFuture::new(Box::new(move || {
        let _ = crate::utils::nvim_exec_lua(script.as_str());
    }))
    .await;
    return Ok(None);
}
