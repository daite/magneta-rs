use crate::{TorrentResult, TorrentSite};
use futures::future::join_all;
use reqwest::Client;
use scraper::{Html, Selector};
use tokio::task::spawn_blocking;

/// TorrentRJ struct holds base_url
pub struct TorrentRJ {
    pub base_url: String,
}

impl TorrentRJ {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    /// Parse the search results page and extract (title, href)
    pub fn parse_search_document(document: &Html) -> Vec<(String, String)> {
        let li_selector = Selector::parse("li.topic-item a[title]").unwrap();

        document
            .select(&li_selector)
            .filter_map(|a_tag| {
                let href = a_tag.value().attr("href")?;
                let title = a_tag.value().attr("title").unwrap_or("").to_string();
                Some((title, href.to_string()))
            })
            .collect()
    }

    /// Parse the magnet URL from detail page HTML
    pub fn parse_magnet_document(document: &Html) -> Option<String> {
        let magnet_selector = Selector::parse("a[href^=\"magnet:\"]").ok()?;
        document
            .select(&magnet_selector)
            .next()?
            .value()
            .attr("href")
            .map(|s| s.to_string())
    }
}

#[async_trait::async_trait]
impl TorrentSite for TorrentRJ {
    async fn search(&self, keyword: &str) -> anyhow::Result<Vec<TorrentResult>> {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; xhtml,application/xml;q=0.9,image/webp,*/*;q=0.8")
            .build()?;

        let search_url = format!(
            "{}/search/index?keywords={}",
            self.base_url,
            urlencoding::encode(keyword)
        );
        let resp = client.get(&search_url).send().await?.text().await?;

        let link_list = spawn_blocking({
            let resp = resp.clone();
            move || -> anyhow::Result<Vec<(String, String)>> {
                let document = Html::parse_document(&resp);
                Ok(Self::parse_search_document(&document))
            }
        })
        .await??;

        let futures = link_list.into_iter().map(|(title, href)| {
            let client = client.clone();
            let base_url = self.base_url.clone();
            async move {
                let detail_url = format!("{}{}", base_url, href);
                let detail_resp = client
                    .get(&detail_url)
                    .send()
                    .await
                    .ok()?
                    .text()
                    .await
                    .ok()?;

                let magnet = spawn_blocking(move || -> Option<String> {
                    let detail_doc = Html::parse_document(&detail_resp);
                    Self::parse_magnet_document(&detail_doc)
                })
                .await
                .ok()??;

                Some(TorrentResult { title, magnet })
            }
        });

        let results = join_all(futures).await;
        let results: Vec<TorrentResult> = results.into_iter().filter_map(|r| r).collect();

        Ok(results)
    }
}
