use magneta::sites::TorrentTop;
use scraper::Html;
use std::fs;

/// Test: Parsing the search result page
#[test]
fn test_parse_search_document() {
    // 1. Load the sample search HTML file
    let search_html = fs::read_to_string("tests/data/torrenttop/search_sample.html")
        .expect("Failed to load search_sample.html");

    let document = Html::parse_document(&search_html);

    // 2. Call TorrentTop's search parser
    let links = TorrentTop::parse_search_document(&document);

    // 3. Validate the result
    assert!(!links.is_empty(), "Parsed search result should not be empty");

    for (title, href) in links {
        println!("Title: {}, Href: {}", title, href);
        assert!(!title.is_empty(), "Title should not be empty");
        assert!(href.starts_with("/torrent/"), "Href should start with /torrent/");
    }
}

/// Test: Parsing the magnet link from detail page
#[test]
fn test_parse_magnet_document() {
    // 1. Load the sample magnet detail HTML file
    let magnet_html = fs::read_to_string("tests/data/torrenttop/magnet_sample.html")
        .expect("Failed to load magnet_sample.html");

    let document = Html::parse_document(&magnet_html);

    // 2. Call TorrentTop's magnet parser
    let magnet = TorrentTop::parse_magnet_document(&document);

    // 3. Validate the result
    assert!(magnet.is_some(), "Parsed magnet link should not be None");

    if let Some(link) = magnet {
        println!("Magnet link: {}", link);
        assert!(link.starts_with("magnet:?xt=urn:btih:"), "Magnet link format is invalid");
    }
}