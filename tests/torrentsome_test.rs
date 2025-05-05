use magneta::site_test;
use magneta::sites::TorrentSome;
use scraper::Html;

site_test!("torrentsome", test_parse_search_document, {
    let html = include_str!("data/torrentsome/search_sample.html");
    let document = Html::parse_document(html);
    let links = TorrentSome::parse_search_document(&document);

    assert!(!links.is_empty());
    assert!(links[0].0.contains("동상이몽2"));
});

site_test!("torrentsome", test_parse_magnet_document, {
    let html = include_str!("data/torrentsome/magnet_sample.html");
    let document = Html::parse_document(html);
    let magnet = TorrentSome::parse_magnet_document(&document);

    assert!(magnet.is_some());
    assert!(magnet.unwrap().starts_with("magnet:?xt=urn:btih:"));
});
