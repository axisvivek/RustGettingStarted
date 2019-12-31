
#![feature(type_ascription)]
//#[allow(dead_code)]
//struct Structure(i32);

extern crate chrono;
use chrono::{DateTime, Utc,Local, TimeZone};

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

    print!("31-");
    print!("12-");
    print!("2019");
    println!();
    //println!("{}",now.to_string());

    let now: DateTime<Utc> = Utc::now();
    let loc = chrono::Local::now();

    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));

    
    println!("{:?}", loc);

    println!("{:?}", now.with_timezone(&Local));
    println!("{:?}", Local.from_utc_datetime(&now.naive_local()));
    println!("Local Time now in a custom format is: {}", 
         Local.from_utc_datetime(&now.naive_local()).format("%a %b %e %T %Y"));

    eprintln!("Hi! This is a error.");
    println!("{} days", 31); //i32 default
    println!("{} days", 31.3: f64);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");


     // Special formatting can be specified after a `:`.
    println!("{:b} of {:b} people know binary, the other half doesn't", 10, 2);

    println!("{number:>width$}", number=1, width=9);

     println!("My name is {0}, {0} {1}", "James", "Bond");

}


