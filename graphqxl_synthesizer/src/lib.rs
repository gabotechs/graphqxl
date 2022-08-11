use crate::synth_spec::SpecSynth;
use crate::synths::{Synth, SynthContext};
use graphqxl_parser::Spec;

mod synth_arguments;
mod synth_block_def;
mod synth_block_field;
mod synth_description;
mod synth_directive_def;
mod synth_scalar;
mod synth_spec;
mod synth_union;
mod synth_value_type;
mod synths;
mod utils;
mod synth_directive;
mod synth_value_data;
mod synth_function_call;

pub struct SynthOptions {
    pub indent_spaces: usize,
    pub multiline: bool,
}

pub fn synth_spec(spec: Spec, options: SynthOptions) -> String {
    SpecSynth(spec).synth(&SynthContext {
        indent_spaces: options.indent_spaces,
        indent_lvl: 0,
        multiline: options.multiline,
    })
}
