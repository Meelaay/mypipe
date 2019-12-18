use std::process::Command;
extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("mypipe fortune-cowsay linux")
                          .arg(Arg::with_name("input")
                            .long("input")
                            .takes_value(true)
                            .required(true))
                          .arg(Arg::with_name("output")
                            .long("output")
                            .takes_value(true)
                            .required(true))
                    .get_matches();

    let cow = Command::new(matches.value_of("output").unwrap())
        .arg(String::from_utf8_lossy(&(Command::new(matches.value_of("input").unwrap())
                                        .output()
                                        .expect("failed to execute process")).stdout).to_string())
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&cow.stdout));
    
}