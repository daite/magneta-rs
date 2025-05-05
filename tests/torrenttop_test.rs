use magneta::site_test;
use magneta::sites::TorrentTop;
use scraper::Html;

site_test!("torrenttop", test_parse_search_document, {
    let html = include_str!("data/torrenttop/search_sample.html");
    let document = Html::parse_document(html);
    let links = TorrentTop::parse_search_document(&document);

    assert!(!links.is_empty());
    assert!(links[0].0.contains("슬기로울"));
});

site_test!("torrenttop", test_parse_magnet_document, {
    let html = include_str!("data/torrenttop/magnet_sample.html");
    let document = Html::parse_document(html);
    let magnet = TorrentTop::parse_magnet_document(&document);

    assert!(magnet.is_some());
    assert!(magnet.unwrap().starts_with("magnet:?xt=urn:btih:"));
});
