#![allow(non_snake_case)]

#[derive(structopt::StructOpt)]
enum Args {
    /// Clones/Downloads all schemes and templates from sources.yaml to the current directory
    Update,
    /// Builds all schemes from all templates in the current directory
    Build
}

#[derive(Debug)]
struct Scheme {
    name:   String,
    author: String,
    slug:   String,
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    base0A: String,
    base0B: String,
    base0C: String,
    base0D: String,
    base0E: String,
    base0F: String,
}

#[derive(Debug)]
struct Template {
    contents:    String,
    extension:   String,
    output_path: String
}

fn main() {
    let args: Args = structopt::StructOpt::from_args();

    match args {
        Args::Update => {
            download_sources();
        }
        Args::Build => {
            build_schemes();
        }
    }
}

fn build_schemes() {
    if std::fs::metadata("sources").is_err() || std::fs::metadata("templates").is_err() {
        error("Required sources not found in current directory, consider running an update");
        return;
    }

    let schemes = get_schemes();
    let templates = get_templates();
    for t in templates {
        for s in &schemes {
            match std::fs::create_dir_all(&t.output_path) {
                Ok(_) => {}
                Err(_) => {
                    error(format!("Failed to recursively create directory {}", t.output_path).as_str());
                }
            }

            let file_path = format!("{}/{}{}", t.output_path, s.slug, t.extension);
            let contents = render_template(t.contents.as_str(), s);
            
            match std::fs::write(&file_path, contents) {
                Ok(_) => {
                    success(format!("Built {}", file_path).as_str());
                }
                Err(_) => {
                    error(format!("Failed to build/write to {}", file_path).as_str())
                }
            }
        }
    }
}

fn get_schemes() -> Vec<Scheme> {
    let mut schemes: Vec<Scheme> = Vec::new();
    
    for directory in std::fs::read_dir("schemes").unwrap() {
        let directory_path = directory
            .unwrap()
            .path();
        let directory_path = directory_path
            .to_str()
            .unwrap();

        for file in std::fs::read_dir(directory_path).unwrap() {
            let slug = file
                .as_ref()
                .unwrap()
                .file_name();

            let slug = slug
                .to_str()
                .unwrap()
                .replace(" ", "_")
                .replace(".yaml", "")
                .replace(".yml", "")
                .to_lowercase();

            let slug = if slug.contains("base16-") {
                slug
            } else {
                format!("base16-{}", slug)
            };

            let extension = file
                .as_ref()
                .unwrap()
                .path();
            let extension = extension
                .extension();
            
            if let Some(extension) = extension {
                if extension == "yaml" || extension == "yml" {
                    success(format!("Reading scheme {}", file.as_ref().unwrap().path().to_str().unwrap()).as_str());
                    let scheme_yaml = read_yaml_file(
                        file
                        .unwrap()
                        .path()
                        .to_str()
                        .unwrap()
                    );

                    let mut scheme = Scheme {
                        name:   String::new(),
                        author: String::new(),
                        slug:   slug,
                        base00: String::new(),
                        base01: String::new(),
                        base02: String::new(),
                        base03: String::new(),
                        base04: String::new(),
                        base05: String::new(),
                        base06: String::new(),
                        base07: String::new(),
                        base08: String::new(),
                        base09: String::new(),
                        base0A: String::new(),
                        base0B: String::new(),
                        base0C: String::new(),
                        base0D: String::new(),
                        base0E: String::new(),
                        base0F: String::new(),
                    };
                    
                    for (key, value) in scheme_yaml {
                        let key = key
                            .as_str()
                            .unwrap();
                        let value = value
                            .as_str()
                            .unwrap()
                            .to_string();
                        
                        match key {
                            "scheme" => {
                                scheme.name = value;
                            }
                            "author" => {
                                scheme.author = value;
                            }
                            "base00" => {
                                scheme.base00 = value;
                            }
                            "base01" => {
                                scheme.base01 = value;
                            }
                            "base02" => {
                                scheme.base02 = value;
                            }
                            "base03" => {
                                scheme.base03 = value;
                            }
                            "base04" => {
                                scheme.base04 = value;
                            }
                            "base05" => {
                                scheme.base05 = value;
                            }
                            "base06" => {
                                scheme.base06 = value;
                            }
                            "base07" => {
                                scheme.base07 = value;
                            }
                            "base08" => {
                                scheme.base08 = value;
                            }
                            "base09" => {
                                scheme.base09 = value;
                            }
                            "base0A" => {
                                scheme.base0A = value;
                            }
                            "base0B" => {
                                scheme.base0B = value;
                            }
                            "base0C" => {
                                scheme.base0C = value;
                            }
                            "base0D" => {
                                scheme.base0D = value;
                            }
                            "base0E" => {
                                scheme.base0E = value;
                            }
                            "base0F" => {
                                scheme.base0F = value;
                            }
                            _ => {}
                        }
                    }

                    schemes.push(scheme);
                }
            }
        }
    }

    return schemes;
}

