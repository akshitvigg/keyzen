use serde_json::Value;
use std::fs;
use std::env;
use std::path::PathBuf;

pub fn get_words(lang: &str) -> Vec<String> {
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
            break;
        }
    }

    if languages.is_empty() {
        eprintln!("No language files found in 'languages/' directory");
        languages.push("fallback".to_string());
    }

    languages.sort();
    languages
}