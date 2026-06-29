use clap::Args;
use serde::Serialize;
use std::io::Write;
use std::time::Instant;

#[derive(Args)]
pub struct FetchArgs {
    pub url: String,
    #[arg(long, help = "Output mode: text or markdown (default: text)")]
    pub mode: Option<String>,
    #[arg(long, default_value = "30", help = "Request timeout in seconds")]
    pub timeout: u64,
    #[arg(long, help = "Write output to file instead of stdout")]
    pub output: Option<String>,
    #[arg(long, help = "User-Agent header value")]
    pub user_agent: Option<String>,
}

#[derive(Args)]
pub struct ScrapeArgs {
    pub url: String,
    #[arg(long, help = "Output mode: text, markdown, or html (default: text)")]
    pub mode: Option<String>,
    #[arg(long, default_value = "30", help = "Request timeout in seconds")]
    pub timeout: u64,
    #[arg(long, help = "CSS selector to extract a specific element")]
    pub selector: Option<String>,
    #[arg(long, help = "Write output to file instead of stdout")]
    pub output: Option<String>,
    #[arg(long, help = "User-Agent header value")]
    pub user_agent: Option<String>,
}

#[derive(Serialize)]
struct FetchResult {
    url: String,
    status: u16,
    content_type: String,
    content_length: u64,
    fetch_time_ms: u64,
    content: String,
}

pub fn run_fetch(args: &FetchArgs) -> Result<(), String> {
    crate::config::require_feature("fetch")?;

    let start = Instant::now();
    let agent = build_agent(args.timeout, args.user_agent.as_deref())?;
    let response = agent
        .get(&args.url)
        .call()
        .map_err(|e| format!("fetch failed: {}", e))?;

    let status: u16 = response.status().into();
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();
    let content_length = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);
    let body = response
        .into_body()
        .read_to_string()
        .map_err(|e| format!("failed to read response body: {}", e))?;
    let fetch_time_ms = start.elapsed().as_millis() as u64;

    let content = if content_type.contains("html") {
        if args.mode.as_deref() == Some("markdown") {
            html_to_markdown(&body)
        } else {
            html_to_text(&body)
        }
    } else {
        body
    };

    if super::json_enabled() {
        let result = FetchResult {
            url: args.url.clone(),
            status,
            content_type,
            content_length,
            fetch_time_ms,
            content,
        };
        super::emit_json(&result)
    } else if let Some(ref output_path) = args.output {
        let mut file = std::fs::File::create(output_path)
            .map_err(|e| format!("failed to create output file: {}", e))?;
        file.write_all(content.as_bytes())
            .map_err(|e| format!("failed to write output: {}", e))?;
        eprintln!("Wrote {} bytes to {}", content.len(), output_path);
        Ok(())
    } else {
        println!("{}", content);
        Ok(())
    }
}

pub fn run_scrape(args: &ScrapeArgs) -> Result<(), String> {
    crate::config::require_feature("fetch")?;

    let start = Instant::now();
    let agent = build_agent(args.timeout, args.user_agent.as_deref())?;
    let response = agent
        .get(&args.url)
        .call()
        .map_err(|e| format!("scrape failed: {}", e))?;

    let status: u16 = response.status().into();
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();
    let content_length = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);
    let body = response
        .into_body()
        .read_to_string()
        .map_err(|e| format!("failed to read response body: {}", e))?;
    let fetch_time_ms = start.elapsed().as_millis() as u64;

    let doc = scraper::Html::parse_document(&body);

    let extracted = if let Some(ref sel) = args.selector {
        let selector = scraper::Selector::parse(sel)
            .map_err(|e| format!("invalid CSS selector '{}': {}", sel, e))?;
        let mut text = String::new();
        for element in doc.select(&selector) {
            text.push_str(&element.text().collect::<Vec<_>>().join(" "));
            text.push('\n');
        }
        if text.is_empty() {
            return Err(format!("selector '{}' matched no elements", sel));
        }
        text
    } else {
        extract_main_content(&doc)
    };

    let content = match args.mode.as_deref() {
        Some("html") => {
            if let Some(ref sel) = args.selector {
                let selector = scraper::Selector::parse(sel).unwrap();
                doc.select(&selector)
                    .map(|e| e.inner_html())
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                extract_main_html(&doc)
            }
        }
        Some("markdown") => html_to_markdown(&wrap_html(&extracted)),
        _ => extracted,
    };

    if super::json_enabled() {
        let result = FetchResult {
            url: args.url.clone(),
            status,
            content_type,
            content_length,
            fetch_time_ms,
            content,
        };
        super::emit_json(&result)
    } else if let Some(ref output_path) = args.output {
        let mut file = std::fs::File::create(output_path)
            .map_err(|e| format!("failed to create output file: {}", e))?;
        file.write_all(content.as_bytes())
            .map_err(|e| format!("failed to write output: {}", e))?;
        eprintln!("Wrote {} bytes to {}", content.len(), output_path);
        Ok(())
    } else {
        println!("{}", content);
        Ok(())
    }
}

