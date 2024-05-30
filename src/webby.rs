use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct RequestProccessor {
    handlers: HashMap<String, Box<dyn Fn(&str) -> String>>,
}

pub enum HttpMethod {
    Get,
    Post,
    Put,
    Update,
}

impl HttpMethod {
    fn to_string(&self) -> &str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Update => "UPDATE",
        }
    }
}

impl Default for RequestProccessor {
    fn default() -> Self {
        RequestProccessor::new()
    }
}
fn render_asset(file_name: &str) -> String {
    println!("hello");
    let prefix = "src/assets";
    let path = format!("{prefix}{file_name}");
    println!("{:?}", path);
    fs::read_to_string(path).expect(&format!("The file {file_name} should be present")[..])
}

fn visit_directries(root: PathBuf, results: &mut Vec<String>) {
    if root.is_file() {
        results.push(String::from(root.to_str().unwrap()));
    } else if root.is_dir() {
        for e in fs::read_dir(root.as_path()).unwrap() {
            let e = e.unwrap();
            visit_directries(e.path(), results);
        }
    }
}

impl RequestProccessor {
    pub fn new() -> Self {
        RequestProccessor {
            handlers: HashMap::new(),
        }
    }

    pub fn serve_assets(&mut self, _path: Option<String>) {
        let mut static_files: Vec<String> = Vec::new();
        let root_path = PathBuf::from("src/assets");
        visit_directries(root_path, &mut static_files);
        let static_files: Vec<&str> = static_files
            .iter()
            .map(|path| -> &str {
                let (_prefix, file) = path.split_at(10);
                file
            })
            .collect();

        // for entry in fs::read_dir("src/assets/css").unwrap() {
        //     let dir = entry.unwrap();
        //     dir.path().is_dir();
        //     println!("{:?}", dir.path());
        // }
        println!("{:?}", static_files);
        for file in static_files {
            self.register(HttpMethod::Get, file, Box::new(render_asset));
        }
    }

    pub fn register(
        &mut self,
        method: HttpMethod,
        route: &str,
        handler: Box<dyn Fn(&str) -> String>,
    ) {
        let key = method.to_string().to_owned() + route;
        self.handlers.insert(key, handler);
    }

    pub fn execute(&self, method: HttpMethod, route: &str) -> Result<String, String> {
        let key = method.to_string().to_owned() + route;
        match self.handlers.get(&key) {
            Some(handler) => Ok(handler(route)),
            None => Err(format!("No Handler found for {:?}", route)),
        }
    }
}
