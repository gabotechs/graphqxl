use graphqxl_parser::{DefType, Spec};

use crate::synth_block_def::BlockDefSynth;
use crate::synth_directive_def::DirectiveDefSynth;
use crate::synth_scalar::ScalarSynth;
use crate::synth_schema::SchemaSynth;
use crate::synth_union::UnionSynth;
use crate::synths::{Synth, SynthContext};

pub(crate) struct SpecSynth(pub(crate) Spec);

impl Synth for SpecSynth {
    fn synth(&self, context: &mut SynthContext) -> bool {
        for def_name in &self.0.order {
            match def_name {
                DefType::Type(name) => {
                    let def = self.0.types.get(&name.id).unwrap().to_owned();
                    if def.generic.is_none() {
                        let has_written = BlockDefSynth(def).synth(context);
                        if has_written {
                            context.write_double_line_jump();
                        }
                    }
                }
                DefType::Input(name) => {
                    let def = self.0.inputs.get(&name.id).unwrap().to_owned();
                    if def.generic.is_none() {
                        let has_written = BlockDefSynth(def).synth(context);
                        if has_written {
                            context.write_double_line_jump();
                        }
                    }
                }
                DefType::Enum(name) => {
                    let def = self.0.enums.get(&name.id).unwrap().to_owned();
                    BlockDefSynth(def).synth(context);
                    context.write_double_line_jump();
                }
                DefType::Interface(name) => {
                    let def = self.0.interfaces.get(&name.id).unwrap().to_owned();
                    BlockDefSynth(def).synth(context);
                    context.write_double_line_jump();
                }
                DefType::Union(name) => {
                    let def = self.0.unions.get(&name.id).unwrap().to_owned();
                    UnionSynth(def).synth(context);
                    context.write_double_line_jump();
                }
                DefType::Scalar(name) => {
                    let def = self.0.scalars.get(&name.id).unwrap().to_owned();
                    ScalarSynth(def).synth(context);
                    context.write_double_line_jump();
                }
                DefType::Directive(name) => {
                    let def = self.0.directives.get(&name.id).unwrap().to_owned();
                    DirectiveDefSynth(def).synth(context);
                    context.write_double_line_jump();
                }
                DefType::Schema(name) => {
                    let def = self.0.schemas.get(name).unwrap().to_owned();
                    SchemaSynth(def).synth(context);
                    context.write_double_line_jump();
                }
                _ => {
                    // nothing to synth
                }
            }
        }
        true
    }
}
