
use git2::Repository;
use yaml_rust::{YamlLoader, Yaml};


pub fn print_color(color: &str, text: String) {
    let color_code: &str = match color {
        "red"    => "\x1b[0;31m",
        "green"  => "\x1b[0;32m",
        "yellow" => "\x1b[0;33m",
        _ => ""
    };

    println!("{}{}{}", color_code, text, "\x1b[0m");
}

pub fn git_clone(url: &str, path: &str) {
    match Repository::clone(url, path) {
        Ok(_) =>   print_color("green", format!("Cloned \"{}\" to \"{}\"", url, path)),
        Err(e) =>  print_color("red", format!("Failed to clone \"{}\" to \"{}\" | {}", url, path, e.message())),
    }
}

pub fn parse_yaml(text: &str) -> Vec<Yaml> {
    let yaml = YamlLoader::load_from_str(text)
        .unwrap();
    return yaml;
}

pub fn check_path(path: &str, expect: &str) -> Result<(), ()> {
    let metadata = std::fs::metadata(path);

    if metadata.is_err() {
        print_color("red", format!("Error trying to read \"{}\", {}", path, metadata.as_ref().unwrap_err().to_string()));
        return Err(());
    }

    let metadata = metadata.unwrap();

    if metadata.is_dir() && expect == "file" {
        print_color("red", format!("Expected file at \"{}\", found directory", path));
        return Err(());
    } else if metadata.is_file() && expect == "dir" {
        print_color("red", format!("Expected directory at \"{}\", found file", path));
        return Err(());
    }

    return Ok(());
}

pub fn home(path: &str) -> String {
    let home = std::env::var("HOME")
        .unwrap()
        .as_str()
        .to_string();
    return path.replace("~", home.as_str());
}
