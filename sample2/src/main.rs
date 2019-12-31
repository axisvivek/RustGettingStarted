use std::net::{TcpListener, TcpStream};
use std::io;
use std::fs;

fn handle_client(_stream: TcpStream) {
    // ...
   createfile();
}

fn createfile() {
    let data = "Some data!";
    fs::write("/usr/local/packages/axis_acapwith_rust/vivek.txt", data).expect("Unable to write file");

}

fn main() -> io::Result<()> {
    createfile();
    let listener = TcpListener::bind("192.168.1.115:8089")?;

    // accept connections and process them serially
    for _stream in listener.incoming() {
        handle_client(_stream?);
    }
    Ok(())
} 