fn get_templates() -> Vec<Template> {
    let mut templates: Vec<Template> = Vec::new();
    
    for directory in std::fs::read_dir("templates").unwrap() {
        let program_name = directory
            .as_ref()
            .unwrap()
            .file_name();
        let program_name = program_name
            .to_str()
            .unwrap();
        let directory_path = directory
            .unwrap()
            .path();
        let directory_path = directory_path
            .to_str()
            .unwrap();

        let template_config = read_yaml_file(format!("{}/templates/config.yaml", directory_path).as_str());

       
        for (template_name, value) in template_config {
            let mut template = Template {
                contents:    String::new(),
                extension:   String::new(),
                output_path: String::new()
            };

            let template_name = template_name
                .as_str()
                .unwrap();
            
            let template_path = format!("{}/templates/{}.mustache", directory_path, template_name);
            success(format!("Reading template {}", template_path).as_str());

            let contents = std::fs::read_to_string(template_path)
                .expect("Error reading template file");
            let extension: Option<&str> = value.get(&serde_yaml::Value::String(String::from("extension")))
                .unwrap()
                .as_str();
            let output = value.get(&serde_yaml::Value::String(String::from("output")))
                .unwrap()
                .as_str()
                .unwrap();

            template.contents = contents;
            template.extension = if extension.is_none() { String::new() } else { String::from(extension.unwrap()) };
            template.output_path = format!("output/{}/{}", program_name, output);
            templates.push(template);
        }
    }

    return templates;
}

fn download_sources() {
    if std::fs::metadata("sources.yaml").is_err() {
        error("sources.yaml does not exist in the current directory");
        return;
    }
    
    let sources = read_yaml_file("sources.yaml");
    let schemes_repo = sources.get(&serde_yaml::Value::String(String::from("schemes")))
        .unwrap()
        .as_str()
        .unwrap();
    let templates_repo = sources.get(&serde_yaml::Value::String(String::from("templates")))
        .unwrap()
        .as_str()
        .unwrap();

    match std::fs::metadata("sources") {
        Err(_) => {
            std::fs::create_dir("sources")
                .expect("Error creating directory");
        }
        Ok(_) => {
            warn("the file/folder \"sources\" will be overwritten");
            if std::path::PathBuf::from("sources").is_dir() {
                std::fs::remove_dir_all("sources").expect("Error removing directory");
            } else if std::path::PathBuf::from("sources").is_file() {
                std::fs::remove_file("sources").expect("Error removing file");
            }
        }
    }

    git_clone(schemes_repo, "sources/schemes");
    git_clone(templates_repo, "sources/templates");

    let schemes = read_yaml_file("sources/schemes/list.yaml");
    let templates = read_yaml_file("sources/templates/list.yaml");

    for scheme in &schemes {
        let name = scheme.0.as_str().unwrap();
        let repo = scheme.1.as_str().unwrap();
        git_clone(repo, format!("schemes/{}", name).as_str());
    }
    for template in &templates {
        let name = template.0.as_str().unwrap();
        let repo = template.1.as_str().unwrap();
        git_clone(repo, format!("templates/{}", name).as_str());
    }

    success("Finished!");
}

