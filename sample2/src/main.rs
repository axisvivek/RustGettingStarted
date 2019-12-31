use std::io;
use std::fs;


fn createfile(message: String) {
    let data = message;
    fs::write("/usr/local/packages/axis_acapwith_rust/vivek.txt", data).expect("Unable to write file");

}

fn main() -> io::Result<()> {
    createfile("Hi Vivek\n".to_string());
    Ok(())
} 


