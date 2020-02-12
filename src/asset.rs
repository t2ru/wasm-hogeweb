use anyhow::Error;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use tokio::task::spawn_blocking;

#[derive(RustEmbed)]
#[folder = "www/"]
pub struct WWW;

impl WWW {
    pub async fn async_get(file_path: &'static str) -> Result<Cow<'static, [u8]>, Error> {
        spawn_blocking(move || WWW::get(file_path).ok_or(Error::msg("no file"))).await?
    }
}
