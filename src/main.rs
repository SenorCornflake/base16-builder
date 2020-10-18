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


fn main() {
    simple_logger::SimpleLogger::new()
        .init()
        .expect("Error initializing logger");

    let args: Args = structopt::StructOpt::from_args();

    match args {
        Args::Update => {
            download_resources();
        }
        _ => ()
    }

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
