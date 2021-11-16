use once_cell::sync::Lazy;
use rss::{Channel, ChannelBuilder, Guid, Item, ItemBuilder};
use uuid::Uuid;

use crate::parser::Crate;

fn generate_uuid(crate_: &Crate) -> Uuid {
    static NAMESPACE: Lazy<Uuid> =
        Lazy::new(|| Uuid::new_v5(&Uuid::NAMESPACE_URL, b"https://lib.rs"));
    Uuid::new_v5(
        &*NAMESPACE,
        format!("{}-{}", crate_.meta.title, crate_.last_update).as_bytes(),
    )
}

pub fn generate_channel(desc: &str, items: Vec<Item>) -> Channel {
    ChannelBuilder::default()
        .title(format!("Lib.rs - {}", desc))
        .link("https://lib.rs")
        .description("Recently published Rust libraries and applications")
        .items(items)
        .build()
}

pub fn generate_entries(crates: Vec<Crate>) -> Vec<Item> {
    crates
        .into_iter()
        .map(|crate_| {
            let url = crate_.meta.url();
            let hash = generate_uuid(&crate_);
            let guid = {
                let mut guid = Guid::default();
                guid.set_value(format!("urn:uuid:{}", hash));
                guid.set_permalink(false);
                guid
            };
            let link = format!("{}?hash={}", url, hash);
            ItemBuilder::default()
                .title(format!(
                    "{} {}",
                    crate_.meta.title,
                    crate_.meta.version.unwrap_or("")
                ))
                .link(link)
                .pub_date(crate_.last_update.to_rfc2822())
                .guid(guid)
                .description(crate_.meta.description.to_string())
                .content(crate_.content)
                .build()
        })
        .collect()
}
