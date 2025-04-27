use magneta::sites::TorrentTop;
use magneta::TorrentSite;
use config::Config;

pub fn create_sites(config: &Config) -> Vec<Box<dyn TorrentSite>> {
    let mut sites: Vec<Box<dyn TorrentSite>> = Vec::new();

    if let Ok(base_url) = config.get_string("torrenttop.base_url") {
        sites.push(Box::new(TorrentTop::new(base_url)));
    }

    // if you have another site
    // if let Ok(another_url) = config.get_string("anothersite.base_url") {
    //     sites.push(Box::new(AnotherSite::new(another_url)));
    // }

    sites
}
