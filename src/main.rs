mod apollo_diagnostic_source;
mod ok_or_anyhow_err;

use crate::apollo_diagnostic_source::reverse_diagnostic_map;
use crate::ok_or_anyhow_err::ok_or_anyhow_err;
use anyhow::Result;
use apollo_compiler::ApolloCompiler;
use clap::Parser;
use graphqxl_parser::parse_spec;
use graphqxl_synthesizer::{synth_spec, SynthConfig};
use graphqxl_transpiler::{transpile_spec, TranspileSpecOptions};
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Path to the .graphqxl file")]
    input: String,

    #[arg(short, long, help = "Output path for the generated .graphql file")]
    output: Option<String>,

    #[arg(
        long,
        default_value_t = 2,
        help = "Number of spaces used for the generated file's indentation"
    )]
    indent_spaces: usize,

    #[arg(
        long,
        default_value_t = String::from("_"),
        help = "String that needs to be prefixed to a type or an input in order to consider it private"
    )]
    private_prefix: String,
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
    let spec = ok_or_anyhow_err(spec_result, "Could not parse GraphQXL spec")?;

    let transpile_result = transpile_spec(
        &spec,
        &TranspileSpecOptions {
            private_prefix: args.private_prefix.clone(),
        },
    );
    let transpiled = ok_or_anyhow_err(transpile_result, "Could not transpile graphqxl spec")?;

    let (result, source_map) = synth_spec(
        transpiled,
        SynthConfig {
            indent_spaces: args.indent_spaces,
            private_prefix: args.private_prefix.clone(),
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

    const ONLY: &str = "";

    #[test]
    fn test_graphqxl_to_graphql() {
        let test_dir = Path::new("src").join("test");
        let paths = fs::read_dir(&test_dir).unwrap();
        for dir_entry in paths {
            let file_name = dir_entry.unwrap().file_name();
            let path = file_name.to_str().unwrap();
            if path.starts_with('_')
                || path.ends_with("result")
                || (!ONLY.is_empty() && !path.contains(ONLY))
            {
                continue;
            }
            let result = graphqxl_to_graphql(&Args {
                input: test_dir.join(path).to_str().unwrap().to_string(),
                output: None,
                indent_spaces: 2,
                private_prefix: "_".to_string(),
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
                assert_eq!(result, expected)
            } else {
                fs::write(out_path, result).unwrap();
            }
        }
    }
}
