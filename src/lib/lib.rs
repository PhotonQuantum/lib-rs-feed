use std::future::Future;
use std::pin::Pin;

use chrono::{DateTime, FixedOffset};
use crates_io_api::AsyncClient;
use crates_io_api::Error as CratesIoError;

pub mod feed;
pub mod parser;

pub trait CrateIoApiExt {
    fn last_update<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<DateTime<FixedOffset>, CratesIoError>> + 'a>>;
}

impl CrateIoApiExt for AsyncClient {
    fn last_update<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<DateTime<FixedOffset>, CratesIoError>> + 'a>> {
        Box::pin(async move {
            let crate_ = self.get_crate(name).await?;
            Ok(crate_.crate_data.updated_at.into())
        })
    }
}
