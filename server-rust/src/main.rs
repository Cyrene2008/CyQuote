// CyQuote API Server (Rust)
// 单二进制语录服务：首次运行生成 config.json 与示例 data/quotes.jsonc。
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{json, Map, Value};

const DEFAULT_CONFIG: &str = "{\n  \"listen\": \"127.0.0.1\",\n  \"port\": 9093,\n  \"quotesFile\": \"data/quotes.jsonc\",\n  \"allowOrigin\": \"*\"\n}\n";

const EXAMPLE_CATALOG: &str = "// CyQuote 语录数据示例\n// 顶层键为分类名，值为语录数组；每条包含 value（正文）、author（作者，可省略）、from（出处，可省略）。\n// 支持 // 行注释与 /* 块注释 */，也容忍多余的尾逗号。\n{\n  \"励志\": [\n    { \"value\": \"路虽远，行则将至；事虽难，做则必成。\" },\n    { \"value\": \"不积跬步，无以至千里；不积小流，无以成江海。\", \"author\": \"荀子\", \"from\": \"《劝学》\" }\n  ],\n  \"温柔\": [\n    { \"value\": \"因为世界对我温柔，我就长成温柔的模样。\", \"author\": \"德谬歌\", \"from\": \"HSR\" }\n  ]\n}\n";

struct State {
    allow_origin: String,
    catalog: Map<String, Value>,
    categories: Vec<String>,
    total: usize,
}

fn executable_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn timestamp() -> String {
    let seconds = SystemTime::now().duration_since(UNIX_EPOCH).map(|value| value.as_secs()).unwrap_or(0) as i64;
    let days = seconds.div_euclid(86400);
    let rest = seconds.rem_euclid(86400);
    let (year, month, day) = civil_from_days(days);
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, rest / 3600, (rest % 3600) / 60, rest % 60)
}

fn log_line(message: &str) {
    let line = format!("{} {}\n", timestamp(), message);
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(executable_dir().join("cyquote.log")) {
        let _ = file.write_all(line.as_bytes());
    }
    print!("{}", line);
    let _ = std::io::stdout().flush();
}

fn strip_jsonc(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut in_string = false;
    let mut escaped = false;
    let mut index = 0;
    while index < bytes.len() {
        let character = bytes[index];
        if in_string {
            out.push(character);
            if escaped {
                escaped = false;
            } else if character == b'\\' {
                escaped = true;
            } else if character == b'"' {
                in_string = false;
            }
            index += 1;
            continue;
        }
        if character == b'"' {
            in_string = true;
            out.push(character);
            index += 1;
            continue;
        }
        if character == b'/' && index + 1 < bytes.len() {
            if bytes[index + 1] == b'/' {
                while index < bytes.len() && bytes[index] != b'\n' {
                    index += 1;
                }
                out.push(b'\n');
                continue;
            }
            if bytes[index + 1] == b'*' {
                index += 2;
                while index + 1 < bytes.len() && !(bytes[index] == b'*' && bytes[index + 1] == b'/') {
                    index += 1;
                }
                index += 2;
                continue;
            }
        }
        out.push(character);
        index += 1;
    }
    let cleaned = String::from_utf8_lossy(&out).to_string();
    let mut result = String::with_capacity(cleaned.len());
    let mut in_string = false;
    let mut escaped = false;
    let chars: Vec<char> = cleaned.chars().collect();
    let mut position = 0;
    while position < chars.len() {
        let character = chars[position];
        if in_string {
            result.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            position += 1;
            continue;
        }
        if character == '"' {
            in_string = true;
            result.push(character);
            position += 1;
            continue;
        }
        if character == ',' {
            let mut next = position + 1;
            while next < chars.len() && chars[next].is_whitespace() {
                next += 1;
            }
            if next < chars.len() && (chars[next] == '}' || chars[next] == ']') {
                position += 1;
                continue;
            }
        }
        result.push(character);
        position += 1;
    }
    result
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[index + 1..index + 3]).unwrap_or("");
            if let Ok(value) = u8::from_str_radix(hex, 16) {
                out.push(value);
                index += 3;
                continue;
            }
        }
        out.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}

