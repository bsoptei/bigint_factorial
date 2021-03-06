use std::env;

mod functions;
use functions::factorial::*;

fn main() {

    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(main_arg) => {
            match main_arg.parse::<u32>() {
                Ok(n) => {
                    println!("I'm calculating the factorial of {}", n);

                    let result = bigint_factorial(n).to_str_radix(10);
                    let shown_length = if &result.len() < &32 { result.len() } else { 32 };

                    println!(
                        "Done. Result:\n{}.{}E{}",
                        &result[..1],
                        &result[1..shown_length],
                        result.len() - 1
                    )
                },
                Err(_e) => println!("Could not parse {}", main_arg)
            }
        },
        None => println!("No argument, no result")
    }
}
