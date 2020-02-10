#[derive(rust_embed::RustEmbed)]
#[folder = "www/"]
pub struct Asset;

use std::borrow::Cow;
use tokio::task::{spawn_blocking, JoinError};

impl Asset {
    pub async fn async_get(
        file_path: &'static str,
    ) -> Result<Option<Cow<'static, [u8]>>, JoinError> {
        spawn_blocking(move || Asset::get(file_path)).await
    }
}
