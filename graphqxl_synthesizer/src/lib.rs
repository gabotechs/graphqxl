use crate::synth_spec::SpecSynth;
pub use crate::synths::SynthConfig;
use crate::synths::{Synth, SynthContext};
use graphqxl_parser::Spec;

mod synth_arguments;
mod synth_block_def;
mod synth_block_field;
mod synth_description;
mod synth_directive;
mod synth_directive_def;
mod synth_function_call;
mod synth_scalar;
mod synth_schema;
mod synth_spec;
mod synth_union;
mod synth_value_data;
mod synth_value_type;
mod synths;
mod utils;

pub fn synth_spec(spec: Spec, options: SynthConfig) -> String {
    SpecSynth(options, spec).synth(&SynthContext { indent_lvl: 0 })
}
