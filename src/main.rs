mod lib;
mod spider;
mod parser;

use lib::{Config, Spider};
use std::io;
use colored::Colorize;

#[tokio::main]
async fn main() {
    println!("
╔══════════════════════════════════╗
║     RUST MULTI-THREADED          ║
║          WEB SPIDER               ║
╚══════════════════════════════════╝
    ".cyan());

    let mut target_url = String::new();
    println!("{}", "Enter Target URL (e.g., https://example.com):".yellow());
    io::stdin().read_line(&mut target_url).expect("Failed to read input");
    let target_url = target_url.trim();

    if target_url.is_empty() {
        println!("{}", "Error: URL cannot be empty.".red());
        return;
    }

    let config = Config {
        max_depth: 1, //demo speed
        max_pages: 20,
        concurrent_requests: 5, // threads
        ..Default::default()
    };

    println!("{} Starting crawl on {}...", "[*]".green(), target_url);
    
    let mut spider = Spider::new(config);
    let start_time = std::time::Instant::now();
    
    let results = spider.crawl(target_url.to_string()).await;
    
    let duration = start_time.elapsed();

    println!("
═══════════════════════════════════
          CRAWL REPORT
═══════════════════════════════════
    ".magenta());
    
    println!("{} Pages Found: {}", "[+]".white(), results.len());
    println!("{} Time Taken: {:.2?}", "[+]".white(), duration);
    println!("═══════════════════════════════════");
}