fn load_config(base: &Path) -> (String, u16, PathBuf, String) {
    let path = base.join("config.json");
    if !path.exists() {
        let _ = fs::write(&path, DEFAULT_CONFIG);
        log_line("已生成默认配置 config.json");
    }
    let mut listen = String::from("127.0.0.1");
    let mut port: u16 = 9093;
    let mut quotes_file = String::from("data/quotes.jsonc");
    let mut allow_origin = String::from("*");
    if let Ok(text) = fs::read_to_string(&path) {
        if let Ok(value) = serde_json::from_str::<Value>(&text) {
            if let Some(item) = value.get("listen").and_then(Value::as_str).filter(|item| !item.is_empty()) {
                listen = item.to_string();
            }
            if let Some(item) = value.get("port").and_then(Value::as_u64) {
                if item > 0 && item < 65536 {
                    port = item as u16;
                }
            }
            if let Some(item) = value.get("quotesFile").and_then(Value::as_str).filter(|item| !item.is_empty()) {
                quotes_file = item.to_string();
            }
            if let Some(item) = value.get("allowOrigin").and_then(Value::as_str).filter(|item| !item.is_empty()) {
                allow_origin = item.to_string();
            }
        }
    }
    (listen, port, base.join(quotes_file), allow_origin)
}

fn ensure_catalog_file(path: &Path) {
    if path.exists() {
        return;
    }
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, EXAMPLE_CATALOG);
    log_line(&format!("已生成示例语录文件 {}", path.display()));
}

fn load_catalog(path: &Path) -> (Map<String, Value>, Vec<String>, usize) {
    ensure_catalog_file(path);
    let mut catalog = Map::new();
    let mut categories = Vec::new();
    let mut total = 0usize;
    let Ok(text) = fs::read_to_string(path) else { return (catalog, categories, total) };
    let Ok(parsed) = serde_json::from_str::<Value>(&strip_jsonc(&text)) else {
        log_line("语录文件解析失败，请检查 JSONC 格式");
        return (catalog, categories, total);
    };
    if let Some(object) = parsed.as_object() {
        for (category, quotes) in object {
            let Some(list) = quotes.as_array() else { continue };
            let valid: Vec<Value> = list
                .iter()
                .filter(|quote| quote.get("value").and_then(Value::as_str).map(|value| !value.trim().is_empty()).unwrap_or(false))
                .cloned()
                .collect();
            if valid.is_empty() {
                continue;
            }
            total += valid.len();
            categories.push(category.clone());
            catalog.insert(category.clone(), Value::Array(valid));
        }
    }
    (catalog, categories, total)
}

fn random_index(seed: &mut u64, length: usize) -> usize {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    (*seed % length as u64) as usize
}

fn respond(stream: &mut TcpStream, state: &State, status: u16, body: &Value) {
    let payload = serde_json::to_string(body).unwrap_or_else(|_| "{}".to_string());
    let reason = match status {
        200 => "OK",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "OK",
    };
    let head = format!(
        "HTTP/1.1 {} {}\r\nAccess-Control-Allow-Origin: {}\r\nAccess-Control-Allow-Methods: GET, OPTIONS\r\nAccess-Control-Allow-Headers: *\r\nCache-Control: no-store, max-age=0\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        status,
        reason,
        state.allow_origin,
        payload.as_bytes().len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(payload.as_bytes());
    let _ = stream.flush();
}

fn query_param(target: &str, key: &str) -> String {
    let Some(index) = target.find('?') else { return String::new() };
    let query = &target[index + 1..];
    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        let name = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("");
        if name == key {
            return percent_decode(&value.replace('+', " "));
        }
    }
    String::new()
}

