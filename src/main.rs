use clap::Parser;
use graphqxl_parser::parse_graphqxl;
use graphqxl_synthesizer::{synth_spec, SynthOptions};
use std::fs;
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'i', long)]
    input: String,

    #[clap(long)]
    indent_spaces: Option<usize>,

    #[clap(long, takes_value = false)]
    multiline: Option<bool>,
}

fn main() {
    let args = Args::parse();
    let out_path = if args.input.ends_with("graphqxl") {
        args.input[..args.input.len() - 2].to_string() + "l"
    } else {
        args.input.to_string() + ".graphql"
    };
    let content_or_err = fs::read_to_string(&args.input);
    if let Ok(content) = content_or_err {
        let spec_or_err = parse_graphqxl(&content);
        if let Ok(spec) = spec_or_err {
            let result = synth_spec(
                spec,
                SynthOptions {
                    indent_spaces: args.indent_spaces.unwrap_or(2),
                    multiline: args.multiline.unwrap_or(false),
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
