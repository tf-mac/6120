use std::{env, io::{self, Read}, process::exit};

use l3::{dce::dce, lvn::lvn, program::Program};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let program: Program = serde_json::from_str(&input)?;

    let cli_args: Vec<String> = env::args().collect();

    let transformed = match cli_args.get(1) {
        Some(p) if p == "dce" => dce(program),
        Some(p) if p == "lvn" => lvn(program),
        _ => {
            eprintln!("Please input a sub-program");
            exit(1);
        }
    };

    println!("{}", serde_json::to_string(&transformed)?);

    Ok(())
}
