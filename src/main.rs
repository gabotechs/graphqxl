use anyhow::{anyhow, Context, Result};
use clap::Parser;
use graphqxl_parser::parse_spec;
use graphqxl_synthesizer::{synth_spec, SynthConfig};
use graphqxl_transpiler::transpile_spec;
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap()]
    input: String,

    #[clap(long)]
    indent_spaces: Option<usize>,

    #[clap(long)]
    private_prefix: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let out_path = if args.input.ends_with("graphqxl") {
        args.input[..args.input.len() - 2].to_string() + "l"
    } else {
        args.input.to_string() + ".graphql"
    };
    let spec_result = parse_spec(&args.input);
    let spec = if let Ok(spec) = spec_result {
        spec
    } else {
        return Err(anyhow!(
            "Could not parse graphqxl spec:\n\n{}",
            spec_result.unwrap_err()
        ));
    };
    let transpiled = transpile_spec(&spec).context("Error transpiling graphqxl file")?;

    let result = synth_spec(
        transpiled,
        SynthConfig {
            indent_spaces: args.indent_spaces.unwrap_or(2),
            private_prefix: args.private_prefix.unwrap_or_else(|| "_".to_string()),
            ..Default::default()
        },
    );
    fs::write(&out_path, result)?;
    Ok(())
}
