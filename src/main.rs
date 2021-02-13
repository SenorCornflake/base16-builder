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
    Build {
        #[structopt(long, short)]
        /// The path to a template repository
        template_repo: Option<String>,
        #[structopt(long, short = "n")]
        /// The name of the template to use
        template_name: Option<String>,
        #[structopt(long, short)]
        /// The path to a scheme file
        scheme: Option<String>,
        #[structopt(long, short)]
        /// The directory to place all generated schemes in
        output_root: Option<String>,
        #[structopt(long, short = "f")]
        /// The name of the file of the generated scheme(s)
        output_file: Option<String>,
    }
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
    output_path:  String,
    name:         String
}

fn main() {
    let args: Args = structopt::StructOpt::from_args();

    match args {
        Args::Update => { download_sources(); }
        Args::Build { template_repo, template_name, scheme, output_root, output_file } => {
            let mut templates: Vec<Template> = Vec::new();
            let mut schemes: Vec<Scheme> = Vec::new();
            let output_root = if output_root.is_some() {
                output_root.unwrap()
            } else {
                String::from("output")
            };

            if template_repo.is_some() {
                let template = util::home(&template_repo.unwrap());
                let t = create_templates(&template);
                if t.is_err() {
                    return;
                }
                templates.extend(t.unwrap());
            } else {
                let t = get_templates("templates");
                if t.is_err() {
                    return;
                }
                templates = t.unwrap();
            }

            if scheme.is_some() {
                let scheme = util::home(&scheme.unwrap());
                let s = create_scheme(&scheme);
                if s.is_err() {
                    return;
                }
                schemes.push(s.unwrap());
            } else {
                let s = get_schemes("schemes");
                if s.is_err() {
                    return;
                }
                schemes = s.unwrap();
            }

            for t in &templates {
                if template_name.is_some() && &t.name != template_name.as_ref().unwrap() {
                    continue;
                }
                for s in &schemes {
                    let output_path = if output_file.is_some() {
                        (&output_root).to_string()
                    } else {
                        format!("{}/{}", &output_root, t.output_path)
                    };

                    let output_file = if output_file.is_some() {
                        output_file.as_ref().unwrap().as_str().to_string()
                    } else {
                        format!("base16-{}{}", s.slug, t.extension)
                    };

                    match std::fs::create_dir_all(&output_path) {
                        Ok(_) => {}
                        Err(e) => {
                            util::print_color("red", format!("Failed to recursively create directory \"{}\", {}", output_path, e.to_string()));
                            return;
                        }
                    }
                    
                    util::print_color("yellow", format!("Building \"{}\" for \"{}\" using template \"{}\"", s.slug, t.program_name, t.name));
                    let rendered_contents = render_template(&t.contents, &s);

                    match std::fs::write(&format!("{}/{}", output_path, output_file), rendered_contents) {
                        Ok(_) => {}
                        Err(e) => {
                            util::print_color("red", format!("Failed to write file \"{}/{}\", {}", output_path, output_file, e.to_string()));
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn download_sources() {
    if util::check_path("sources.yaml", "file").is_err() {
        util::print_color("red", "sources.yaml not found in current directory".to_string());
        return
    }

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

    for (name, template_config) in config.as_hash().unwrap() {
        let name = name
            .as_str()
            .unwrap()
            .to_string();

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
        
        let contents = std::fs::read_to_string(&format!("{}/templates/{}.mustache", &template_repo, name))
            .expect("Failed to read file");

        let mut program_name: Vec<&str> = template_repo
            .split("/")
            .collect();
        if program_name.last().unwrap() == &"" {
            program_name.pop();
        }
        let program_name = program_name
            .last()
            .unwrap()
            .to_string();

        templates.push(Template {
            output_path,
            extension,
            program_name,
            contents,
            name,
        });
    }

    return Ok(templates)
}

fn get_schemes(schemes_dir: &str) -> Result<Vec<Scheme>, ()>{
    if util::check_path(schemes_dir, "dir").is_err() {
        return Err(());
    }
    let mut schemes: Vec<Scheme> = Vec::new();
    for scheme_repo in std::fs::read_dir(schemes_dir).unwrap() {
        let scheme_repo = scheme_repo
            .unwrap()
            .file_name();
        let scheme_repo = scheme_repo
            .to_str()
            .unwrap();
        
        for file in std::fs::read_dir(&format!("{}/{}", schemes_dir, scheme_repo)).unwrap() {
            let file_name = file
                .unwrap()
                .file_name();
            let file_name = file_name
                .to_str()
                .unwrap();

            let file_ext = std::path::PathBuf::from(file_name);
            let file_ext = file_ext
                .extension();
            let file_ext = if file_ext.is_some() {
                file_ext
                    .unwrap()
                    .to_str()
                    .unwrap()
            } else {
                ""
            };

            if file_ext == "yml" || file_ext == "yaml" {
                let scheme = create_scheme(&format!("{}/{}/{}", schemes_dir, scheme_repo, file_name));
                if scheme.is_err() {
                    return Err(());
                }
                schemes.push(scheme.unwrap());
            }
        }
    }
    return Ok(schemes)
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

fn get_templates(templates_dir: &str) -> Result<Vec<Template>, ()> {
    let templates_dir = util::home(templates_dir);
    if util::check_path(&templates_dir, "dir").is_err() {
        return Err(());
    }

    let mut templates: Vec<Template> = Vec::new();

    for template_repo in std::fs::read_dir(&templates_dir).unwrap() {
        let template_repo = template_repo
            .unwrap()
            .file_name();
        let template_repo = template_repo
            .to_str()
            .unwrap();
            
        let ts = create_templates(&format!("{}/{}", templates_dir, template_repo));
        if ts.is_ok() {
            templates.extend(ts.unwrap());
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
        template_content = template_content.replace("{{scheme-slug}}", format!("base16-{}", scheme.slug).as_str());
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
