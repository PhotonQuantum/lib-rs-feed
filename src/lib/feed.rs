use rss::{Channel, ChannelBuilder, Item, ItemBuilder};

use crate::parser::Crate;

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
            ItemBuilder::default()
                .title(format!(
                    "{} {}",
                    crate_.meta.title,
                    crate_.meta.version.unwrap_or("")
                ))
                .link(url)
                .pub_date(crate_.last_update.to_rfc2822())
                .description(crate_.meta.description.to_string())
                .content(crate_.content)
                .build()
        })
        .collect()
}
