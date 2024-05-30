use super::app;
use super::http::handle_connection;
use super::webby::{HttpMethod, RequestProccessor};
use core::time;
use std::net::TcpStream;
use std::sync::mpsc::{self, Sender};
use std::thread;

pub struct ThreadPool {
    transmitters: Vec<Sender<TcpStream>>,
    next_thread: usize,
    num_threads: usize,
}

fn setup() -> RequestProccessor {
    let mut rust_server = RequestProccessor::new();
    rust_server.serve_assets(None);
    rust_server.register(HttpMethod::Get, "/", Box::new(app::home));
    rust_server.register(HttpMethod::Get, "/rahul", Box::new(app::rahul));
    rust_server
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> Self {
        let mut transmitters: Vec<Sender<TcpStream>> = Vec::new();
        for thread_num in 1..=num_threads {
            let (tx, rx) = mpsc::channel::<TcpStream>();
            thread::spawn(move || {
                let rust_server = setup();
                println!("Setup completed in thread : {thread_num}");
                loop {
                    let stream = rx.recv().unwrap();
                    // thread::sleep(time::Duration::from_secs(5));
                    handle_connection(&rust_server, stream);
                    println!("responded from thread : {thread_num}")
                }
            });
            transmitters.push(tx);
        }
        ThreadPool {
            next_thread: 0,
            num_threads,
            transmitters,
        }
    }
    pub fn process(&mut self, stream: TcpStream) {
        let mut idx = self.next_thread;
        let num_threads = self.num_threads;
        self.transmitters[idx].send(stream).unwrap();
        idx = (idx + 1) % num_threads;
        self.next_thread = idx;
    }
}
