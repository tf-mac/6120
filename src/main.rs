use std::{io::{self, Read}};

use bpaf::Parser;
use bpaf::{construct, positional, OptionParser};
use hw6120::{cfg::{build_blocks, build_cfg, map_blocks}, dfa::reaching};
use hw6120::program::Program;

pub fn options() -> OptionParser<String> {
    let mode = positional("MODE")
        .help("Mode for the program to run");
    construct!(mode)
    .to_options()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let program: Program = serde_json::from_str(&input)?;
    for i in program.functions {
        let basic_blocks = build_blocks(i);
        match options().run().as_str() {
            "bb" => print!("{:#?}\n",basic_blocks),
            "bb_map" => print!("{:#?}\n", map_blocks(basic_blocks)),
            "cfg" => {
                let (a,b,c) = map_blocks(basic_blocks);
                print!("{:#?}\n", build_cfg(a,b,c))
            },
            "dfa" => {
                let (a,b,c) = map_blocks(basic_blocks);
                let cfg = build_cfg(a,b,c);
                print!("{:#?}\n", reaching(cfg, a, "blah".to_string(), i.args.iter().cloned()))
            }
            j => print!("Mode {} not implemented\n", j)
        }
    };
    Ok(())
}
