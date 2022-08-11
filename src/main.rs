use clap::Parser;
use graphqxl_parser::parse_graphqxl;
use graphqxl_synthesizer::{synth_spec, SynthOptions};
use std::fs;
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap()]
    input: String,

    #[clap(long)]
    indent_spaces: Option<usize>,

    #[clap(short, long, action)]
    multiline: bool,
}

fn main() {
    let args = Args::parse();
    let out_path = if args.input.ends_with("graphql") {
        args.input[..args.input.len() - 1].to_string() + "xl"
    } else {
        args.input.to_string() + ".graphqxl"
    };
    let content_or_err = fs::read_to_string(&args.input);
    if let Ok(content) = content_or_err {
        let spec_or_err = parse_graphqxl(&content);
        if let Ok(spec) = spec_or_err {
            let result = synth_spec(
                spec,
                SynthOptions {
                    indent_spaces: args.indent_spaces.unwrap_or(2),
                    multiline: args.multiline,
                },
            );
            if let Err(err) = fs::write(&out_path, result) {
                eprintln!("{}", err);
                exit(1)
            } else {
                println!("Generated file {}", out_path);
            }
        } else {
            eprintln!("{}", spec_or_err.unwrap_err());
            exit(1);
        }
    } else {
        eprintln!("{}", content_or_err.unwrap_err());
        exit(1);
    }
}
