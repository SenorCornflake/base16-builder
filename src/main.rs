//#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
//#![allow(dead_code)]
mod util;


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
    contents:     String,
    program_name: String,
    extension:    String,
    output_path:  String
}

fn main() {
    println!("{:#?}", get_templates("templates"));
    let args: Args = structopt::StructOpt::from_args();

    match args {
        Args::Update => { update(); }
        _ => {}
    }

    //println!("{:#?}", util::parse_yaml("lemon:")[0]["lemon"]);
    //util::git_clone("https://github.com/SenorCornflake/base16-builder".to_string(), "./base16-builder".to_string());
    
}

fn update() {
    if util::check_path("sources.yaml", "file").is_err() {
        util::print_color("red", "sources.yaml not found in current directory".to_string());
        return
    }

//    let sources = read_yaml_file("sources.yaml");
//    let schemes_repo = sources.get(&serde_yaml::Value::String(String::from("schemes")))
//        .unwrap()
//        .as_str()
//        .unwrap();
//    let templates_repo = sources.get(&serde_yaml::Value::String(String::from("templates")))
//        .unwrap()
//        .as_str()
//        .unwrap();
//
//    match std::fs::metadata("sources") {
//        Err(_) => {
//            std::fs::create_dir("sources")
//                .expect("Error creating directory");
//        }
//        Ok(_) => {
//            warn("the file/folder \"sources\" will be overwritten");
//            if std::path::PathBuf::from("sources").is_dir() {
//                std::fs::remove_dir_all("sources").expect("Error removing directory");
//            } else if std::path::PathBuf::from("sources").is_file() {
//                std::fs::remove_file("sources").expect("Error removing file");
//            }
//        }
//    }
//
//    git_clone(schemes_repo, "sources/schemes");
//    git_clone(templates_repo, "sources/templates");
//
//    let schemes = read_yaml_file("sources/schemes/list.yaml");
//    let templates = read_yaml_file("sources/templates/list.yaml");
//
//    for scheme in &schemes {
//        let name = scheme.0.as_str().unwrap();
//        let repo = scheme.1.as_str().unwrap();
//        git_clone(repo, format!("schemes/{}", name).as_str());
//    }
//    for template in &templates {
//        let name = template.0.as_str().unwrap();
//        let repo = template.1.as_str().unwrap();
//        git_clone(repo, format!("templates/{}", name).as_str());
//    }
    let sources = std::fs::read_to_string("sources.yaml")
        .expect("Failed to read file");
    let sources = &util::parse_yaml(&sources)[0];
    let templates_repo = sources["templates"]
        .as_str()
        .unwrap();
    let schemes_repo = sources["schemes"]
        .as_str()
        .unwrap();

    // TODO: Use metadata instead of PathBuf to detect if file or dir
    match std::fs::metadata("sources") {
        Err(_) => {
            std::fs::create_dir("sources")
                .expect("Error creating directory");
        }
        Ok(_) => {
            util::print_color("red", "the file/folder \"sources\" will be overwritten".to_string());
            if std::path::PathBuf::from("sources").is_dir() {
                std::fs::remove_dir_all("sources").expect("Error removing directory");
            } else if std::path::PathBuf::from("sources").is_file() {
                std::fs::remove_file("sources").expect("Error removing file");
            }
        }
    }

    util::git_clone(templates_repo, "sources/templates");
    util::git_clone(schemes_repo, "sources/schemes");
    
    let templates = std::fs::read_to_string("sources/templates/list.yaml")
        .expect("Failed to read file");
    let schemes = std::fs::read_to_string("sources/schemes/list.yaml")
        .expect("Failed to read file");

    let templates = &util::parse_yaml(&templates)[0];
    let schemes = &util::parse_yaml(&schemes)[0];

    for (name, repo) in templates.as_hash().unwrap() {
        let name = name
            .as_str()
            .unwrap();
        let repo = repo
            .as_str()
            .unwrap();
        util::git_clone(repo, &format!("templates/{}", name))
    }

    for (name, repo) in schemes.as_hash().unwrap() {
        let name = name
            .as_str()
            .unwrap();
        let repo = repo
            .as_str()
            .unwrap();
        util::git_clone(repo, &format!("schemes/{}", name))
    }
}

