use config::Config;
use magneta::sites::{TorrentSome, TorrentTop, TorrentRJ};
use magneta::TorrentSite;

pub fn create_sites(config: &Config) -> Vec<(String, Box<dyn TorrentSite>)> {
    let mut sites: Vec<(String, Box<dyn TorrentSite>)> = Vec::new();

    if let Ok(base_url) = config.get_string("torrenttop.base_url") {
        sites.push((
            "torrenttop".to_string(),
            Box::new(TorrentTop::new(base_url)),
        ));
    }

    if let Ok(base_url) = config.get_string("torrentrj.base_url") {
        sites.push((
            "torrentrj".to_string(),
            Box::new(TorrentRJ::new(base_url)),
        ));
    }

    if let Ok(base_url) = config.get_string("torrentsome.base_url") {
        sites.push((
            "torrentsome".to_string(),
            Box::new(TorrentSome::new(base_url)),
        ));
    }

    // Example for additional site:
    // if let Ok(another_url) = config.get_string("anothersite.base_url") {
    //     sites.push(("anothersite".to_string(), Box::new(AnotherSite::new(another_url))));
    // }

    sites
}
