use std::env;

mod functions;
use functions::factorial::*;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let main_arg = &args[1];

        match main_arg.parse::<u32>() {
            Ok(n) => {
                println!("I'm calculating the factorial of {}", n);
                let result = bigint_factorial(n);
                let shown_length = if &result.len() < &32 { result.len() } else { 32 };
                println!("Done. Result:\n{}.{}E{}",  &result[..1], &result[1..shown_length], result.len() - 1);
            }
            Err(_e) => println!("{} is not an integer >= 0.", main_arg)
        }
    } else {
        println!("No argument, no result.");
    }

}
