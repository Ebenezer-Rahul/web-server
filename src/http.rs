use super::webby::{self, HttpMethod};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn handle_connection(rust_server: &webby::RequestProccessor, mut stream: TcpStream) {
    println!("{:?}", stream);
    let buffer = BufReader::new(&mut stream);
    let http_request: Vec<_> = buffer
        .lines()
        .map(|result_line| result_line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);
    let request_line = &http_request[0];
    let request_line: Vec<_> = request_line.split(' ').collect();
    println!("{:#?}", request_line);
    let _method = &request_line[0];
    let route = &request_line[1];
    let mut status_code = 200;
    let content = rust_server
        .execute(HttpMethod::Get, route)
        .unwrap_or_else(|_| -> String {
            status_code = 404;
            String::from("<h1> Page Not Found <h1>")
        });
    println!("{content}");
    let response = build_response(content, status_code);
    if let Err(err) = stream.write_all(response.as_bytes()) {
        panic!("err : {:?}", err);
    };
}

fn build_response(content: String, status_code: i32) -> String {
    let mut reason_phrase;
    match status_code {
        200 => reason_phrase = "OK",
        _ => reason_phrase = "BAD REQUEST",
    };
    let headers = build_headers(status_code);
    let response = format!("HTTP/1.1 {status_code} {reason_phrase}\r\n{headers}\r\n{content}");
    response
}

fn build_headers(_status_code: i32) -> String {
    String::from("")
}
