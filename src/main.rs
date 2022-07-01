use scraper::{Html, Selector};

fn main() {
    let url = std::env::args().nth(1).unwrap();
    let response = ureq::get(&url).call().unwrap().into_string().unwrap();

    let document = Html::parse_document(&response);
    let selector = Selector::parse("a").unwrap();
    for anchor in document.select(&selector) {
        let href = anchor.value().attr("href");
        println!("{:?}", &href);
    }
}
