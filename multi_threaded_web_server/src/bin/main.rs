use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::fs;
use std::{thread, time::Duration};
use multi_threaded_web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream){

        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        // println!("Request: {http_request:#?}");

        let request_line = match http_request.first() {
        Some(line) => line.clone(),
        None => {
            eprintln!("No request line found");
            return;
        }
    };

    // println!("Request Line: {}", request_line);

    let (status_line, filename) = 
        if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "index.html")
        } else if request_line == "GET /sleep HTTP/1.1"{
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
    
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
       
}
fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool= ThreadPool::new(5);

    for stream in listener.incoming().take(2){ 
            let stream = stream.unwrap();
            pool.execute(||  {
                 handle_connection(stream);
            })
    }
}
