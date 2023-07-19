#[allow(unused)]
#[allow(dead_code)]

mod parsing;

use std::env;

use parsing::Data;


fn main() {
    let args: Vec<String> = env::args().collect();

    let data: Data = match Data::build(args) {
        Ok(info) => match info.check_value() {
            Ok(()) => info,
            Err(err) => {
                println!("Error {err}");
                std::process::exit(1);
            }
        },
        Err(err) => {println!("Error {err}"); std::process::exit(1)},
    };

    println!("Data: {:?}", data);
}
