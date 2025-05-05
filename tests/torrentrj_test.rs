use magneta::site_test;
use magneta::sites::TorrentRJ;
use scraper::Html;

site_test!("torrentrj", test_parse_search_document, {
    let html = include_str!("data/torrentrj/search_sample.html");
    let document = Html::parse_document(html);
    let links = TorrentRJ::parse_search_document(&document);
    assert!(!links.is_empty());
    assert!(links[0].0.contains("동상이몽2"));
});

site_test!("torrentrj", test_parse_magnet_document, {
    let html = include_str!("data/torrentrj/magnet_sample.html");
    let document = Html::parse_document(html);
    let magnet = TorrentRJ::parse_magnet_document(&document);
    println!("{:?}", magnet);
    assert!(magnet.is_some());
    assert!(magnet.unwrap().starts_with("magnet:?xt=urn:btih:"));
});
