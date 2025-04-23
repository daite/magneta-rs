use crate::{TorrentSite, TorrentResult};

pub struct TorrentTop;

#[async_trait::async_trait]
impl TorrentSite for TorrentTop {
    async fn search(&self, keyword: &str) -> anyhow::Result<Vec<TorrentResult>> {
        // HTML 파싱 로직 구현 예정
        Ok(vec![])
    }
}