fn create_templates(template_repo: &str) -> Result<Vec<Template>, ()> {
    let template_repo = util::home(template_repo);
    // Look for both yml and yaml files, just is case
    let mut config_ext = "";
    let mut exists = false;

    if util::check_path(&format!("{}/templates/config.yaml", &template_repo), "file").is_ok() {
        config_ext = "yaml";
        exists = true;
    } else {
        return Err(());
    }

    if !exists && util::check_path(&format!("{}/templates/config.yml", &template_repo), "file").is_ok() {
        config_ext = "yml";
    } else if !exists {
        return Err(());
    }

    let config = std::fs::read_to_string(&format!("{}/templates/config.{}", &template_repo, config_ext))
        .expect("Failed to read file");

    let config = &util::parse_yaml(&config)[0];

    let mut templates: Vec<Template> = Vec::new();

    for (template_name, template_config) in config.as_hash().unwrap() {
        let template_name = template_name
            .as_str()
            .unwrap();

        let output_path = template_config["output"]
            .as_str()
            .unwrap()
            .to_string();
        let extension = template_config["extension"]
            .as_str();

        let extension = if extension.is_none() {
            String::new()
        } else {
            extension
                .unwrap()
                .to_string()
        };
        
        let contents = std::fs::read_to_string(&format!("{}/templates/{}.mustache", &template_repo, template_name))
            .expect("Failed to read file");

        let program_name: Vec<&str> = template_repo
            .split("/")
            .collect();
        let program_name = program_name
            .last()
            .unwrap()
            .to_string();

        templates.push(Template {
            output_path,
            extension,
            program_name,
            contents
        });
    }

    return Ok(templates)
}

