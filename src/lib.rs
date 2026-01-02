pub mod spider;
pub mod parser;

use std::collections::HashSet;
use dashmap::DashSet;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Config {
    pub max_depth: usize,
    pub max_pages: usize,
    pub concurrent_requests: usize,
    pub user_agent: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_depth: 2,
            max_pages: 50,
            concurrent_requests: 10,
            user_agent: String::from("RustSpider/1.0"),
        }
    }
}

#[derive(Debug)]
pub struct PageData {
    pub url: String,
    pub title: Option<String>,
    pub links: Vec<String>,
    pub status_code: Option<u16>,
}
