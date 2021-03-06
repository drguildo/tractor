use scraper::{Html, Selector};

fn main() {
    let url = std::env::args().nth(1).unwrap();
    let response = ureq::get(&url).call().unwrap().into_string().unwrap();

    let document = Html::parse_document(&response);
    for url in search_img_tags(&document) {
        println!("{}", url);
    }
    for url in search_anchor_tags(&document) {
        println!("{}", url);
    }
}

fn search_img_tags(document: &Html) -> Vec<&str> {
    let mut urls = Vec::new();
    let selector = Selector::parse("img").unwrap();
    for anchor in document.select(&selector) {
        if let Some(href) = anchor.value().attr("src") {
            urls.push(href);
        }
    }
    urls
}

fn search_anchor_tags(document: &Html) -> Vec<&str> {
    let mut urls = Vec::new();
    let selector = Selector::parse("a").unwrap();
    for anchor in document.select(&selector) {
        if let Some(href) = anchor.value().attr("href") {
            urls.push(href);
        }
    }
    urls
}

#[cfg(test)]
mod tests {
    use crate::search_img_tags;

    #[test]
    fn img() {
        let html = r#"<img src="http://www.example.net/example.png" />"#;
        let document = scraper::Html::parse_document(&html);
        let urls = search_img_tags(&document);
        assert!(urls.len() == 1);
        assert!(*urls.first().unwrap() == "http://www.example.net/example.png");
    }
}
