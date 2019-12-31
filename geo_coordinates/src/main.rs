use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u64,
    green: u64,
    blue: u64,
}

impl Display for Color {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {  
        write!(f, "RGB ({},{},{}) 0x{:02X}{:02X}{:02X}",
               self.red, self.green,self.blue,self.red, self.green,self.blue)
               
    }
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", *color);
    }

    // Variables can be type annotated.
    //let logical: bool = true;

    //let a_float: f64 = 1.0;  // Regular annotation
    //let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    //let default_float   = 3.0; // `f64`
    //let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    //let mut inferred_type = 12; // Type i64 is inferred from another line
    //inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    //let mut mutable = 12; // Mutable `i32`
    //mutable = 21;
    
    // Error! The type of a variable can't be changed.
    //mutable = true;
    
    // Variables can be overwritten with shadowing.
    //let mutable = true;
}

