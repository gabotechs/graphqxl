use graphqxl_parser::{DefType, Spec};

use crate::synth_block_def::BlockDefSynth;
use crate::synth_directive_def::DirectiveDefSynth;
use crate::synth_scalar::ScalarSynth;
use crate::synth_schema::SchemaSynth;
use crate::synth_union::UnionSynth;
use crate::synths::{Synth, SynthContext};

pub(crate) struct SpecSynth(pub(crate) Spec);

impl Synth for SpecSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let mut result = "".to_string();
        for def_name in &self.0.order {
            match def_name {
                DefType::Type(name) => {
                    let def = self.0.types.get(&name.id).unwrap().to_owned();
                    if def.generic.is_none() {
                        result += &BlockDefSynth(def).synth(context);
                        result += "\n\n";
                    }
                }
                DefType::Input(name) => {
                    let def = self.0.inputs.get(&name.id).unwrap().to_owned();
                    if def.generic.is_none() {
                        result += &BlockDefSynth(def).synth(context);
                        result += "\n\n";
                    }
                }
                DefType::Enum(name) => {
                    let def = self.0.enums.get(&name.id).unwrap().to_owned();
                    result += &BlockDefSynth(def).synth(context);
                    result += "\n\n";
                }
                DefType::Interface(name) => {
                    let def = self.0.interfaces.get(&name.id).unwrap().to_owned();
                    result += &BlockDefSynth(def).synth(context);
                    result += "\n\n";
                }
                DefType::Union(name) => {
                    let def = self.0.unions.get(&name.id).unwrap().to_owned();
                    result += &UnionSynth(def).synth(context);
                    result += "\n\n";
                }
                DefType::Scalar(name) => {
                    let def = self.0.scalars.get(&name.id).unwrap().to_owned();
                    result += &ScalarSynth(def).synth(context);
                    result += "\n\n";
                }
                DefType::Directive(name) => {
                    let def = self.0.directives.get(&name.id).unwrap().to_owned();
                    result += &DirectiveDefSynth(def).synth(context);
                    result += "\n\n";
                }
                _ => {
                    // nothing to synth
                }
            }
        }
        result += &SchemaSynth(self.0.schema.clone()).synth(context);
        result += "\n";
        result
    }
}
