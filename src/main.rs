#[derive(structopt::StructOpt, Debug)]
enum Args {
    Update,
    Build {
        #[structopt(long, short)]
        /// Specify template by name or path
        template: Option<String>,
        #[structopt(long, short)]
        /// Specify scheme by name or path
        scheme: Option<String>,
    }
}

#[derive(Debug)]
struct Template {
    data: String,
    extension: String,
    output: String
}

#[derive(Debug)]
struct Scheme {
    slug: String,
    name: String,
    author: String,
    colors: std::collections::HashMap<String, String>
}

fn main() {
    simple_logger::SimpleLogger::new()
        .init()
        .expect("Error initializing logger");

    let args: Args = structopt::StructOpt::from_args();

    match args {
        Args::Update => {
            download_resources();
        }
        Args::Build{template, scheme} => {
            build(template, scheme);
        }
    }
}

fn build(template: Option<String>, scheme: Option<String>) {
    if std::fs::metadata("templates").is_err() || std::fs::metadata("schemes").is_err() {
        log::error!("Required resources not found in current directory, consider running update");
        return;
    }

    let templates = get_templates();
    let schemes = get_schemes();

    for s in schemes {
        for t in templates {
            log::info!("Building {}/base16-{}{}", t.output, s.slug, t.extension);

            let mut data = rustache::HashBuilder::new();
            data = data.insert("scheme-slug", s.slug.as_ref());
            data = data.insert("scheme-name", s.name.as_ref());
            data = data.insert("scheme-author", s.author.as_ref());

            for (base, color) in &s.colors {
                data = data.insert()
            }
        }
    }
}

fn get_templates() -> Vec<Template> {
    let mut templates: Vec<Template> = Vec::new();

    for template_dir in std::fs::read_dir("templates").unwrap() {
        let template_dir = template_dir.unwrap().path();
        let template_dir_path = template_dir.to_str().unwrap();

        let template_config = parse_yaml_file(format!("{}/templates/config.yaml", template_dir_path).as_str()).unwrap();

        for (config, data) in template_config.iter() {
            let template_path = format!("{}/templates/{}.mustache", template_dir_path, config.as_str().unwrap());

            log::info!("Reading template at {}", template_path);

            let template_data = std::fs::read_to_string(template_path).unwrap();

            let template = Template {
                data: template_data,
                extension: data
                    .as_hash()
                    .unwrap()
                    .get(&yaml_rust::Yaml::from_str("extension"))
                    .unwrap()
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                output: data
                    .as_hash()
                    .unwrap()
                    .get(&yaml_rust::Yaml::from_str("output"))
                    .unwrap()
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
            };

            templates.push(template);
        }
    }

    return templates;
}

fn get_schemes() -> Vec<Scheme> {
    let mut schemes: Vec<Scheme> = Vec::new();

    let schemes_dir = std::fs::read_dir("schemes").unwrap();

    for scheme in schemes_dir {
        let scheme = scheme.unwrap();
        let scheme_files = std::fs::read_dir(scheme.path()).unwrap();
        
        for scheme_file in scheme_files {
            let scheme_file = scheme_file.unwrap().path();
            
            if let Some(extension) = scheme_file.extension() {
                if extension == "yaml" {
                    log::info!("Reading scheme {}", scheme_file.display());
                    
                    let scheme_data = parse_yaml_file(format!("{}", scheme_file.display()).as_str()).unwrap();
                    
                    let mut scheme_author = String::new();
                    let mut scheme_name = String::new();
                    let mut scheme_colors: std::collections::HashMap<String, String> = std::collections::HashMap::new();

                    for (attr, value) in scheme_data{
                        let attr = attr.as_str().unwrap();
                        let value = value.into_string().unwrap();
                        match attr {
                            "scheme" => {
                                scheme_name = value;
                            }
                            "author" => {
                                scheme_author = value;
                            }
                            _ => {
                                scheme_colors.insert(attr.to_string(), value);
                            }
                        }
                    }

                    let scheme = Scheme {
                        name: scheme_name,
                        author: scheme_author,
                        slug: scheme_file.file_stem().unwrap().to_str().unwrap().to_string(),
                        colors: scheme_colors
                    };

                    schemes.push(scheme);
                }
            }
        }
    }

    return schemes;
}

fn download_resources() {
    if std::fs::metadata("sources.yaml").is_err() {
        log::error!("sources.yaml not found.");
        return;
    }

    let sources = parse_yaml_file("sources.yaml");

    if sources.is_err() { return; }
    let sources = sources.unwrap();

    for (source, repo) in sources.iter() {
        git_clone(repo.as_str().unwrap(), format!("sources/{}", source.as_str().unwrap()).as_str());
    }

    if std::fs::metadata("sources/templates/list.yaml").is_err() {
        log::error!("sources/templates/list.yaml not found!");
        return;
    }

    if std::fs::metadata("sources/schemes/list.yaml").is_err() {
        log::error!("sources/schemes/list.yaml not found!");
        return;
    }

    let templates = parse_yaml_file("sources/templates/list.yaml");
    let schemes = parse_yaml_file("sources/schemes/list.yaml");

    if templates.is_err() || schemes.is_err() { return; }
    let templates = templates.unwrap();
    let schemes = schemes.unwrap();

    for (template, repo) in templates.iter() {
        git_clone(repo.as_str().unwrap(), format!("templates/{}", template.as_str().unwrap()).as_str());
    }

    for (scheme, repo) in schemes.iter() {
        git_clone(repo.as_str().unwrap(), format!("schemes/{}", scheme.as_str().unwrap()).as_str());
    }
}

fn parse_yaml_file(path: &str) -> Result<linked_hash_map::LinkedHashMap<yaml_rust::Yaml, yaml_rust::Yaml>, ()> {
    let file = std::fs::read_to_string(&path);

    if file.is_err() {
        log::error!("Error reading \"{}\"", path);
        return Err(());
    }

    let file = yaml_rust::YamlLoader::load_from_str(file.unwrap().as_str());

    if file.is_err() {
        log::error!("Error parsing Yaml of \"{}\"", path);
    }

    return Ok((*file.unwrap().get(0).unwrap()).as_hash().unwrap().clone());
}

fn git_clone(url: &str, path: &str) {

    if std::fs::metadata(path).is_ok() {
        log::warn!("Overwriting {}", path);

        match std::fs::remove_dir_all(path) {
            Err(_) => { log::error!("Failed to remove {}", path); }
            _ => {}
        };
    }

    match git2::Repository::clone(url, path) {
        Ok(_) => { log::info!("Cloned {} to {}", url, path) }
        Err(_) => { log::error!("Failed to clone {}", url) }
    }
}
