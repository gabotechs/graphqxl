mod apollo_diagnostic_source;

use crate::apollo_diagnostic_source::reverse_diagnostic_map;
use anyhow::{anyhow, Context, Result};
use apollo_compiler::ApolloCompiler;
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
    output: Option<String>,

    #[clap(long)]
    indent_spaces: Option<usize>,

    #[clap(long)]
    private_prefix: Option<String>,
}

fn graphqxl_to_graphql(args: &Args) -> Result<(String, String)> {
    let out_path = if let Some(out_path) = &args.output {
        out_path.to_string()
    } else if args.input.ends_with("graphqxl") {
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

    let (result, source_map) = synth_spec(
        transpiled,
        SynthConfig {
            indent_spaces: args.indent_spaces.unwrap_or(2),
            private_prefix: args
                .private_prefix
                .clone()
                .unwrap_or_else(|| "_".to_string()),
            ..Default::default()
        },
    );
    let ctx = ApolloCompiler::new(&result);
    let diagnostics = ctx.validate();
    for diagnostic in diagnostics {
        if diagnostic.is_error() {
            reverse_diagnostic_map(&diagnostic, &source_map)?;
        }
    }
    Ok((result, out_path))
}

fn main() -> Result<()> {
    let args = Args::parse();
    let (result, out_path) = graphqxl_to_graphql(&args)?;
    fs::write(&out_path, result)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::path::Path;

    #[test]
    fn test_graphqxl_to_graphql() {
        let test_dir = Path::new("src").join("test");
        let paths = fs::read_dir(&test_dir).unwrap();
        for dir_entry in paths {
            let file_name = dir_entry.unwrap().file_name();
            let path = file_name.to_str().unwrap();
            if path.starts_with('_') || path.ends_with("result") {
                continue;
            }
            let result = graphqxl_to_graphql(&Args {
                input: test_dir.join(path).to_str().unwrap().to_string(),
                output: None,
                indent_spaces: None,
                private_prefix: None,
            });
            let result = if let Ok((result, _)) = result {
                result
            } else {
                let err = format!("{}", result.unwrap_err());
                let re = Regex::new(r"(/.+)+\.graphqxl").unwrap();
                re.replace_all(&err, "").to_string()
            };
            let out_path = test_dir.join(path.to_string() + ".result");
            if out_path.exists() {
                let expected = fs::read_to_string(out_path).unwrap();
                assert_eq!(expected, result)
            } else {
                fs::write(out_path, result).unwrap();
            }
        }
    }
}
