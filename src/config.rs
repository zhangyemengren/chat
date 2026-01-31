use std::{collections::HashMap, env, fs, io};

fn load_env_file(path: &str) -> io::Result<HashMap<String, String>> {
    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(HashMap::new()),
        Err(e) => return Err(e),
    };

    let mut map = HashMap::new();

    for (lineno, raw) in content.lines().enumerate() {
        let line = raw.trim();

        // 空行 / 注释
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            eprintln!("[.env:{}] ignore invalid line: {}", lineno + 1, raw);
            continue;
        };

        let key = key.trim();
        if key.is_empty() {
            eprintln!("[.env:{}] ignore empty key: {}", lineno + 1, raw);
            continue;
        }

        let value = strip_quotes_strict(value.trim());
        map.insert(key.to_string(), value.to_string());
    }

    Ok(map)
}

fn strip_quotes_strict(s: &str) -> &str {
    if s.len() >= 2 {
        let bytes = s.as_bytes();
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return &s[1..s.len() - 1];
        }
    }
    s
}

fn get_env(key: &str, dotenv: &HashMap<String, String>) -> Option<String> {
    env::var(key).ok().or_else(|| dotenv.get(key).cloned())
}

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> io::Result<Self> {
        let dotenv = load_env_file(".env")?;

        let api_key = get_env("API_KEY", &dotenv)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "API_KEY is missing"))?;

        Ok(Self { api_key })
    }
}
