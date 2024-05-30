use std::fs;

pub fn render_page(
    filename: &str,
    data: Option<String>,
    parser: Option<Box<dyn Fn(String) -> Option<String>>>,
) -> String {
    let prefix = "src".to_string();
    let path = format!("{prefix}/templates/{filename}");
    println!("{:?}", path);
    let content = fs::read_to_string(path).unwrap();
    match parser {
        Some(parser) => parser(content).unwrap_or(String::from("Unable to parse")),
        None => build_page(content, data),
    }
}

fn build_page(content: String, _data: Option<String>) -> String {
    content
}
