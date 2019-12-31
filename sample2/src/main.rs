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



}


