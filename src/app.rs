use super::template::render_page;

pub fn home(_request: &str) -> String {
    render_page("home.html", None, None)
}
pub fn rahul(_request: &str) -> String {
    String::from("<h1> Rahul </h1>")
}