fn build_agent(timeout_secs: u64, user_agent: Option<&str>) -> Result<ureq::Agent, String> {
    let ua = user_agent
        .unwrap_or("Mozilla/5.0 (compatible; tk/0.1.0; LLM Dev Toolkit)")
        .to_string();
    let dur = std::time::Duration::from_secs(timeout_secs);
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(dur))
        .timeout_connect(Some(dur))
        .user_agent(ua)
        .http_status_as_error(false)
        .build()
        .new_agent();
    Ok(agent)
}

fn html_to_text(html: &str) -> String {
    let doc = scraper::Html::parse_document(html);
    let body_sel = scraper::Selector::parse("body").unwrap();
    let body = match doc.select(&body_sel).next() {
        Some(b) => b,
        None => return String::new(),
    };
    collect_text(body, false)
}

fn html_to_markdown(html: &str) -> String {
    let doc = scraper::Html::parse_document(html);
    let body_sel = scraper::Selector::parse("body").unwrap();
    let body = match doc.select(&body_sel).next() {
        Some(b) => b,
        None => return String::new(),
    };
    collect_text(body, true)
}

fn collect_text(element: scraper::ElementRef, markdown: bool) -> String {
    use scraper::ElementRef;

    let mut out = String::new();
    let children: Vec<_> = element.children().collect();

    for child in &children {
        if let Some(text) = child.value().as_text() {
            let t = text.text.trim();
            if !t.is_empty() {
                out.push_str(t);
                out.push(' ');
            }
        } else if let Some(el) = ElementRef::wrap(*child) {
            let tag = el.value().name();
            match tag {
                "script" | "style" | "nav" | "noscript" => {}
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    if markdown {
                        let level = tag[1..].parse::<usize>().unwrap_or(1);
                        let prefix = "#".repeat(level);
                        out.push('\n');
                        out.push_str(&prefix);
                        out.push(' ');
                    }
                    let inner = collect_text(el, markdown);
                    let trimmed = inner.trim();
                    if !trimmed.is_empty() {
                        out.push_str(trimmed);
                    }
                    out.push('\n');
                    out.push('\n');
                }
                "p" | "div" | "section" | "article" | "blockquote" | "li" => {
                    let inner = collect_text(el, markdown);
                    let trimmed = inner.trim();
                    if !trimmed.is_empty() {
                        if tag == "li" {
                            out.push_str("  - ");
                        }
                        out.push_str(trimmed);
                        out.push('\n');
                    }
                    if tag != "li" {
                        out.push('\n');
                    }
                }
                "br" => {
                    out.push('\n');
                }
                "a" => {
                    let inner = collect_text(el, markdown);
                    let trimmed = inner.trim();
                    if !trimmed.is_empty() {
                        if markdown {
                            if let Some(href) = el.value().attr("href") {
                                out.push_str(&format!("[{}]({})", trimmed, href));
                                continue;
                            }
                        }
                        out.push_str(trimmed);
                        out.push(' ');
                    }
                }
                "img" => {
                    if markdown {
                        if let Some(alt) = el.value().attr("alt") {
                            if let Some(src) = el.value().attr("src") {
                                out.push_str(&format!("![{}]({}) ", alt, src));
                            }
                        }
                    }
                }
                "pre" | "code" => {
                    let inner = collect_text(el, markdown);
                    let trimmed = inner.trim();
                    if !trimmed.is_empty() {
                        if markdown {
                            if tag == "pre" {
                                out.push_str("```\n");
                                out.push_str(trimmed);
                                out.push('\n');
                                out.push_str("```\n");
                            } else {
                                out.push_str(&format!("`{}` ", trimmed));
                            }
                        } else {
                            out.push_str(trimmed);
                            out.push('\n');
                        }
                    }
                }
                _ => {
                    out.push_str(&collect_text(el, markdown));
                }
            }
        }
    }

    out
}

fn extract_main_content(doc: &scraper::Html) -> String {
    let selectors = [
        "article",
        "main",
        "[role=main]",
        ".content",
        ".post",
        ".article",
        "#content",
        "#main",
    ];

    for sel_str in &selectors {
        if let Ok(sel) = scraper::Selector::parse(sel_str) {
            if let Some(el) = doc.select(&sel).next() {
                let text = collect_text(el, false);
                let trimmed = text.trim();
                if !trimmed.is_empty() && trimmed.len() > 50 {
                    return trimmed.to_string();
                }
            }
        }
    }

    let body_sel = scraper::Selector::parse("body").unwrap();
    if let Some(body) = doc.select(&body_sel).next() {
        collect_text(body, false)
    } else {
        String::new()
    }
}

fn extract_main_html(doc: &scraper::Html) -> String {
    let selectors = [
        "article",
        "main",
        "[role=main]",
        ".content",
        ".post",
        ".article",
        "#content",
        "#main",
    ];

    for sel_str in &selectors {
        if let Ok(sel) = scraper::Selector::parse(sel_str) {
            if let Some(el) = doc.select(&sel).next() {
                let html = el.inner_html();
                if !html.is_empty() && html.len() > 50 {
                    return html;
                }
            }
        }
    }

    let body_sel = scraper::Selector::parse("body").unwrap();
    doc.select(&body_sel)
        .next()
        .map(|el| el.inner_html())
        .unwrap_or_default()
}

fn wrap_html(text: &str) -> String {
    format!("<html><body>{}</body></html>", text)
}
