use std::borrow::Cow;
use std::ops::Deref;

use chrono::{DateTime, FixedOffset};
use cow_utils::CowUtils;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};

pub struct Crate<'a> {
    pub meta: CrateMeta<'a>,
    pub last_update: DateTime<FixedOffset>,
    pub content: String,
}

#[derive(Debug)]
pub struct CrateMeta<'a> {
    pub url: &'a str,
    pub title: &'a str,
    pub description: Cow<'a, str>,
    pub version: Option<&'a str>,
    pub pubdate: Option<&'a str>,
}

impl CrateMeta<'_> {
    pub fn url(&self) -> String {
        format!("https://lib.rs{}", self.url)
    }
}

pub struct CrateContent {
    pub last_updated: DateTime<FixedOffset>,
    pub content: String,
}

fn parse_elem(elem: ElementRef) -> Option<CrateMeta> {
    static TITLE_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse(".h > h4").unwrap());
    static DESC_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse(".h > .desc").unwrap());
    static VERSION_SELECTOR: Lazy<Selector> =
        Lazy::new(|| Selector::parse(".meta > .version").unwrap());
    static PUBDATE_SELECTOR: Lazy<Selector> =
        Lazy::new(|| Selector::parse(".meta > .pubdate").unwrap());

    Some(CrateMeta {
        url: elem.value().attr("href")?,
        title: elem.select(&TITLE_SELECTOR).next()?.text().next()?,
        description: elem
            .select(&DESC_SELECTOR)
            .next()?
            .text()
            .next()?
            .trim()
            .cow_replace('\n', " "),
        version: elem
            .select(&VERSION_SELECTOR)
            .next()
            .and_then(|elem| Some(&**elem.children().nth(1)?.value().as_text()?)),
        pubdate: elem
            .select(&PUBDATE_SELECTOR)
            .next()
            .and_then(|elem| elem.text().next()),
    })
}

pub fn parse_new(src: &Html) -> Option<Vec<CrateMeta>> {
    static LIST_SELECTOR: Lazy<Selector> =
        Lazy::new(|| Selector::parse(".new > .crates-list > li > a").unwrap());

    src.select(&LIST_SELECTOR).map(parse_elem).collect()
}

pub fn parse_trending(src: &Html) -> Option<Vec<CrateMeta>> {
    static LIST_SELECTOR: Lazy<Selector> =
        Lazy::new(|| Selector::parse(".trending > .crates-list > li > a").unwrap());

    src.select(&LIST_SELECTOR).map(parse_elem).collect()
}

pub fn extract_content(src: &Html) -> Option<String> {
    static README_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("#readme").unwrap());

    src.select(&README_SELECTOR).next().map(|item| item.html())
}