fn create_scheme(scheme_file: &str) -> Result<Scheme, ()> {   
    let scheme_file = util::home(scheme_file);
    if util::check_path(&scheme_file, "file").is_err() {
        return Err(());
    }

    let slug: Vec<&str> = scheme_file
        .split("/")
        .collect();
    let slug = slug
        .last()
        .unwrap();
    let slug = std::path::PathBuf::from(slug);
    let slug = slug
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    

    let scheme = std::fs::read_to_string(scheme_file)
        .expect("Failed to read file");

    let scheme = &util::parse_yaml(&scheme)[0];

    let mut parsed_scheme = Scheme {
        name:   String::new(),
        author: String::new(),
        slug:   slug.to_string(),
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

    for (key, value) in scheme.as_hash().unwrap() {
        let key = key
            .as_str()
            .unwrap();
        let value = value
            .as_str()
            .unwrap()
            .to_string();


        match key {
            "scheme" => { parsed_scheme.name   = value }
            "author" => { parsed_scheme.author = value }
            "base00" => { parsed_scheme.base00 = value }
            "base01" => { parsed_scheme.base01 = value }
            "base02" => { parsed_scheme.base02 = value }
            "base03" => { parsed_scheme.base03 = value }
            "base04" => { parsed_scheme.base04 = value }
            "base05" => { parsed_scheme.base05 = value }
            "base06" => { parsed_scheme.base06 = value }
            "base07" => { parsed_scheme.base07 = value }
            "base08" => { parsed_scheme.base08 = value }
            "base09" => { parsed_scheme.base09 = value }
            "base0A" => { parsed_scheme.base0A = value }
            "base0B" => { parsed_scheme.base0B = value }
            "base0C" => { parsed_scheme.base0C = value }
            "base0D" => { parsed_scheme.base0D = value }
            "base0E" => { parsed_scheme.base0E = value }
            "base0F" => { parsed_scheme.base0F = value }
            _ => {}
        }
    }

    return Ok(parsed_scheme);
}

fn get_templates(templates_dir: &str) -> Result<Vec<Vec<Template>>, ()> {
    let templates_dir = util::home(templates_dir);
    if util::check_path(&templates_dir, "dir").is_err() {
        return Err(());
    }

    let mut templates: Vec<Vec<Template>> = Vec::new();

    for template_repo in std::fs::read_dir(&templates_dir).unwrap() {
        let template_repo = template_repo
            .unwrap()
            .file_name();
        let template_repo = template_repo
            .to_str()
            .unwrap();
            
        let ts = create_templates(&format!("{}/{}", templates_dir, template_repo));
        if ts.is_ok() {
            templates.push(ts.unwrap());
        }
    }

    return Ok(templates)
}


//fn build_schemes() {
//    if std::fs::metadata("sources").is_err() || std::fs::metadata("templates").is_err() {
//        error("Required sources not found in current directory, consider running an update");
//        return;
//    }
//
//    let schemes = get_schemes();
//    let templates = get_templates();
//    for t in templates {
//        for s in &schemes {
//            match std::fs::create_dir_all(&t.output_path) {
//                Ok(_) => {}
//                Err(_) => {
//                    error(format!("Failed to recursively create directory {}", t.output_path).as_str());
//                }
//            }
//
//            let file_path = format!("{}/{}{}", t.output_path, format!("base16-{}", s.slug), t.extension);
//            let contents = render_template(t.contents.as_str(), s);
//            
//            match std::fs::write(&file_path, contents) {
//                Ok(_) => {
//                    success(format!("Built {}", file_path).as_str());
//                }
//                Err(_) => {
//                    error(format!("Failed to build/write to {}", file_path).as_str())
//                }
//            }
//        }
//    }
//}

//fn get_schemes() -> Vec<Scheme> {
//    let mut schemes: Vec<Scheme> = Vec::new();
//    
//    for directory in std::fs::read_dir("schemes").unwrap() {
//        let directory_path = directory
//            .unwrap()
//            .path();
//        let directory_path = directory_path
//            .to_str()
//            .unwrap();
//
//        for file in std::fs::read_dir(directory_path).unwrap() {
//            let slug = file
//                .as_ref()
//                .unwrap()
//                .file_name();
//
//            let slug = slug
//                .to_str()
//                .unwrap()
//                .replace(" ", "_")
//                .replace(".yaml", "")
//                .replace(".yml", "")
//                .to_lowercase();
//
//            let extension = file
//                .as_ref()
//                .unwrap()
//                .path();
//            let extension = extension
//                .extension();
//            
//            if let Some(extension) = extension {
//                if extension == "yaml" || extension == "yml" {
//                    success(format!("Reading scheme {}", file.as_ref().unwrap().path().to_str().unwrap()).as_str());
//                    let scheme_yaml = read_yaml_file(
//                        file
//                        .unwrap()
//                        .path()
//                        .to_str()
//                        .unwrap()
//                    );
//
//                    let mut scheme = Scheme {
//                        name:   String::new(),
//                        author: String::new(),
//                        slug:   slug,
//                        base00: String::new(),
//                        base01: String::new(),
//                        base02: String::new(),
//                        base03: String::new(),
//                        base04: String::new(),
//                        base05: String::new(),
//                        base06: String::new(),
//                        base07: String::new(),
//                        base08: String::new(),
//                        base09: String::new(),
//                        base0A: String::new(),
//                        base0B: String::new(),
//                        base0C: String::new(),
//                        base0D: String::new(),
//                        base0E: String::new(),
//                        base0F: String::new(),
//                    };
//                    
//                    for (key, value) in scheme_yaml {
//                        let key = key
//                            .as_str()
//                            .unwrap();
//                        let value = value
//                            .as_str()
//                            .unwrap()
//                            .to_string();
//                        
//                        match key {
//                            "scheme" => {
//                                scheme.name = value;
//                            }
//                            "author" => {
//                                scheme.author = value;
//                            }
//                            "base00" => {
//                                scheme.base00 = value;
//                            }
//                            "base01" => {
//                                scheme.base01 = value;
//                            }
//                            "base02" => {
//                                scheme.base02 = value;
//                            }
//                            "base03" => {
//                                scheme.base03 = value;
//                            }
//                            "base04" => {
//                                scheme.base04 = value;
//                            }
//                            "base05" => {
//                                scheme.base05 = value;
//                            }
//                            "base06" => {
//                                scheme.base06 = value;
//                            }
//                            "base07" => {
//                                scheme.base07 = value;
//                            }
//                            "base08" => {
//                                scheme.base08 = value;
//                            }
//                            "base09" => {
//                                scheme.base09 = value;
//                            }
//                            "base0A" => {
//                                scheme.base0A = value;
//                            }
//                            "base0B" => {
//                                scheme.base0B = value;
//                            }
//                            "base0C" => {
//                                scheme.base0C = value;
//                            }
//                            "base0D" => {
//                                scheme.base0D = value;
//                            }
//                            "base0E" => {
//                                scheme.base0E = value;
//                            }
//                            "base0F" => {
//                                scheme.base0F = value;
//                            }
//                            _ => {}
//                        }
//                    }
//
//                    schemes.push(scheme);
//                }
//            }
//        }
//    }
//
//    return schemes;
//}
//
//fn get_templates() -> Vec<Template> {
//    let mut templates: Vec<Template> = Vec::new();
//    
//    for directory in std::fs::read_dir("templates").unwrap() {
//        let program_name = directory
//            .as_ref()
//            .unwrap()
//            .file_name();
//        let program_name = program_name
//            .to_str()
//            .unwrap();
//        let directory_path = directory
//            .unwrap()
//            .path();
//        let directory_path = directory_path
//            .to_str()
//            .unwrap();
//
//        let template_config = read_yaml_file(format!("{}/templates/config.yaml", directory_path).as_str());
//
//       
//        for (template_name, value) in template_config {
//            let mut template = Template {
//                contents:    String::new(),
//                extension:   String::new(),
//                output_path: String::new()
//            };
//
//            let template_name = template_name
//                .as_str()
//                .unwrap();
//            
//            let template_path = format!("{}/templates/{}.mustache", directory_path, template_name);
//            success(format!("Reading template {}", template_path).as_str());
//
//            let contents = std::fs::read_to_string(template_path)
//                .expect("Error reading template file");
//            let extension: Option<&str> = value.get(&serde_yaml::Value::String(String::from("extension")))
//                .unwrap()
//                .as_str();
//            let output = value.get(&serde_yaml::Value::String(String::from("output")))
//                .unwrap()
//                .as_str()
//                .unwrap();
//
//            template.contents = contents;
//            template.extension = if extension.is_none() { String::new() } else { String::from(extension.unwrap()) };
//            template.output_path = format!("output/{}/{}", program_name, output);
//            templates.push(template);
//        }
//    }
//
//    return templates;
//}
//
//fn download_sources() {
//    if std::fs::metadata("sources.yaml").is_err() {
//        error("sources.yaml does not exist in the current directory");
//        return;
//    }
//    
//    let sources = read_yaml_file("sources.yaml");
//    let schemes_repo = sources.get(&serde_yaml::Value::String(String::from("schemes")))
//        .unwrap()
//        .as_str()
//        .unwrap();
//    let templates_repo = sources.get(&serde_yaml::Value::String(String::from("templates")))
//        .unwrap()
//        .as_str()
//        .unwrap();
//
//    match std::fs::metadata("sources") {
//        Err(_) => {
//            std::fs::create_dir("sources")
//                .expect("Error creating directory");
//        }
//        Ok(_) => {
//            warn("the file/folder \"sources\" will be overwritten");
//            if std::path::PathBuf::from("sources").is_dir() {
//                std::fs::remove_dir_all("sources").expect("Error removing directory");
//            } else if std::path::PathBuf::from("sources").is_file() {
//                std::fs::remove_file("sources").expect("Error removing file");
//            }
//        }
//    }
//
//    git_clone(schemes_repo, "sources/schemes");
//    git_clone(templates_repo, "sources/templates");
//
//    let schemes = read_yaml_file("sources/schemes/list.yaml");
//    let templates = read_yaml_file("sources/templates/list.yaml");
//
//    for scheme in &schemes {
//        let name = scheme.0.as_str().unwrap();
//        let repo = scheme.1.as_str().unwrap();
//        git_clone(repo, format!("schemes/{}", name).as_str());
//    }
//    for template in &templates {
//        let name = template.0.as_str().unwrap();
//        let repo = template.1.as_str().unwrap();
//        git_clone(repo, format!("templates/{}", name).as_str());
//    }
//
//    success("Finished!");
//}
//
//fn render_template(template_content: &str, scheme: &Scheme) -> String {
//    let mut template_content = String::from(template_content);
//
//    let mut base16: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
//    base16.insert("base00", scheme.base00.as_str());
//    base16.insert("base01", scheme.base01.as_str());
//    base16.insert("base02", scheme.base02.as_str());
//    base16.insert("base03", scheme.base03.as_str());
//    base16.insert("base04", scheme.base04.as_str());
//    base16.insert("base05", scheme.base05.as_str());
//    base16.insert("base06", scheme.base06.as_str());
//    base16.insert("base07", scheme.base07.as_str());
//    base16.insert("base08", scheme.base08.as_str());
//    base16.insert("base09", scheme.base09.as_str());
//    base16.insert("base0A", scheme.base0A.as_str());
//    base16.insert("base0B", scheme.base0B.as_str());
//    base16.insert("base0C", scheme.base0C.as_str());
//    base16.insert("base0D", scheme.base0D.as_str());
//    base16.insert("base0E", scheme.base0E.as_str());
//    base16.insert("base0F", scheme.base0F.as_str());
//
//    for (key, value) in base16 {
//        let hex_r = value[0..2].to_string();
//        let hex_g = value[2..4].to_string();
//        let hex_b = value[4..6].to_string();
//        let rgb_r = i32::from_str_radix(hex_r.as_str(), 16).unwrap();
//        let rgb_g = i32::from_str_radix(hex_g.as_str(), 16).unwrap();
//        let rgb_b = i32::from_str_radix(hex_b.as_str(), 16).unwrap();
//        let dec_r = rgb_r / 255;
//        let dec_g = rgb_g / 255;
//        let dec_b = rgb_b / 255;
//        
//        template_content = template_content.replace("{{scheme-name}}", scheme.name.as_str());
//        template_content = template_content.replace("{{scheme-author}}", scheme.author.as_str());
//        template_content = template_content.replace("{{scheme-slug}}", format!("base16-{}", scheme.slug).as_str());
//        template_content = template_content.replace(format!("{{{{{}-hex}}}}", key).as_str(), value);
//        template_content = template_content.replace(format!("{{{{{}-hex-r}}}}", key).as_str(), hex_r.as_str());
//        template_content = template_content.replace(format!("{{{{{}-hex-g}}}}", key).as_str(), hex_g.as_str());
//        template_content = template_content.replace(format!("{{{{{}-hex-b}}}}", key).as_str(), hex_b.as_str());
//        template_content = template_content.replace(format!("{{{{{}-dec-r}}}}", key).as_str(), dec_r.to_string().as_str());
//        template_content = template_content.replace(format!("{{{{{}-dec-g}}}}", key).as_str(), dec_g.to_string().as_str());
//        template_content = template_content.replace(format!("{{{{{}-dec-b}}}}", key).as_str(), dec_b.to_string().as_str());
//        template_content = template_content.replace(
//            format!("{{{{{}-hex-bgr}}}}", key).as_str(),
//            format!("{}{}{}", hex_b, hex_g, hex_r).as_str()
//        );
//    }
//
//    return template_content;
//}
