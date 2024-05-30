use std::net::TcpListener;
mod app;
mod http;
mod template;
mod threadpool;
mod webby;
use webby::HttpMethod;

use self::threadpool::ThreadPool;

fn main() {
    let server_addr = "127.0.0.1:2828";
    let listner = TcpListener::bind(server_addr).unwrap();
    let mut pool = ThreadPool::new(10);

    for stream in listner.incoming() {
        // thread::spawn(|| {
        //     match stream {
        //         Ok(stream) => handle_connection(&rust_server, stream),
        //         Err(err) => {
        //             println!("{:?}", err);
        //         }
        //     };
        // });
        let stream = stream.unwrap();
        pool.process(stream);
    }
}
