use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::fs;


fn handle_connection(mut stream: TcpStream){
/**************************************************************************************************************************************/
/*                                                       Reading the Request                                                          */
/**************************************************************************************************************************************/

        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        println!("Request: {http_request:#?}");

        let request_line = match http_request.first() {
        Some(line) => line.clone(),
        None => {
            eprintln!("No request line found");
            return;
        }
    };

    println!("Request Line: {}", request_line);
/**************************************************************************************************************************************/
/*                            Writing a Response and validating the request                                                           */
/**************************************************************************************************************************************/
        // following code will send a blank page
            // let response = "HTTP/1.1 200 OK\r\n\r\n";
            // stream.write_all(response.as_bytes()).unwrap();

        let (status_line, filename) = 
            if request_line == "GET / HTTP/1.1" {
                ("HTTP/1.1 200 OK", "index.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND", "404.html")
            };
        
        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
       
}
fn main() {


    /**************************************************************************************************************************************/
    /*        Listening to the TCP Connection(This code will listen at the local address 127.0.0.1:7878 for incoming TCP streams)         */
    /**************************************************************************************************************************************/

   let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //bind will return a new TcpListener instance

   for stream in listener.incoming(){ //incoming method returns an iterator that gives us a sequence of streams
        let stream = stream.unwrap();
        handle_connection(stream);
   }


}
