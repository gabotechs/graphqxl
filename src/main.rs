use clap::Parser;
use graphqxl_parser::parse_spec;
use graphqxl_synthesizer::{synth_spec, SynthConfig};
use graphqxl_transpiler::transpile_spec;
use std::error::Error;
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap()]
    input: String,

    #[clap(long)]
    indent_spaces: Option<usize>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let out_path = if args.input.ends_with("graphqxl") {
        args.input[..args.input.len() - 2].to_string() + "l"
    } else {
        args.input.to_string() + ".graphql"
    };
    let spec = parse_spec(&args.input)?;
    let transpiled = transpile_spec(&spec)?;

    let result = synth_spec(transpiled, SynthConfig::default());
    fs::write(&out_path, result)?;
    Ok(())
}