fn error(message: &str) {
    eprintln!("\x1b[31;1mError: {}", message);
}

fn success(message: &str) {
    println!("\x1b[32;1m{}", message);
}

fn warn(message: &str) {
    println!("\x1b[33;1mWarning: {}", message);
}

fn git_clone(repo: &str, path: &str) {
    if std::fs::metadata(path).is_ok() {
        warn(format!("folder at \"{}\" will be overwritten", path).as_str());
        std::fs::remove_dir_all(path)
            .expect("Error removing directory");
    }

    match git2::Repository::clone(repo, path) {
        Ok(_) => {
            success(format!("Cloned \"{}\" to \"{}\"", repo, path).as_str())
        }
        Err(_) => {
            error(format!("failed cloning \"{}\" to \"{}\"", repo, path).as_str());
        }
    }
}

fn read_yaml_file(path: &str) -> serde_yaml::Mapping {
    let file = std::fs::read_to_string(path);
    let file = serde_yaml::from_str(file.unwrap().as_str()).unwrap();
    return file;
}

fn render_template(template_content: &str, scheme: &Scheme) -> String {
    let mut template_content = String::from(template_content);

    let mut base16: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    base16.insert("base00", scheme.base00.as_str());
    base16.insert("base01", scheme.base01.as_str());
    base16.insert("base02", scheme.base02.as_str());
    base16.insert("base03", scheme.base03.as_str());
    base16.insert("base04", scheme.base04.as_str());
    base16.insert("base05", scheme.base05.as_str());
    base16.insert("base06", scheme.base06.as_str());
    base16.insert("base07", scheme.base07.as_str());
    base16.insert("base08", scheme.base08.as_str());
    base16.insert("base09", scheme.base09.as_str());
    base16.insert("base0A", scheme.base0A.as_str());
    base16.insert("base0B", scheme.base0B.as_str());
    base16.insert("base0C", scheme.base0C.as_str());
    base16.insert("base0D", scheme.base0D.as_str());
    base16.insert("base0E", scheme.base0E.as_str());
    base16.insert("base0F", scheme.base0F.as_str());

    for (key, value) in base16 {
        let hex_r = value[0..2].to_string();
        let hex_g = value[2..4].to_string();
        let hex_b = value[4..6].to_string();
        let rgb_r = i32::from_str_radix(hex_r.as_str(), 16).unwrap();
        let rgb_g = i32::from_str_radix(hex_g.as_str(), 16).unwrap();
        let rgb_b = i32::from_str_radix(hex_b.as_str(), 16).unwrap();
        let dec_r = rgb_r / 255;
        let dec_g = rgb_g / 255;
        let dec_b = rgb_b / 255;
        
        template_content = template_content.replace("{{scheme-name}}", scheme.name.as_str());
        template_content = template_content.replace("{{scheme-author}}", scheme.author.as_str());
        template_content = template_content.replace(format!("{{{{{}-hex}}}}", key).as_str(), value);
        template_content = template_content.replace(format!("{{{{{}-hex-r}}}}", key).as_str(), hex_r.as_str());
        template_content = template_content.replace(format!("{{{{{}-hex-g}}}}", key).as_str(), hex_g.as_str());
        template_content = template_content.replace(format!("{{{{{}-hex-b}}}}", key).as_str(), hex_b.as_str());
        template_content = template_content.replace(format!("{{{{{}-dec-r}}}}", key).as_str(), dec_r.to_string().as_str());
        template_content = template_content.replace(format!("{{{{{}-dec-g}}}}", key).as_str(), dec_g.to_string().as_str());
        template_content = template_content.replace(format!("{{{{{}-dec-b}}}}", key).as_str(), dec_b.to_string().as_str());
        template_content = template_content.replace(
            format!("{{{{{}-hex-bgr}}}}", key).as_str(),
            format!("{}{}{}", hex_b, hex_g, hex_r).as_str()
        );
    }

    return template_content;
}
