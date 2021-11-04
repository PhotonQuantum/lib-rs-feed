use std::collections::HashMap;
use std::env;
use std::time::Duration;

use anyhow::{anyhow, Error, Result};
use crates_io_api::AsyncClient;
use oss_rust_sdk::object::ObjectAPI;
use oss_rust_sdk::oss::OSS;
use reqwest::Client;
use reqwest::header::HOST;
use rss::Channel;
use scraper::Html;

use lib_rs_feed_lib::CrateIoApiExt;
use lib_rs_feed_lib::feed::{generate_channel, generate_entries};
use lib_rs_feed_lib::parser::{Crate, CrateMeta, extract_content, parse_new, parse_trending};

async fn fetch_with_client(
    client: &Client,
    crates_io_client: &AsyncClient,
    src: &Html,
    desc: &str,
    f: impl for<'a> Fn(&'a Html) -> Option<Vec<CrateMeta<'a>>>,
) -> Result<Channel> {
    let crate_metas = f(src).ok_or_else(|| anyhow!("list fetch error"))?;

    let crates = futures::future::try_join_all(crate_metas.into_iter().map(|meta| async {
        let content = client.get(meta.url()).send().await?.text().await?;
        let src = Html::parse_document(&*content);
        let last_update = crates_io_client.last_update(meta.title).await?;
        Result::<_, Error>::Ok(Crate {
            meta,
            last_update,
            content: extract_content(&src).unwrap(),
        })
    }))
        .await?;

    Ok(generate_channel(desc, generate_entries(crates)))
}

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = env::var("APP_OSS_ENDPOINT")?;
    let bucket = env::var("APP_OSS_BUCKET")?;
    let oss_key = env::var("APP_OSS_ACCESSKEY_ID")?;
    let oss_secret = env::var("APP_OSS_ACCESSKEY_SECRET")?;
    let oss_client = OSS::new(oss_key, oss_secret, endpoint, bucket);

    let client = Client::builder()
        .user_agent(format!(
            "{}/{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .default_headers([(HOST, "lib.rs".parse().unwrap())].into_iter().collect())
        .build()?;
    let crates_io_client = AsyncClient::new(
        format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).as_str(),
        Duration::from_millis(100),
    )?;

    let content = client
        .get("https://lib.rs/new")
        .send()
        .await?
        .text()
        .await?;
    let src = Html::parse_document(&*content);

    println!("fetching new crates");
    let new_channel = fetch_with_client(&client, &crates_io_client, &src, "new", parse_new).await?;
    println!("uploading new crates feed");
    oss_client.put_object_from_buffer(
        &*new_channel.to_string().into_bytes(),
        "new.xml",
        HashMap::from([("content-type", "application/rss+xml")]),
        None,
    )?;

    println!("fetching trending crates");
    let trending_channel =
        fetch_with_client(&client, &crates_io_client, &src, "trending", parse_trending).await?;
    println!("uploading trending crates feed");
    oss_client.put_object_from_buffer(
        &*trending_channel.to_string().into_bytes(),
        "trending.xml",
        HashMap::from([("content-type", "application/rss+xml")]),
        None,
    )?;

    Ok(())
}
