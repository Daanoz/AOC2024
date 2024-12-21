use reqwest::blocking::Client;
use std::{env, fs, sync::Mutex};

pub const AOC_SESSION_ENV_VAR: &str = "AOC_SESSION";

lazy_static::lazy_static! {
    static ref AOC_SESSION: Mutex<Option<String>> = Mutex::new(env::var_os(AOC_SESSION_ENV_VAR).and_then(|v| v.into_string().ok()));
}

/// Set the AOC session to use when downloading data.
pub fn set_session(session_id: String) {
    let mut session = AOC_SESSION.lock().unwrap();
    *session = Some(session_id);
}

pub enum AocDataType {
    Text,
    Input,
}

impl AocDataType {
    fn fetch(&self, day: u32, year: u32) -> Result<String, String> {
        match self {
            AocDataType::Text => {
                let path = format!("{}/day/{}", year, day);
                let output = fetch_from_aoc(&path)?;
                let output = process_puzzle_html(output)?;
                Ok(output)
            }
            AocDataType::Input => {
                let path = format!("{}/day/{}/input", year, day);
                fetch_from_aoc(&path)
            }
        }
    }

    fn file_name(&self) -> String {
        match self {
            AocDataType::Text => "text.md".into(),
            AocDataType::Input => "input".into(),
        }
    }
}

pub fn get_aoc_data(data_type: AocDataType, day: u32, year: u32) -> Result<String, String> {
    let current_dir = env::current_dir().expect("Current directory");
    let mut dir = Some(current_dir.as_path());
    let data_dir = loop {
        if let Some(d) = dir {
            if d.join("aoc_data").exists() {
                break Some(d.join("aoc_data"));
            }
            dir = d.parent();
        } else {
            break None;
        }
    }
    .unwrap_or_else(|| current_dir.join("aoc_data"));
    let data_dir = data_dir.join(year.to_string()).join(day.to_string());
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).expect("Create data directory");
    }
    let file_name = data_dir.join(data_type.file_name());
    if !file_name.exists() {
        let data = data_type.fetch(day, year)?;
        fs::write(&file_name, data.clone()).expect("Write data to file");
        return Ok(data);
    }
    fs::read_to_string(&file_name).map_err(|e| e.to_string())
}

fn fetch_from_aoc(path: &str) -> Result<String, String> {
    let aoc_session = AOC_SESSION.lock().unwrap().clone();
    let aoc_session = if let Some(session_id) = aoc_session {
        session_id
    } else {
        return Err("Cannot download input, AOC_SESSION unavailable".to_string());
    };

    let repo_url = env!("CARGO_PKG_REPOSITORY");
    let authors = env!("CARGO_PKG_AUTHORS");

    let url = format!("https://adventofcode.com/{}", path);
    let client = Client::new();
    let response = client
        .get(url)
        .header("cookie", format!("session={}", aoc_session))
        .header("User-Agent", format!("{} by {}", repo_url, authors))
        .send()
        .map_err(|e| e.to_string())?;
    let status = response.status();
    let text: String = response.text().map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("Downloading input failed: {}; {}", status, text));
    }
    Ok(text)
}

lazy_static::lazy_static! {
    static ref ARTICLE_REGEX: regex::Regex = regex::RegexBuilder::new(r#"<article class="day-desc">(.+?)<\/article>"#)
        .multi_line(true)
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    static ref LINE_ENDINGS: regex::Regex = regex::Regex::new(r#"</p>|</pre>"#).unwrap();
    static ref STRONG_BLOCK: regex::Regex = regex::Regex::new(r#"<code><em>([^<]*)</em></code>"#).unwrap();
}

pub fn process_puzzle_html(text: String) -> Result<String, String> {
    use html2md_rs::to_md::safe_from_html_to_md;

    let text = extract_puzzle_text(text)?;
    let text = LINE_ENDINGS.replace_all(&text, "\n$0").to_string();
    let text = STRONG_BLOCK
        .replace_all(&text, "<strong>$1</strong>")
        .to_string();
    let text = safe_from_html_to_md(format!("<div>{}</div>", text)).map_err(|e| e.to_string())?;
    Ok(text)
}

fn extract_puzzle_text(text: String) -> Result<String, String> {
    Ok(ARTICLE_REGEX
        .captures_iter(&text)
        .map(|c| c.get(1).unwrap().as_str().to_string())
        .collect::<Vec<String>>()
        .join("\n***\n"))
}
