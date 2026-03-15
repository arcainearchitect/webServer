use std::net::TcpListener;
use std::io::{Read, Write};

fn main(){
    let addr:&str="127.0.0.1:6000";
    let listener:TcpListener=TcpListener::bind(&addr)
        .expect("Failed to bind to address!");
    println!("Server listening on {}...", &addr);

    for stream in listener.incoming(){
        match stream {
            Ok(mut stream)=> {
                // Read request
                let mut buffer= [0u8; 1024];
                let bytes_read=stream.read(&mut buffer).unwrap_or(0);
                
                if bytes_read>0{
                    // Print received data
                    println!("Received request: \n{}", String::from_utf8_lossy(&buffer[..bytes_read]));

                    // Send HTTP response
                    let response="HTTP/1.1 200 OK\r\n\r\nContent-Length:13\r\n\r\nHello, World!";
                stream.write_all(response.as_bytes())
                    .expect("Failed to write response!");
                }
            }
            Err(e)=>eprintln!("Connection failed: {}", e),
        }
    }
}
