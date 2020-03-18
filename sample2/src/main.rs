extern crate chrono;
use chrono::{DateTime, Utc,Local, TimeZone};
use std::fs::File;
use std::io::{Write};
//use curl::easy::{Easy,Auth};
use std::process::Command;
use std::fmt; 

struct Output(Vec<String>);

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn write_to_file(data_in_string: String) -> std::io::Result<()> {
    let data = data_in_string.as_bytes();
    let now: DateTime<Utc> = Utc::now();
    let filename= Local.from_utc_datetime(&now.naive_local()).format("%Y_%b_%d_%H_%M_%S%.3f.jpg").to_string();
    let mut pos = 0;
    let mut buffer = File::create(filename)?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}


// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Welcome to Axis
    println!("Hello Axis!");
 /*
======================================================
ACAP Development using RUST : Vivek Kumar (31-12-2019)
======================================================

 */
    println!("You can build ACAP Application using Rust :)");
    
  
{
    /*
    let data = "Vivek Kumar, Axis Communications";
     //write_to_file(data.to_string()).unwrap();
     // Write the contents of rust-lang.org to stdout
     let mut easy = Easy::new();
     easy.url("http://192.168.1.193/axis-cgi/param.cgi?action=list&responseformat=rfc").unwrap();
     easy.get(true).unwrap();
     let mut auth=Auth::new();
     auth.digest(true);
     easy.http_auth(&auth).unwrap(); 
     easy.username("root").unwrap();
     easy.password("pass").unwrap();     
     //easy.perform().unwrap();

 let mut data = Vec::new();
 {
    // Create transfer in separate scope ...
    let mut transfer = easy.transfer();
    // Response body
    transfer.write_function(|new_data| {
        data.extend_from_slice(new_data);
        Ok(new_data.len())
    }).unwrap();

    transfer.perform().unwrap();
    // .. to force drop it here, so we can use easy.response_code()
 } 

println!("{}", easy.response_code().unwrap());
println!("Received {} bytes", data.len());
if !data.is_empty() {
    println!("Bytes: {:?}", data);
    println!("As string: {}", String::from_utf8_lossy(&data));
}


*/
}
// curl -u root:pass --digest --noproxy "*"  "http://192.168.1.193/axis-cgi/param.cgi?action=list&responseformat=rfc"
let mut curldigest = Command::new("curl");
let now: DateTime<Utc> = Utc::now();
let filename= Local.from_utc_datetime(&now.naive_local()).format("%Y_%b_%d_%H_%M_%S%.3f.jpg").to_string();
curldigest.arg("-u")
          .arg("root:pass")
          .arg("--digest")
          .arg("--noproxy")
          .arg("--output")
          .arg("/mnt/d/vivek.jpg")
          .arg("*")
          .arg("http://192.168.1.193/axis-cgi/jpg/image.cgi?&text=1&textstring=My%20Camera&camera=2");

//let digest1 = 
curldigest.output().expect("curl command failed to execute");
//let sparkle_string = String::from_utf8(digest1.stdout).unwrap();
//let sparkle_string = String::from_utf8(digest1.stdout).unwrap();
//write_to_file(sparkle_string).unwrap();
//println!("{}",digest1);

}