fn handle_client(mut stream: TcpStream, state: &Arc<State>, seed: &mut u64) {
    let Ok(clone) = stream.try_clone() else { return };
    let mut reader = BufReader::new(clone);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() || request_line.trim().is_empty() {
        return;
    }
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if line == "\r\n" || line == "\n" {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let method = parts[0];
    let target = parts[1];
    if method == "OPTIONS" {
        respond(&mut stream, state, 204, &json!({}));
        return;
    }
    if method != "GET" {
        respond(&mut stream, state, 405, &json!({ "error": "仅支持 GET" }));
        return;
    }
    let path = target.split('?').next().unwrap_or("/").trim_end_matches('/').to_string();
    match path.as_str() {
        "" | "/quote" => {
            let requested = query_param(target, "category");
            let mut selected: Vec<String> = Vec::new();
            for item in requested.split(',') {
                let name = item.trim();
                if !name.is_empty() && state.catalog.contains_key(name) && !selected.iter().any(|value| value == name) {
                    selected.push(name.to_string());
                }
            }
            if selected.is_empty() {
                selected = state.categories.clone();
            }
            let mut pool: Vec<&Value> = Vec::new();
            for category in &selected {
                if let Some(list) = state.catalog.get(category).and_then(Value::as_array) {
                    pool.extend(list.iter());
                }
            }
            if pool.is_empty() {
                respond(&mut stream, state, 404, &json!({ "error": "没有可用的语录" }));
                return;
            }
            let quote = pool[random_index(seed, pool.len())];
            respond(
                &mut stream,
                state,
                200,
                &json!({
                    "value": quote.get("value").cloned().unwrap_or(Value::String(String::new())),
                    "author": quote.get("author").cloned().unwrap_or(Value::String(String::new())),
                    "from": quote.get("from").cloned().unwrap_or(Value::String(String::new())),
                    "category": selected,
                    "source": "CyQuote"
                }),
            );
        }
        "/categories" => {
            respond(&mut stream, state, 200, &json!({ "categories": state.categories }));
        }
        "/count" => {
            let requested = query_param(target, "category");
            let mut counts = BTreeMap::new();
            for category in &state.categories {
                if let Some(list) = state.catalog.get(category).and_then(Value::as_array) {
                    counts.insert(category.clone(), list.len());
                }
            }
            if requested.is_empty() {
                respond(&mut stream, state, 200, &json!({ "total": state.total, "categories": counts }));
            } else if requested.contains(',') {
                respond(&mut stream, state, 400, &json!({ "error": "一次只能查询一个分类" }));
            } else if let Some(count) = counts.get(&requested) {
                respond(&mut stream, state, 200, &json!({ "category": requested, "count": count }));
            } else {
                respond(&mut stream, state, 404, &json!({ "error": "分类不存在", "category": requested }));
            }
        }
        "/health" => {
            respond(&mut stream, state, 200, &json!({ "status": "ok", "categories": state.categories.len(), "quotes": state.total }));
        }
        _ => respond(&mut stream, state, 404, &json!({ "error": "未知路径" })),
    }
}

fn main() {
    let base = executable_dir();
    let (listen, port, quotes_file, allow_origin) = load_config(&base);
    let (catalog, categories, total) = load_catalog(&quotes_file);
    let state = Arc::new(State { allow_origin, catalog, categories, total });
    let address = format!("{}:{}", listen, port);
    let listener = match TcpListener::bind(&address) {
        Ok(listener) => listener,
        Err(error) => {
            log_line(&format!("无法监听 {}：{}", address, error));
            return;
        }
    };
    log_line(&format!("CyQuote 已启动：http://{}（{} 个分类 / {} 条语录）", address, state.categories.len(), state.total));
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).map(|value| value.as_nanos() as u64).unwrap_or(88172645463325252) | 1;
    for connection in listener.incoming() {
        if let Ok(stream) = connection {
            let state = Arc::clone(&state);
            let mut local_seed = seed;
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            std::thread::spawn(move || handle_client(stream, &state, &mut local_seed));
        }
    }
}
