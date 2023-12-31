use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::TcpListener;
use std::io::Read;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Server{
        Server {
            addr: addr
        }
    }
    pub fn run(self){
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop{
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(size) =>{
                            println!("read conn: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(Request) =>{},
                                Err(e) => println!("Failed to parse error {}", e)
                            }
                        },
                        Err(e) => println!("Error reading: {}", e)

                    }
                    
                },
                Err(e) =>{
                    println!("Failed to establish a conn: {}", e);
                    continue 
                }
            }
        }
        

    }
}
