use serde_json::Value;
use std::fs;
use std::env;

// Embed all language files into the binary
const LANGUAGE_FILES: &[(&str, &str)] = &[
    ("bash", include_str!("../languages/bash.json")),
    ("c", include_str!("../languages/c.json")),
    ("c++", include_str!("../languages/c++.json")),
    ("crystal", include_str!("../languages/crystal.json")),
    ("csharp", include_str!("../languages/csharp.json")),
    ("css", include_str!("../languages/css.json")),
    ("emacs", include_str!("../languages/emacs.json")),
    ("english", include_str!("../languages/english.json")),
    ("erlang", include_str!("../languages/erlang.json")),
    ("go", include_str!("../languages/go.json")),
    ("haskell", include_str!("../languages/haskell.json")),
    ("html", include_str!("../languages/html.json")),
    ("java", include_str!("../languages/java.json")),
    ("javascript", include_str!("../languages/javascript.json")),
    ("json", include_str!("../languages/json.json")),
    ("julia", include_str!("../languages/julia.json")),
    ("lisp", include_str!("../languages/lisp.json")),
    ("lua", include_str!("../languages/lua.json")),
    ("ocaml", include_str!("../languages/ocaml.json")),
    ("perl", include_str!("../languages/perl.json")),
    ("php", include_str!("../languages/php.json")),
    ("powershell", include_str!("../languages/powershell.json")),
    ("python", include_str!("../languages/python.json")),
    ("r", include_str!("../languages/r.json")),
    ("ruby", include_str!("../languages/ruby.json")),
    ("rust", include_str!("../languages/rust.json")),
    ("scss", include_str!("../languages/scss.json")),
    ("sql", include_str!("../languages/sql.json")),
    ("swift", include_str!("../languages/swift.json")),
    ("tex", include_str!("../languages/tex.json")),
    ("typescript", include_str!("../languages/typescript.json")),
    ("vala", include_str!("../languages/vala.json")),
    ("vim", include_str!("../languages/vim.json")),
    ("wolfram", include_str!("../languages/wolfram.json")),
    ("yaml", include_str!("../languages/yaml.json")),
    ("zig", include_str!("../languages/zig.json")),
];

pub fn get_words(lang: &str) -> Vec<String> {
    // First try to get from embedded files
    if let Some(content) = get_embedded_language_content(lang) {
        match serde_json::from_str::<Value>(content) {
            Ok(json) => {
                if let Some(words_array) = json["words"].as_array() {
                    return words_array
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                } else {
                    eprintln!("No 'words' array found in embedded {}", lang);
                }
            }
            Err(_) => {
                eprintln!("Invalid JSON in embedded {}", lang);
            }
        }
    }

    // Fallback to file system (for development or custom languages)
    let possible_paths = get_language_file_paths(lang);

    let filename = possible_paths
        .iter()
        .find(|path| fs::metadata(path).is_ok())
        .cloned()
        .unwrap_or_else(|| format!("languages/{}.json", lang));

    match fs::read_to_string(&filename) {
        Ok(content) => match serde_json::from_str::<Value>(&content) {
            Ok(json) => {
                if let Some(words_array) = json["words"].as_array() {
                    words_array
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                } else {
                    eprintln!("No 'words' array found in {}", filename);
                    fallback_words()
                }
            }
            Err(_) => {
                eprintln!("Invalid JSON in {}", filename);
                fallback_words()
            }
        },
        Err(_) => {
            eprintln!("Could not read {}, using fallback words", filename);
            fallback_words()
        }
    }
}

fn get_embedded_language_content(lang: &str) -> Option<&'static str> {
    LANGUAGE_FILES
        .iter()
        .find(|(name, _)| *name == lang)
        .map(|(_, content)| *content)
}

fn fallback_words() -> Vec<String> {
    vec!["hello", "world", "test", "example", "quick", "brown", "fox"]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

fn get_language_file_paths(lang: &str) -> Vec<String> {
    let mut paths = Vec::new();
    
    paths.push(format!("languages/{}.json", lang));
    
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            paths.push(exe_dir.join("languages").join(format!("{}.json", lang)).to_string_lossy().to_string());
            
            if let Some(parent) = exe_dir.parent() {
                paths.push(parent.join("languages").join(format!("{}.json", lang)).to_string_lossy().to_string());
            }
        }
    }
    
    paths.push(format!("../languages/{}.json", lang));
    paths.push(format!("../../languages/{}.json", lang));
    
    paths
}

fn get_language_directory_paths() -> Vec<String> {
    let mut paths = Vec::new();
    
    paths.push("languages".to_string());
    
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            paths.push(exe_dir.join("languages").to_string_lossy().to_string());
            
            if let Some(parent) = exe_dir.parent() {
                paths.push(parent.join("languages").to_string_lossy().to_string());
            }
        }
    }
    
    paths.push("../languages".to_string());
    paths.push("../../languages".to_string());
    
    paths
}

pub fn get_available_languages() -> Vec<String> {
    let mut languages = Vec::new();
    
    // Add embedded languages
    for (name, _) in LANGUAGE_FILES {
        languages.push(name.to_string());
    }
    
    // Also check file system for additional languages (for development or custom languages)
    let possible_dirs = get_language_directory_paths();
    
    for dir in possible_dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with(".json") {
                        let lang_name = filename.replace(".json", "");
                        if !languages.contains(&lang_name) {
                            languages.push(lang_name);
                        }
                    }
                }
            }
        }
    }

    if languages.is_empty() {
        eprintln!("No language files found in 'languages/' directory");
        languages.push("fallback".to_string());
    }

    languages.sort();
    languages
}