<div align="center">

# 👻 NetSpectre-RS

<img src="https://img.shields.io/badge/Version-0.1.0-cyan?style=for-the-badge" />
<img src="https://img.shields.io/badge/Rust-2025-orange?style=for-the-badge&logo=rust" />
<img src="https://img.shields.io/badge/Runtime-Tokio-7c3aed?style=for-the-badge" />

<p align="center">
  <b>A high-performance, multi-threaded web reconnaissance spider engineered for speed and deep link discovery.</b>
</p>

</div>

---

### 🌌 Overview

**NetSpectre-RS** is an asynchronous web crawler built with the **Rust 2025 edition**. It utilizes a multi-threaded architecture to perform rapid domain mapping and metadata extraction. By leveraging `tokio` for its async runtime and `dashmap` for thread-safe state management, it can handle high-concurrency workloads without the overhead of traditional scrapers.

### 🚀 Key Features

* **Multi-Threaded Crawling**: Uses an `Arc<Semaphore>` to manage and limit concurrent HTTP requests effectively.
* **Asynchronous Engine**: Powered by the `tokio` 1.0 runtime for non-blocking I/O operations.
* **Domain-Locked Logic**: Includes built-in filters to ensure the spider stays within the target host's domain.
* **Robust Parsing**: Extracts page titles and cleans URLs (removing fragments) using `scraper` and `regex`.
* **Thread-Safe Visited Tracking**: Implements an `Arc<DashSet>` to prevent duplicate processing of URLs across different tasks.

---

### 🛠️ Tech Stack & Dependencies

* **HTTP Client**: `reqwest` (with JSON support)
* **HTML Parser**: `scraper` (CSS Selector based)
* **Concurrency**: `dashmap` (Lock-free concurrent hash set)
* **URL Handling**: `url` crate for safe link joining and cleaning
* **CLI UI**: `colored` for high-visibility terminal output

---

### ⚙️ Configuration

You can customize the spider's behavior in `lib.rs` via the `Config` struct:

| Parameter | Default Value | Description |
| :--- | :--- | :--- |
| `max_depth` | 2 | Maximum recursion level for link following. |
| `max_pages` | 50 | Limit of total pages to crawl. |
| `concurrent_requests` | 10 | Number of parallel threads/tasks. |
| `user_agent` | "RustSpider/1.0" | Custom HTTP User-Agent string. |

---

### 📥 Usage

1.  **Build the project**:
    ```bash
    cargo build --release
    ```

2.  **Run the binary**:
    ```bash
    cargo run
    ```

3.  **Provide Target**: Enter the full URL (e.g., `https://example.com`) when the "NetSpectre" banner appears.

---

## ⚖️ License

This project is dual-licensed under:
- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))


> [!IMPORTANT]
> **Disclaimer**: This tool is for authorized security auditing and educational purposes only. The author (**Norct**) is not responsible for any misuse or legal consequences of using this software.

---

<div align="center">

**Developed by [norct](https://github.com/Unjou) 👾** *Fullstack Web Developer | Game Localization Specialist | Pentester & Bug Hunter | Cyber Security Enthusiast*

</div>
