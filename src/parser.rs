use crate::PageData;
use regex::Regex;
use scraper::{Html, Selector};
use url::Url;

pub fn parse_html(html_body: &str, base_url: &str) -> PageData {
    let document = Html::parse_document(html_body);
    
    let title_selector = Selector::parse("title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|t| t.inner_html().trim().to_string());

    // extract Links
    let link_selector = Selector::parse("a[href]").unwrap();
    let mut links = Vec::new();
    
    if let Ok(base) = Url::parse(base_url) {
        let re = Regex::new(r"(?i)^https?://").unwrap(); 
        
        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(full_url) = base.join(href) {
                    let clean_url = full_url.to_string();
                    //filter to avoid fragments (#section)
                    let final_url = clean_url.split('#').next().unwrap_or(&clean_url);
                    links.push(final_url.to_string());
                }
            }
        }
    }

    PageData {
        url: base_url.to_string(),
        title,
        links,
        status_code: None, // set during fetch
    }
}